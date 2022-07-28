use crate::contracts::bindings::uniswap_v2_router_02::UniswapV2Router02;
use anyhow;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::*;
use ethers::prelude::{Address, U256};
use ethers::providers::Middleware;
use ethers::providers::{Http, Provider};
use ethers::signers::Signer;
use ethers::signers::Wallet;
use ethers_flashbots::FlashbotsMiddleware;
use once_cell::sync::Lazy;
use std::convert::TryFrom;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use url::Url;

//  Mainnet
// https://mainnet.infura.io/v3/20ca45667c5d4fa6b259b9a36babe5c3
// wss://mainnet.infura.io/ws/v3/20ca45667c5d4fa6b259b9a36babe5c3

// Goerli
// https://goerli.infura.io/v3/0ab0b9c9d5bf44818399aea45b5ade51
// wss://goerli.infura.io/ws/v3/0ab0b9c9d5bf44818399aea45b5ade51

// Alchemy Mainnet - Trace / Debug APIs
// API Key: EcNibd4j6LaA9r9pJJRkyAVRvZ_MvKk7
// https://eth-mainnet.alchemyapi.io/v2/EcNibd4j6LaA9r9pJJRkyAVRvZ_MvKk7
// wss://eth-mainnet.alchemyapi.io/v2/EcNibd4j6LaA9r9pJJRkyAVRvZ_MvKk7

pub mod mainnet {
    use super::*;

    static private_key: &str = "7005b56052be4776bffe00ff781879c65aa87ac3d5f8945c0452f27e11fa9236";

    pub static flashbots_client: Lazy<
        Arc<
            SignerMiddleware<
                FlashbotsMiddleware<Provider<Http>, Wallet<SigningKey>>,
                Wallet<SigningKey>,
            >,
        >,
    > = Lazy::new(|| {
        Arc::new(SignerMiddleware::new(
            FlashbotsMiddleware::new(
                infura_provider_http.clone(),
                Url::parse("https://relay.flashbots.net").unwrap(),
                flashbots_bundle_signer.clone(),
            ),
            wallet.clone(),
        ))
    });

    pub static flashbots_bundle_signer: Lazy<Wallet<SigningKey>> = Lazy::new(|| {
        private_key
            .parse::<LocalWallet>()
            .unwrap()
            .with_chain_id(1u64)
    });

    pub static wallet: Lazy<Wallet<SigningKey>> = Lazy::new(|| {
        private_key
            .parse::<LocalWallet>()
            .unwrap()
            .with_chain_id(1u64)
    });

    /* Clients */
    pub static client: Lazy<Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>> =
        Lazy::new(|| {
            Arc::new(SignerMiddleware::new(
                infura_provider_http.clone(),
                wallet.clone(),
            ))
        });
    pub static alchemy_http_client: Lazy<
        Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    > = Lazy::new(|| {
        Arc::new(SignerMiddleware::new(
            infura_provider_http.clone(),
            wallet.clone(),
        ))
    });

    /* Providers */
    pub static alchemy_provider_http: Lazy<Provider<Http>> = Lazy::new(|| {
        Provider::<Http>::try_from(
            "https://eth-mainnet.alchemyapi.io/v2/EcNibd4j6LaA9r9pJJRkyAVRvZ_MvKk7",
        )
        .unwrap()
    });
    pub static infura_provider_http: Lazy<Provider<Http>> = Lazy::new(|| {
        Provider::<Http>::try_from("https://mainnet.infura.io/v3/20ca45667c5d4fa6b259b9a36babe5c3")
            .unwrap()
    });

    /*
    pub static infura_provider_wss: Lazy<Provider::<Ws>> =
        Lazy::new(|| Provider::<Ws>::connect("wss://mainnet.infura.io/ws/v3/20ca45667c5d4fa6b259b9a36babe5c3"));
    */

    pub static router_v2: Lazy<Address> =
        Lazy::new(|| Address::from_str("0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D").unwrap());
    pub static flash_contract: Lazy<Address> =
        Lazy::new(|| Address::from_str("0x32504C2CF2F4096E0e85258fF383F3e34D5B6B0C").unwrap());

    pub static from: Lazy<Address> =
        Lazy::new(|| Address::from_str("0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D").unwrap());
    pub static to: Lazy<Address> =
        Lazy::new(|| Address::from_str("0x32504C2CF2F4096E0e85258fF383F3e34D5B6B0C").unwrap());
    pub static max_amount: Lazy<U256> =
        Lazy::new(|| U256::from_dec_str("9999999999999999999999999999999999").unwrap());

    pub fn valid_timestamp() -> U256 {
        let start = SystemTime::now();
        let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();
        let time_millis = since_epoch.as_millis().checked_add(30000_u128).unwrap();
        U256::from(time_millis)
    }
}

/*
pub async fn GOERLI_WALLET: Lazy<Wallet<SigningKey>> = Lazy::new(|| {
    MnemonicBuilder::<English>::default()
        .phrase("unveil spoon stable govern diesel park glory visa lucky teach aspect spy")
        .index(0u32)
        .unwrap()
        .build()
        .unwrap()
        .with_chain_id(5_u64)
});

pub async fn FLASHBOTS_GOERLI_PROVIDER: Lazy<
    Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
> = Lazy::new(|| {
    Arc::new(SignerMiddleware::new(
        FlashbotsMiddleware::new(
            Provider::<Http>::try_from("https://mainnet.eth.aragon.network").unwrap(),
            Url::parse("https://relay.flashbots.net").unwrap(),
            LocalWallet::new(&mut thread_rng()),
        ),
        LocalWallet::new(&mut thread_rng()),
    ))
});

*/
pub mod kovan {
    use super::*;

    static private_key: &str = "7005b56052be4776bffe00ff781879c65aa87ac3d5f8945c0452f27e11fa9236";

    pub static wallet: Lazy<Wallet<SigningKey>> = Lazy::new(|| {
        private_key
            .parse::<LocalWallet>()
            .unwrap()
            .with_chain_id(1u64)
    });

    /* Clients */
    pub static client: Lazy<Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>> =
        Lazy::new(|| {
            Arc::new(SignerMiddleware::new(
                infura_provider_http.clone(),
                wallet.clone(),
            ))
        });

    /* Providers */

    pub static infura_provider_http: Lazy<Provider<Http>> = Lazy::new(|| {
        Provider::<Http>::try_from("https://kovan.infura.io/v3/0ab0b9c9d5bf44818399aea45b5ade51")
            .unwrap()
    });

    pub static router_v2: Lazy<Address> =
        Lazy::new(|| Address::from_str("0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D").unwrap());
    pub static flash_contract: Lazy<Address> =
        Lazy::new(|| Address::from_str("0x32504C2CF2F4096E0e85258fF383F3e34D5B6B0C").unwrap());

    pub static from: Lazy<Address> =
        Lazy::new(|| Address::from_str("0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D").unwrap());
    pub static to: Lazy<Address> =
        Lazy::new(|| Address::from_str("0x32504C2CF2F4096E0e85258fF383F3e34D5B6B0C").unwrap());
    pub static max_amount: Lazy<U256> =
        Lazy::new(|| U256::from_dec_str("9999999999999999999999999999999999").unwrap());

    pub fn valid_timestamp() -> U256 {
        let start = SystemTime::now();
        let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();
        let time_millis = since_epoch.as_millis().checked_add(30000_u128).unwrap();
        U256::from(time_millis)
    }
}
/*

pub async fn ROPSTEN_PROVIDER: Lazy<Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>> =
    Lazy::new(|| {
        Arc::new(SignerMiddleware::new(
            Provider::<Http>::try_from(
                "https://ropsten.infura.io/v3/7b15aafb575849f4ab4eaccc2725b4a7",
            )
            .unwrap(),
            (*ROPSTEN_WALLET.deref()).clone(),
        ))
    });

*/
// Ropsten Uniswap v2
// Router: 0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D

/*
         pub async fn ROPSTEN_PROVIDER: Lazy<Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>> =
        Lazy::new(|| {
            Arc::new(SignerMiddleware::new(
                Provider::<Http>::try_from(
                    "https://ropsten.infura.io/v3/7b15aafb575849f4ab4eaccc2725b4a7",
                )
                .unwrap(),
                (*ROPSTEN_WALLET.deref()).clone(),
            ))
        });
*/

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Dex {
    Balancer,
    Curve,
    PancakeSwap,
    SushiSwap,
    QuickSwap,
    UniswapV2,
    UniswapV3,
}

impl fmt::Display for Dex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
