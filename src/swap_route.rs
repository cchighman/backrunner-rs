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
use crate::uniswap_providers::MAINNET_PROVIDER;
use crate::uniswap_providers::TO_ADDRESS;
use crate::uniswap_providers::MAX_AMOUNT;
use crate::uniswap_providers::ROUTER_CONTRACT;
use crate::uniswap_providers::TIMESTAMP_SEED;

use std::env;
#[derive(Clone)]
pub struct SwapRoute {
    pub pair: (Address, Address),
    pub source_amount: U256,
    pub dest_amount: U256,
    pub router: Address,
}

impl SwapRoute {
    /* TODO - For each pair */

    pub fn new(tokens: (Address, Address), source: U256, dest: U256, router: Address) -> Self {
        Self {
            pair: tokens.clone(),
            source_amount: source,
            dest_amount: dest,
            router,
        }
    }

    pub async fn swap_eth_for_exact_tokens(&self) -> Bytes {
        (*ROUTER_CONTRACT)
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
            .unwrap()
    }

    pub async fn swap_tokens_for_exact_eth(&self) -> Bytes {
        (*ROUTER_CONTRACT)
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
            .unwrap()
    }

    pub async fn swap_tokens_for_exact_tokens(&self) -> Bytes {
        (*ROUTER_CONTRACT)
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
            .unwrap()
    }

    pub async fn swap_exact_tokens_for_tokens(&self) -> Bytes {
        (*ROUTER_CONTRACT)
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
            .unwrap()
    }

    /*
    Provided some amount for some pair, return abi-encoded data for swap
     */
    pub async fn calldata(&self) -> ethers::core::types::Bytes {
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
        self.swap_tokens_for_exact_tokens().await
    }
    fn get_valid_timestamp(&self) -> U256 {
        let start = SystemTime::now();
        let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();
        let time_millis = since_epoch.as_millis().checked_add(TIMESTAMP_SEED).unwrap();
        return U256::from(time_millis);
    }
}

pub async fn route_calldata(swap_routes: Vec<SwapRoute>) -> Bytes {
    /* For each pair, get abi-encoded swap call */
    let miner_tip = Token::Uint(U256::from(0));

    let mut trade_routers = Vec::<Token>::new();
    let mut sell_tokens = Vec::<Token>::new();
    let mut swap_data = Vec::<Token>::new();

    /* Build data */
    for trade in swap_routes {
        trade_routers.push(Token::Address(trade.router));
        sell_tokens.push(Token::Address(trade.pair.1));
        swap_data.push(Token::Bytes(trade.calldata().await.clone().to_vec()));
    }

    /* abi encode data */
    let tokens = vec![
        miner_tip,
        Token::Array(trade_routers),
        Token::Array(sell_tokens),
        Token::Array(swap_data),
    ];
    Bytes::from(abi::encode(&tokens))
}
