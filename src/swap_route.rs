use std::time::SystemTime;

use crate::uniswap_providers::UNISWAP_PROVIDERS;
use anyhow;
use anyhow::Result;
use ethers::abi::Token;
use ethers::prelude::*;
use ethers::prelude::{Address, U256};
use ethers::providers::Middleware;
use std::time::UNIX_EPOCH;

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
    /*
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

    /
    Provided some amount for some pair, return abi-encoded data for swap
     */
    pub async fn calldata(&self) -> Bytes {
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
        //self.swap_tokens_for_exact_tokens().await
        return Bytes::default();
    }
    pub fn get_valid_timestamp(&self) -> U256 {
        let start = SystemTime::now();
        let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();
        let time_millis = since_epoch
            .as_millis()
            .checked_add(*UNISWAP_PROVIDERS.TIMESTAMP_SEED)
            .unwrap();
        return U256::from(time_millis);
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
}
