#![cfg(feature = "client")]

use std::path::PathBuf;

use zcash_protocol::{PoolType, ShieldedProtocol};

use zingolib::{
    testutils::lightclient::{from_inputs, get_base_address},
    testvectors::REG_O_ADDR_FROM_ABANDONART,
};

use zcash_local_net::{
    client,
    indexer::{Indexer as _, Lightwalletd, LightwalletdConfig, Zainod, ZainodConfig},
    network, utils,
    validator::{Validator, Zcashd, ZcashdConfig, Zebrad, ZebradConfig, ZEBRAD_DEFAULT_MINER},
    LocalNet,
};

const ZCASHD_BIN: Option<PathBuf> = None;
const ZCASH_CLI_BIN: Option<PathBuf> = None;
const ZEBRAD_BIN: Option<PathBuf> = None;
const LIGHTWALLETD_BIN: Option<PathBuf> = None;
const ZAINOD_BIN: Option<PathBuf> = None;

#[tokio::test]
async fn launch_zcashd() {
    tracing_subscriber::fmt().init();

    let zcashd = Zcashd::launch(ZcashdConfig {
        zcashd_bin: ZCASHD_BIN,
        zcash_cli_bin: ZCASH_CLI_BIN,
        rpc_listen_port: None,
        activation_heights: network::ActivationHeights::default(),
        miner_address: Some(REG_O_ADDR_FROM_ABANDONART),
        chain_cache: None,
    })
    .await
    .unwrap();
    zcashd.print_stdout();
    zcashd.print_stderr();
}

#[tokio::test]
async fn launch_zebrad() {
    tracing_subscriber::fmt().init();

    let zebrad = Zebrad::launch(ZebradConfig {
        zebrad_bin: ZEBRAD_BIN,
        network_listen_port: None,
        rpc_listen_port: None,
        activation_heights: network::ActivationHeights::default(),
        miner_address: ZEBRAD_DEFAULT_MINER,
        chain_cache: None,
        network: network::Network::Regtest,
    })
    .await
    .unwrap();
    zebrad.print_stdout();
    zebrad.print_stderr();
}

#[tokio::test]
async fn launch_zebrad_with_cache() {
    tracing_subscriber::fmt().init();

    let zebrad = Zebrad::launch(ZebradConfig {
        zebrad_bin: ZEBRAD_BIN,
        network_listen_port: None,
        rpc_listen_port: None,
        activation_heights: network::ActivationHeights::default(),
        miner_address: ZEBRAD_DEFAULT_MINER,
        chain_cache: Some(utils::chain_cache_dir().join("client_rpc_tests_large")),
        network: network::Network::Regtest,
    })
    .await
    .unwrap();
    zebrad.print_stdout();
    zebrad.print_stderr();

    assert_eq!(zebrad.get_chain_height().await, 52.into());
}

#[tokio::test]
async fn launch_localnet_zainod_zcashd() {
    tracing_subscriber::fmt().init();

    let local_net = LocalNet::<Zainod, Zcashd>::launch(
        ZainodConfig {
            zainod_bin: ZAINOD_BIN,
            listen_port: None,
            validator_port: 0,
            network: network::Network::Regtest,
        },
        ZcashdConfig {
            zcashd_bin: ZCASHD_BIN,
            zcash_cli_bin: ZCASH_CLI_BIN,
            rpc_listen_port: None,
            activation_heights: network::ActivationHeights::default(),
            miner_address: Some(REG_O_ADDR_FROM_ABANDONART),
            chain_cache: None,
        },
    )
    .await;

    local_net.validator().print_stdout();
    local_net.validator().print_stderr();
    local_net.indexer().print_stdout();
    local_net.indexer().print_stderr();
}

#[ignore = "flake"]
#[tokio::test]
async fn launch_localnet_zainod_zebrad() {
    tracing_subscriber::fmt().init();

    let local_net = LocalNet::<Zainod, Zebrad>::launch(
        ZainodConfig {
            zainod_bin: ZAINOD_BIN,
            listen_port: None,
            validator_port: 0,
            network: network::Network::Regtest,
        },
        ZebradConfig {
            zebrad_bin: ZEBRAD_BIN,
            network_listen_port: None,
            rpc_listen_port: None,
            activation_heights: network::ActivationHeights::default(),
            miner_address: ZEBRAD_DEFAULT_MINER,
            chain_cache: None,
            network: network::Network::Regtest,
        },
    )
    .await;

    local_net.validator().print_stdout();
    local_net.validator().print_stderr();
    local_net.indexer().print_stdout();
    local_net.indexer().print_stderr();
}

#[tokio::test]
async fn launch_localnet_lightwalletd_zcashd() {
    tracing_subscriber::fmt().init();

    let local_net = LocalNet::<Lightwalletd, Zcashd>::launch(
        LightwalletdConfig {
            lightwalletd_bin: LIGHTWALLETD_BIN,
            listen_port: None,
            zcashd_conf: PathBuf::new(),
        },
        ZcashdConfig {
            zcashd_bin: ZCASHD_BIN,
            zcash_cli_bin: ZCASH_CLI_BIN,
            rpc_listen_port: None,
            activation_heights: network::ActivationHeights::default(),
            miner_address: Some(REG_O_ADDR_FROM_ABANDONART),
            chain_cache: None,
        },
    )
    .await;

    local_net.validator().print_stdout();
    local_net.validator().print_stderr();
    local_net.indexer().print_stdout();
    local_net.indexer().print_lwd_log();
    local_net.indexer().print_stderr();
}

#[tokio::test]
async fn launch_localnet_lightwalletd_zebrad() {
    tracing_subscriber::fmt().init();

    let local_net = LocalNet::<Lightwalletd, Zebrad>::launch(
        LightwalletdConfig {
            lightwalletd_bin: LIGHTWALLETD_BIN,
            listen_port: None,
            zcashd_conf: PathBuf::new(),
        },
        ZebradConfig {
            zebrad_bin: ZEBRAD_BIN,
            network_listen_port: None,
            rpc_listen_port: None,
            activation_heights: network::ActivationHeights::default(),
            miner_address: ZEBRAD_DEFAULT_MINER,
            chain_cache: None,
            network: network::Network::Regtest,
        },
    )
    .await;

    local_net.validator().print_stdout();
    local_net.validator().print_stderr();
    local_net.indexer().print_stdout();
    local_net.indexer().print_lwd_log();
    local_net.indexer().print_stderr();
}

#[tokio::test]
async fn zainod_zcashd_basic_send() {
    tracing_subscriber::fmt().init();

    let local_net = LocalNet::<Zainod, Zcashd>::launch(
        ZainodConfig {
            zainod_bin: ZAINOD_BIN,
            listen_port: None,
            validator_port: 0,
            network: network::Network::Regtest,
        },
        ZcashdConfig {
            zcashd_bin: ZCASHD_BIN,
            zcash_cli_bin: ZCASH_CLI_BIN,
            rpc_listen_port: None,
            activation_heights: network::ActivationHeights::default(),
            miner_address: Some(REG_O_ADDR_FROM_ABANDONART),
            chain_cache: None,
        },
    )
    .await;

    let lightclient_dir = tempfile::tempdir().unwrap();
    let (faucet, recipient) = client::build_lightclients(
        lightclient_dir.path().to_path_buf(),
        local_net.indexer().port(),
    )
    .await;

    faucet.do_sync(false).await.unwrap();
    from_inputs::quick_send(
        &faucet,
        vec![(
            &get_base_address(&recipient, PoolType::Shielded(ShieldedProtocol::Orchard)).await,
            100_000,
            None,
        )],
    )
    .await
    .unwrap();
    local_net.validator().generate_blocks(1).await.unwrap();
    faucet.do_sync(false).await.unwrap();
    recipient.do_sync(false).await.unwrap();

    let recipient_balance = recipient.do_balance().await;
    assert_eq!(recipient_balance.verified_orchard_balance, Some(100_000));

    local_net.validator().print_stdout();
    local_net.validator().print_stderr();
    local_net.indexer().print_stdout();
    local_net.indexer().print_stderr();
    println!("faucet balance:");
    println!("{:?}\n", faucet.do_balance().await);
    println!("recipient balance:");
    println!("{:?}\n", recipient_balance);
}

#[ignore = "flake"]
#[tokio::test]
async fn zainod_zebrad_basic_send() {
    tracing_subscriber::fmt().init();

    let local_net = LocalNet::<Zainod, Zebrad>::launch(
        ZainodConfig {
            zainod_bin: ZAINOD_BIN,
            listen_port: None,
            validator_port: 0,
            network: network::Network::Regtest,
        },
        ZebradConfig {
            zebrad_bin: ZEBRAD_BIN,
            network_listen_port: None,
            rpc_listen_port: None,
            activation_heights: network::ActivationHeights::default(),
            miner_address: ZEBRAD_DEFAULT_MINER,
            chain_cache: None,
            network: network::Network::Regtest,
        },
    )
    .await;

    let lightclient_dir = tempfile::tempdir().unwrap();
    let (faucet, recipient) = client::build_lightclients(
        lightclient_dir.path().to_path_buf(),
        local_net.indexer().port(),
    )
    .await;

    local_net.validator().generate_blocks(100).await.unwrap();
    faucet.do_sync(false).await.unwrap();
    faucet.quick_shield().await.unwrap();
    local_net.validator().generate_blocks(1).await.unwrap();
    faucet.do_sync(false).await.unwrap();

    from_inputs::quick_send(
        &faucet,
        vec![(
            &get_base_address(&recipient, PoolType::Shielded(ShieldedProtocol::Orchard)).await,
            100_000,
            None,
        )],
    )
    .await
    .unwrap();
    local_net.validator().generate_blocks(1).await.unwrap();
    faucet.do_sync(false).await.unwrap();
    recipient.do_sync(false).await.unwrap();

    let recipient_balance = recipient.do_balance().await;
    assert_eq!(recipient_balance.verified_orchard_balance, Some(100_000));

    local_net.validator().print_stdout();
    local_net.validator().print_stderr();
    local_net.indexer().print_stdout();
    local_net.indexer().print_stderr();
    println!("faucet balance:");
    println!("{:?}\n", faucet.do_balance().await);
    println!("recipient balance:");
    println!("{:?}\n", recipient_balance);
}

#[tokio::test]
async fn lightwalletd_zcashd_basic_send() {
    tracing_subscriber::fmt().init();

    let local_net = LocalNet::<Lightwalletd, Zcashd>::launch(
        LightwalletdConfig {
            lightwalletd_bin: LIGHTWALLETD_BIN,
            listen_port: None,
            zcashd_conf: PathBuf::new(),
        },
        ZcashdConfig {
            zcashd_bin: ZCASHD_BIN,
            zcash_cli_bin: ZCASH_CLI_BIN,
            rpc_listen_port: None,
            activation_heights: network::ActivationHeights::default(),
            miner_address: Some(REG_O_ADDR_FROM_ABANDONART),
            chain_cache: None,
        },
    )
    .await;

    let lightclient_dir = tempfile::tempdir().unwrap();
    let (faucet, recipient) = client::build_lightclients(
        lightclient_dir.path().to_path_buf(),
        local_net.indexer().port(),
    )
    .await;

    faucet.do_sync(false).await.unwrap();
    from_inputs::quick_send(
        &faucet,
        vec![(
            &get_base_address(&recipient, PoolType::Shielded(ShieldedProtocol::Orchard)).await,
            100_000,
            None,
        )],
    )
    .await
    .unwrap();
    local_net.validator().generate_blocks(1).await.unwrap();
    faucet.do_sync(false).await.unwrap();
    recipient.do_sync(false).await.unwrap();

    let recipient_balance = recipient.do_balance().await;
    assert_eq!(recipient_balance.verified_orchard_balance, Some(100_000));

    local_net.validator().print_stdout();
    local_net.validator().print_stderr();
    local_net.indexer().print_stdout();
    local_net.indexer().print_lwd_log();
    local_net.indexer().print_stderr();
    println!("faucet balance:");
    println!("{:?}\n", faucet.do_balance().await);
    println!("recipient balance:");
    println!("{:?}\n", recipient_balance);
}

#[tokio::test]
async fn lightwalletd_zebrad_basic_send() {
    tracing_subscriber::fmt().init();

    let local_net = LocalNet::<Lightwalletd, Zebrad>::launch(
        LightwalletdConfig {
            lightwalletd_bin: LIGHTWALLETD_BIN,
            listen_port: None,
            zcashd_conf: PathBuf::new(),
        },
        ZebradConfig {
            zebrad_bin: ZEBRAD_BIN,
            network_listen_port: None,
            rpc_listen_port: None,
            activation_heights: network::ActivationHeights::default(),
            miner_address: ZEBRAD_DEFAULT_MINER,
            chain_cache: None,
            network: network::Network::Regtest,
        },
    )
    .await;

    let lightclient_dir = tempfile::tempdir().unwrap();
    let (faucet, recipient) = client::build_lightclients(
        lightclient_dir.path().to_path_buf(),
        local_net.indexer().port(),
    )
    .await;

    local_net.validator().generate_blocks(100).await.unwrap();
    faucet.do_sync(false).await.unwrap();
    faucet.quick_shield().await.unwrap();
    local_net.validator().generate_blocks(1).await.unwrap();
    faucet.do_sync(false).await.unwrap();

    from_inputs::quick_send(
        &faucet,
        vec![(
            &get_base_address(&recipient, PoolType::Shielded(ShieldedProtocol::Orchard)).await,
            100_000,
            None,
        )],
    )
    .await
    .unwrap();
    local_net.validator().generate_blocks(1).await.unwrap();
    faucet.do_sync(false).await.unwrap();
    recipient.do_sync(false).await.unwrap();

    let recipient_balance = recipient.do_balance().await;
    assert_eq!(recipient_balance.verified_orchard_balance, Some(100_000));

    local_net.validator().print_stdout();
    local_net.validator().print_stderr();
    local_net.indexer().print_stdout();
    local_net.indexer().print_stderr();
    println!("faucet balance:");
    println!("{:?}\n", faucet.do_balance().await);
    println!("recipient balance:");
    println!("{:?}\n", recipient_balance);
}
