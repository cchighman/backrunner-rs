use std::time::SystemTime;
use url::Url;
use async_std::sync::Arc;
use crate::contracts::bindings::uniswap_v2_router_02::UniswapV2Router02;

use anyhow;
use anyhow::Result;
use ethers::abi::Token;
use ethers::prelude::*;
use ethers::prelude::{Address, U256};
use ethers::providers::Middleware;
use std::time::UNIX_EPOCH;
use std::str::FromStr;
use ethers_flashbots::FlashbotsMiddleware;
use super::uniswap_providers::*;

#[derive(Clone, Debug)]
pub struct SwapRoute {
    pub pair: (Address, Address),
    pub source_amount: U256,
    pub dest_amount: U256,
    pub router: Address,
}

impl SwapRoute {
    pub fn new(tokens: (Address, Address), source: U256, dest: U256, router: Address) -> Self {
        Self {
            pair: tokens.clone(),
            source_amount: source,
            dest_amount: dest,
            router,
        }
    }

    pub async fn swap_tokens_for_exact_tokens(&self) -> Result<Bytes, anyhow::Error> {
    
        let router_contract = UniswapV2Router02::new(
            mainnet::router_v2.clone(),
            mainnet::client.clone());
         
        let payload = router_contract.swap_tokens_for_exact_tokens(
                self.dest_amount,
                mainnet::max_amount.clone(),
                vec![
                    self.pair.0,
                    self.pair.1
                ],
                mainnet::to.clone(),
                mainnet::get_valid_timestamp(),
            ).calldata().unwrap();
            Ok(payload)
            
            //Ok(Bytes::from(vec![3_u8, 1_u8]))
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
    pub async fn calldata(&self) -> Result<Bytes, anyhow::Error> {
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
        Ok(self.swap_tokens_for_exact_tokens().await?)
    }

    pub async fn route_calldata(swap_routes: Vec<SwapRoute>)->Result<Bytes, anyhow::Error> {
        /* For each pair, get abi-encoded swap call */
        let miner_tip = Token::Uint(U256::from(0));

        let mut trade_routers = Vec::<Token>::new();
        let mut sell_tokens = Vec::<Token>::new();
        let mut swap_data = Vec::<Token>::new();

        /* Build data */
        for trade in swap_routes {
            trade_routers.push(Token::Address(trade.router));
            sell_tokens.push(Token::Address(trade.pair.1));
            swap_data.push(Token::Bytes(trade.calldata().await?.to_vec()));
        }

        /* abi encode data */
        let tokens = vec![
            miner_tip,
            Token::Array(trade_routers),
            Token::Array(sell_tokens),
            Token::Array(swap_data),
        ];
        Ok(Bytes::from(abi::encode(&tokens)))
    }
}

