
    use std::fmt;
    use ethers::abi::Token;
    use std::str::FromStr;
    use std::sync::Arc;
    use std::time::{SystemTime, UNIX_EPOCH};
    
    use ethers::core::k256::ecdsa::SigningKey;
    use ethers::middleware::SignerMiddleware;
    use ethers::prelude::{Address, U256};
    use ethers::providers::Middleware;
    use ethers::providers::{Http, Provider};
    use ethers::signers::Signer;
    use ethers::signers::Wallet;
    use ethers::contract::Lazy;
    
    //use crate::contracts::bindings::uniswap_v2_router_02::UniswapV2Router02;
    use anyhow;
    use ethers::abi::AbiDecode;
    use ethers::core::utils::keccak256;
    use ethers::core::{rand::thread_rng, types::transaction::eip2718::TypedTransaction};
    use ethers::prelude::*;
    use ethers::providers::Ws;
    use ethers::signers::{coins_bip39::English, MnemonicBuilder};
    use std::convert::TryFrom;
    use std::ops::Deref;
    
    use std::{collections::HashMap, fs::File, io, thread, time};
    use stream_cancel::Tripwire;
    use url::Url;
    
    
    use std::env;
    
    use crate::crypto_pair::CryptoPair;
    


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


/*
pub static GOERLI_WALLET: Lazy<Wallet<SigningKey>> = Lazy::new(|| {
    MnemonicBuilder::<English>::default()
        .phrase("unveil spoon stable govern diesel park glory visa lucky teach aspect spy")
        .index(0u32)
        .unwrap()
        .build()
        .unwrap()
        .with_chain_id(5_u64)
});

pub static FLASHBOTS_GOERLI_PROVIDER: Lazy<
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

pub(crate) static ROPSTEN_WALLET: Lazy<Wallet<SigningKey>> = Lazy::new(|| {
    MnemonicBuilder::<English>::default()
        .phrase("unveil spoon stable govern diesel park glory visa lucky teach aspect spy")
        .index(0u32)
        .unwrap()
        .build()
        .unwrap()
        .with_chain_id(3_u64)
});

pub static ROPSTEN_PROVIDER: Lazy<Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>> =
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

pub static MAINNET_BOT_SIGNER: Lazy<Wallet<SigningKey>> = Lazy::new(|| {
    let private_key = "7005b56052be4776bffe00ff781879c65aa87ac3d5f8945c0452f27e11fa9236";
    let wallet = private_key.parse::<LocalWallet>().unwrap();
    let wallet = wallet.with_chain_id(1u64);
    wallet
});

pub static MAINNET_BUNDLE_SIGNER: Lazy<Wallet<SigningKey>> = Lazy::new(|| {
    let private_key = "7005b56052be4776bffe00ff781879c65aa87ac3d5f8945c0452f27e11fa9236";
    let wallet = private_key.parse::<LocalWallet>().unwrap();
    let wallet = wallet.with_chain_id(1u64);
    wallet
});

pub static TIMESTAMP_SEED: u128 = 30000_u128;
pub static MAX_AMOUNT: Lazy<U256> =
    Lazy::new(|| U256::from_str("9999999999999999999999999999999999").unwrap());

pub static TO_ADDRESS: Lazy<Address> =
    Lazy::new(|| Address::from_str("0x5C1201e06F2EB55dDf656F0a82e57cF92F634273").unwrap());

pub static CONTRACT_ADDRESS: Lazy<Address> =
    Lazy::new(|| Address::from_str("0x5C1201e06F2EB55dDf656F0a82e57cF92F634273").unwrap());

pub static FROM_ADDRESS: Lazy<Address> =
    Lazy::new(|| Address::from_str("0x5C1201e06F2EB55dDf656F0a82e57cF92F634273").unwrap());


// Ropsten Uniswap v2
// Router: 0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D
static MAINNET_ROUTER_V2_ADDY: Lazy<Address> =
    Lazy::new(|| Address::from_str("0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D").unwrap());

    
pub static MAINNET_PROVIDER: Lazy<Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>> =
Lazy::new(|| {
    Arc::new(SignerMiddleware::new(
        Provider::<Http>::try_from(
            "https://mainnet.infura.io/v3/20ca45667c5d4fa6b259b9a36babe5c3",
        )
        .unwrap(),
        (*MAINNET_BOT_SIGNER.deref()).clone(),
    ))
});


pub static ROUTER_CONTRACT: Lazy<u8
> = Lazy::new(|| 7);
