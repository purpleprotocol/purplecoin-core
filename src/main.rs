// Copyright (c) 2022 Octavian Oncescu
// Copyright (c) 2022-2023 The Purplecoin Core developers
// Licensed under the Apache License, Version 2.0 see LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0 or the MIT license, see
// LICENSE-MIT or http://opensource.org/licenses/MIT

#![feature(stmt_expr_attributes)]

use futures::prelude::*;
use iced::window::icon::Icon;
use iced::{Application, Settings};
use log::*;
use mimalloc::MiMalloc;
use purplecoin::chain::backend::disk::DiskBackend;
use purplecoin::chain::backend::ShardBackend;
use purplecoin::chain::*;

use purplecoin::node::*;

use purplecoin::primitives::*;

use purplecoin::settings::SETTINGS;

use rand::prelude::*;
use rayon::prelude::*;

use std::env;

use std::sync::atomic::AtomicBool;

use std::thread;
use std::time::Duration;
use tarpc::server::{self, incoming::Incoming, Channel};
use tokio::runtime::Builder;
use tokio::time::sleep;
use tracing_subscriber::prelude::*;

use warp::Filter;

#[cfg(feature = "sha256sum")]
use sha2::{Digest, Sha256};

#[cfg(feature = "blake3sum")]
use blake3::Hasher as Blake3;

#[cfg(feature = "gui")]
use purplecoin::gui::GUI;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
static EXIT_SIGNAL: AtomicBool = AtomicBool::new(true);

fn main() -> anyhow::Result<()> {
    purplecoin::global::init(4, SETTINGS.node.randomx_fast_mode);

    #[cfg(feature = "gui")]
    thread::spawn(start_runtime);

    #[cfg(not(feature = "gui"))]
    start_runtime()?;

    #[cfg(feature = "gui")]
    start_gui()?;

    Ok(())
}

fn start_runtime() -> anyhow::Result<()> {
    let db = purplecoin::chain::backend::create_rocksdb_backend();
    let config = ChainConfig::new(&SETTINGS.node.network_name);
    let disk_backend =
        DiskBackend::new(db, std::sync::Arc::new(config.clone()), None, None).unwrap();
    let _chain = Chain::new(disk_backend, &config);
    let worker_threads = if SETTINGS.node.network_threads == 0 {
        num_cpus::get()
    } else {
        SETTINGS.node.network_threads as usize
    };

    let verifier_threads = if SETTINGS.node.verifier_threads == 0 {
        num_cpus::get()
    } else {
        SETTINGS.node.verifier_threads as usize
    };

    // Also set rayon thread count according to verifier_threads value
    rayon::ThreadPoolBuilder::new()
        .num_threads(verifier_threads)
        .build_global()
        .unwrap();

    let runtime = Builder::new_multi_thread()
        .max_blocking_threads(verifier_threads)
        .worker_threads(worker_threads)
        .enable_io()
        .enable_time()
        .build()
        .unwrap();

    runtime.block_on(async {
        init_tracing("PurplecoinCore").unwrap();
        perform_sanity_checks();

        if SETTINGS.node.memory_only {
            info!(
                "Running Purplecoin Core v{} on {} in memory only mode",
                env!("CARGO_PKG_VERSION"),
                SETTINGS.node.network_name
            );
        } else {
            info!(
                "Running Purplecoin Core v{} on {}",
                env!("CARGO_PKG_VERSION"),
                SETTINGS.node.network_name
            );
        }

        tokio::try_join!(
            #[cfg(feature = "rpc")]
            run_rpc(),
            run_periodics(),
        )?;

        Ok(())
    })
}

#[cfg(feature = "gui")]
fn start_gui() -> iced::Result {
    let mut gui_settings = Settings::default();

    gui_settings.id = Some("org.purplecoin.PurplecoinCore".to_owned());

    // Set application icon
    {
        let icon_with_format = image::io::Reader::with_format(
            std::io::Cursor::new(include_bytes!("./gui/img/logo_purple_square.png")),
            image::ImageFormat::Png,
        );
        let pixels = icon_with_format.decode().unwrap().to_rgba8();
        gui_settings.window.icon =
            Some(Icon::from_rgba(pixels.to_vec(), pixels.width(), pixels.height()).unwrap());
    }

    // Don't close application implicitly when clicking the close window button
    gui_settings.exit_on_close_request = false;

    info!(
        "Starting Purplecoin Core v{} GUI",
        env!("CARGO_PKG_VERSION")
    );
    GUI::run(gui_settings)
}

#[cfg(feature = "rpc")]
async fn run_rpc() -> anyhow::Result<()> {
    if SETTINGS.network.rpc_enabled {
        // Create transports
        let (client_transport, server_transport) = tarpc::transport::channel::unbounded();
        let server = server::BaseChannel::with_defaults(server_transport);
        let client =
            RpcServerDefinitionClient::new(tarpc::client::Config::default(), client_transport)
                .spawn();

        // Schedule rpc server
        tokio::spawn(server.execute(RpcServer.serve()));

        // Set up http route
        let client_filter = warp::any().map(move || client.clone());
        let rpc_path = warp::post()
            .and(warp::path::end())
            .and(json_body())
            .and(client_filter.clone())
            .and(warp::header("authorization"))
            .and_then(handle_rpc_request);

        let port = match SETTINGS.node.network_name.as_str() {
            "mainnet" => SETTINGS.network.rpc_listen_port_mainnet,
            "testnet" => SETTINGS.network.rpc_listen_port_testnet,
            other => panic!("Invalid network name: {other}"),
        };

        info!(
            "Purplecoin Core v{} RPC Listening on port {}",
            env!("CARGO_PKG_VERSION"),
            port
        );

        warp::serve(rpc_path).run(([127, 0, 0, 1], port)).await;
    }

    Ok(())
}

/// Schedules periodic jobs such as chain pruning
async fn run_periodics() -> anyhow::Result<()> {
    loop {
        sleep(Duration::from_millis(1)).await;
    }

    Ok(())
}

async fn handle_rpc_request(
    request: tarpc::Request<RpcServerDefinitionRequest>,
    client: RpcServerDefinitionClient,
    authorization: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    if !check_authorization_header(authorization) {
        return Ok(warp::reply::with_status(
            warp::reply::json(&"Forbidden".to_owned()),
            warp::http::status::StatusCode::FORBIDDEN,
        ));
    }

    match dispatch_rpc_request(request, client).await {
        Ok(resp) => Ok(warp::reply::with_status(
            warp::reply::json(&resp),
            warp::http::StatusCode::CREATED,
        )),

        Err(err) => Ok(warp::reply::with_status(
            warp::reply::json(&err),
            warp::http::StatusCode::BAD_REQUEST,
        )),
    }
}

fn check_authorization_header(auth: String) -> bool {
    let split: Vec<_> = auth.split(' ').collect();

    if split.len() != 2 {
        return false;
    }

    if split[0] != "Basic" {
        return false;
    }

    let decoded = match base64::decode(split[1]) {
        Ok(decoded) => decoded,
        Err(_) => return false,
    };

    // Hash both stored credentials and given ones and then constant compare the two hashes
    let hash_key = "purplecoin.basic_auth";
    let oracle_key = format!(
        "{}:{}",
        SETTINGS.network.rpc_username, SETTINGS.network.rpc_password
    );
    let oracle_hash = Hash256::hash_from_slice(oracle_key.as_bytes(), hash_key);
    let hash = Hash256::hash_from_slice(decoded, hash_key);

    constant_time_eq::constant_time_eq_32(&oracle_hash.0, &hash.0)
}

/// Initializes an OpenTelemetry tracing subscriber with a Jaeger backend.
fn init_tracing(service_name: &str) -> anyhow::Result<()> {
    env::set_var("OTEL_BSP_MAX_EXPORT_BATCH_SIZE", "12");
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name(service_name)
        .with_max_packet_size(2usize.pow(13))
        .install_batch(opentelemetry::runtime::Tokio)?;

    tracing_subscriber::registry()
        .with(tracing_subscriber::filter::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .try_init()?;

    Ok(())
}

fn json_body(
) -> impl Filter<Extract = (tarpc::Request<RpcServerDefinitionRequest>,), Error = warp::Rejection> + Clone
{
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 64)
        .and(warp::body::json::<tarpc::Request<RpcServerDefinitionRequest>>())
}

fn perform_sanity_checks() {
    // Verify addresses checksums
    let addresses_mainnet_filename = pub_addresses_file_mainnet();
    let addresses_testnet_filename = pub_addresses_file_testnet();
    verify_addresses_checksum(addresses_mainnet_filename, ADDRESSES_RAW_MAINNET);
    verify_addresses_checksum(addresses_testnet_filename, ADDRESSES_RAW_TESTNET);

    // Add here more sanity checks
}

fn verify_addresses_checksum(addresses_path: &str, addresses_raw: &str) {
    let mut addresses_path_split: Vec<_> = addresses_path.split('.').collect();
    if !addresses_path.contains("genesisbalances") {
        panic!("Invalid addresses path");
    }

    if addresses_path_split.len() < 4 {
        panic!("invalid addresses path!");
    }

    #[cfg(feature = "blake3sum")]
    let sumtype = "blake3";

    #[cfg(feature = "sha256sum")]
    let sumtype = "sha256";

    addresses_path_split.pop().unwrap();
    let oracle_checksum = addresses_path_split
        .iter()
        .find(|nibble| nibble.contains(sumtype))
        .ok_or("addresses file name doesn't contain a checksum")
        .unwrap()
        .to_owned()
        .split('_')
        .collect::<Vec<_>>()[1];

    info!("Verifying addresses file checksum...");
    {
        #[cfg(feature = "blake3sum")]
        let mut hasher = Blake3::new();

        #[cfg(feature = "sha256sum")]
        let mut hasher = Sha256::new();

        hasher.update(addresses_raw.as_bytes());
        let result = hasher.finalize();

        #[cfg(feature = "blake3sum")]
        let checksum = hex::encode(result.as_bytes());

        #[cfg(feature = "sha256sum")]
        let checksum = hex::encode(result);

        if checksum != oracle_checksum {
            panic!(
                "Addresses file checksum verification failed! Got: {checksum}, Expected: {oracle_checksum}"
            );
        }
    }
    info!("Checksum verification passed!");
}
