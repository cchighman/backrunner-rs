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

use once_cell::sync::Lazy;

use crate::contracts::bindings::uniswap_v2_router_02::UniswapV2Router02;
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

pub static GOERLI_WALLET: Lazy<Wallet<SigningKey>> = Lazy::new(|| {
    MnemonicBuilder::<English>::default()
        .phrase("unveil spoon stable govern diesel park glory visa lucky teach aspect spy")
        .index(0u32)
        .unwrap()
        .build()
        .unwrap()
        .with_chain_id(5_u64)
});
/*
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
*/
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

pub static TIMESTAMP_SEED: u128 = 30000_u128;
pub static MAX_AMOUNT: Lazy<U256> =
    Lazy::new(|| U256::from_str("9999999999999999999999999999999999").unwrap());

pub static TO_ADDRESS: Lazy<Address> =
    Lazy::new(|| Address::from_str("0x5C1201e06F2EB55dDf656F0a82e57cF92F634273").unwrap());

// Ropsten Uniswap v2
// Router: 0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D
static ROPSTEN_ROUTER_V2_ADDY: Lazy<Address> =
    Lazy::new(|| Address::from_str("0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D").unwrap());

pub static ROUTER_CONTRACT: Lazy<
    UniswapV2Router02<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
> = Lazy::new(|| UniswapV2Router02::new(*ROPSTEN_ROUTER_V2_ADDY, Arc::clone(&*ROPSTEN_PROVIDER)));

#[derive(Clone)]
pub struct SwapRoute {
    pub pair: (Address, Address),
    pub source_amount: U256,
    pub dest_amount: U256,
    pub router: Address,
}

impl SwapRoute {
    /* TODO - For each pair */

    pub fn new(tokens: (Address, Address), source: U256, dest: U256, router: H160) -> Self {
        Self {
            pair: tokens.clone(),
            source_amount: source,
            dest_amount: dest,
            router,
        }
    }

    pub fn swap_eth_for_exact_tokens(&self) -> ethers::core::types::Bytes {
        return (*ROUTER_CONTRACT)
            .swap_eth_for_exact_tokens(
                self.dest_amount,
                vec![
                    Address::from_str(&*self.pair.0.to_string()).unwrap(),
                    Address::from_str(&*self.pair.1.to_string()).unwrap(),
                ],
                *TO_ADDRESS,
                self.get_valid_timestamp(),
            )
            .calldata()
            .unwrap();
    }

    pub fn swap_tokens_for_exact_eth(&self) -> ethers::core::types::Bytes {
        return (*ROUTER_CONTRACT)
            .swap_tokens_for_exact_eth(
                self.dest_amount,
                *MAX_AMOUNT,
                vec![
                    Address::from_str(&*self.pair.0.to_string()).unwrap(),
                    Address::from_str(&*self.pair.1.to_string()).unwrap(),
                ],
                *TO_ADDRESS,
                self.get_valid_timestamp(),
            )
            .calldata()
            .unwrap();
    }

    pub fn swap_tokens_for_exact_tokens(&self) -> ethers::core::types::Bytes {
        return (*ROUTER_CONTRACT)
            .swap_tokens_for_exact_tokens(
                self.dest_amount,
                *MAX_AMOUNT,
                vec![
                    Address::from_str(&*self.pair.0.to_string()).unwrap(),
                    Address::from_str(&*self.pair.1.to_string()).unwrap(),
                ],
                *TO_ADDRESS,
                self.get_valid_timestamp(),
            )
            .calldata()
            .unwrap();
    }

    pub fn swap_exact_tokens_for_tokens(&self) -> ethers::core::types::Bytes {
        return (*ROUTER_CONTRACT)
            .swap_exact_tokens_for_tokens(
                self.source_amount,
                self.dest_amount,
                vec![
                    Address::from_str(&*self.pair.0.to_string()).unwrap(),
                    Address::from_str(&*self.pair.1.to_string()).unwrap(),
                ],
                *TO_ADDRESS,
                self.get_valid_timestamp(),
            )
            .calldata()
            .unwrap();
    }

    /*
    Provided some amount for some pair, return abi-encoded data for swap
     */
    pub fn calldata(&self) -> ethers::core::types::Bytes {
        /*
        match (
            self.pair.0.get_symbol().as_str(),
            self.pair.1.get_symbol().as_str(),
        ) {
            ("WETH", _) => self.swap_tokens_for_exact_tokens(),
            ("ETH", _) => self.swap_eth_for_exact_tokens(),
            (_, "WETH") => self.swap_tokens_for_exact_tokens(),
            (_, "ETH") => self.swap_tokens_for_exact_eth(),
            (_, _) => self.swap_tokens_for_exact_tokens(),
        }
        */
        self.swap_tokens_for_exact_tokens()
    }
    fn get_valid_timestamp(&self) -> U256 {
        let start = SystemTime::now();
        let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();
        let time_millis = since_epoch.as_millis().checked_add(TIMESTAMP_SEED).unwrap();
        return U256::from(time_millis);
    }
}

pub fn route_calldata(swap_routes: Vec<SwapRoute>) -> ethers::core::types::Bytes {
    /* For each pair, get abi-encoded swap call */
    let miner_tip = Token::Uint(U256::from(0));

    let mut trade_routers = Vec::<Token>::new();
    let mut sell_tokens = Vec::<Token>::new();
    let mut swap_data = Vec::<Token>::new();

    /* Build data */
    for trade in swap_routes {
        trade_routers.push(Token::Address(
            Address::from_str(&*trade.router.to_string()).unwrap(),
        ));
        sell_tokens.push(Token::Address(
            Address::from_str(&*trade.pair.1.to_string()).unwrap(),
        ));
        swap_data.push(Token::Bytes(trade.calldata().clone().to_vec()));
    }

    /* abi encode data */
    let tokens = vec![
        miner_tip,
        Token::Array(trade_routers),
        Token::Array(sell_tokens),
        Token::Array(swap_data),
    ];
    return ethers::core::types::Bytes::from(ethers::core::abi::encode(&tokens));
}