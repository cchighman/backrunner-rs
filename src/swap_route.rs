use crate::contracts::bindings::uniswap_v2_router_02::UniswapV2Router02;
use crate::utils::conversions::big_rational_to_u256;
use async_std::sync::Arc;
use bigdecimal::BigDecimal;
use num::BigRational;
use num_bigint::BigInt;
use std::ops::{Sub, Add};
use std::time::SystemTime;
use url::Url;

use super::uniswap_providers::*;
use anyhow;
use anyhow::Result;
use ethers::abi::Token;
use ethers::prelude::*;
use ethers::prelude::{Address, U256};
use ethers::providers::Middleware;
use ethers_flashbots::FlashbotsMiddleware;
use std::str::FromStr;
use std::time::UNIX_EPOCH;

use ethers::contract::{
    builders::{ContractCall, Event},
    Contract, Lazy,
};

use crate::contracts::bindings::uniswap_v2_pair::UniswapV2Pair;

#[derive(Clone, Debug)]
pub struct SwapRoute {
    pub pair: (Address, Address),
    pub source_amount: U256,
    pub source_amount_0: U256,
    pub source_amount_1: U256,
    pub transfer_amt: U256,
    pub dest_amount: U256,
    pub router: Address,
    pair_id: H160,
    pub pair_id_2: H160
}

impl SwapRoute {
    pub fn new(
        tokens: (Address, Address),
        source: U256,
        source0: U256,
        source1: U256,
        dest: U256,
        router: Address,
        pair_id: Address,
        pair_id_2: Address
    ) -> Self {
        let mut transfer = if source0.eq(&U256::zero()) { source1 } else { source0 };
        Self {
            pair: tokens,
            source_amount: source,
            source_amount_0: source0,
            source_amount_1: source1,
            transfer_amt: transfer,
            dest_amount: dest,
            router,
            pair_id,
            pair_id_2
        }
    }

    pub async fn swap(&self)->Result<Bytes, anyhow::Error> {
        let pair_contract = UniswapV2Pair::new(self.pair_id, mainnet::client.clone());
        let contract_call = pair_contract.swap(self.source_amount_0, self.source_amount_1 , self.pair_id_2, Bytes::default());
        
        Ok(contract_call.calldata().unwrap())
    }

    pub async fn swap_tokens_for_exact_tokens(&self) -> Result<Bytes, anyhow::Error> {
        let router_contract =
            UniswapV2Router02::new(*mainnet::router_v2, mainnet::client.clone());
/* 
        let pair_contract = UniswapV2Pair::new(self.pair_id, mainnet::client.clone());
        let contract_call = pair_contract.swap(self.source_amount.add(U256::from(1000_i32)), self.dest_amount.add(U256::from(1000_i32)), mainnet::flash_contract.clone(), Bytes::default());
        Ok(contract_call.calldata().unwrap())
   */     
        let payload = router_contract
            .swap_exact_tokens_for_tokens(
                self.source_amount,
                self.dest_amount,
                vec![self.pair.0, self.pair.1],
                *mainnet::to,
                mainnet::valid_timestamp(),
            )
            .calldata()
            .unwrap();
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
                self.valid_timestamp(),
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
                self.valid_timestamp(),
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
                self.valid_timestamp(),
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
            self.pair.0.symbol().as_str(),
            self.pair.1.symbol().as_str(),
        ) {
            ("WETH", _) => self.swap_tokens_for_exact_tokens(),
            ("ETH", _) => self.swap_eth_for_exact_tokens(),
            (_, "WETH") => self.swap_tokens_for_exact_tokens(),
            (_, "ETH") => self.swap_tokens_for_exact_eth(),
            (_, _) => self.swap_tokens_for_exact_tokens(),
        }
        */
        self.swap().await
        //self.swap_tokens_for_exact_tokens().await
    }

    pub async fn route_calldata(
        swap_routes: Vec<SwapRoute>,
        calls: Vec<
            ethers::prelude::builders::ContractCall<
                ethers::prelude::SignerMiddleware<
                    ethers::prelude::Provider<ethers::prelude::Http>,
                    ethers::prelude::Wallet<ethers::prelude::k256::ecdsa::SigningKey>,
                >,
                bool,
            >,
        >,
    ) -> Result<Bytes, anyhow::Error> {
        /* For each pair, get abi-encoded swap call */
        let miner_tip = Token::Uint(U256::zero());

        let mut trade_routers = Vec::<Token>::new();
        let mut sell_tokens = Vec::<Token>::new();
        let mut call_data = Vec::<Token>::new();
        let mut transfer_amts = Vec::<Token>::new();

        /* Build data */
        for trade in swap_routes {
            trade_routers.push(Token::Address(trade.pair_id));
            sell_tokens.push(Token::Address(trade.pair.0));
            sell_tokens.push(Token::Address(trade.pair.1));
            call_data.push(Token::Bytes(trade.calldata().await?.to_vec()));
            transfer_amts.push(Token::Uint(trade.transfer_amt));
            
          
        }

        for call in calls {
            call_data.push(Token::Bytes(call.calldata().unwrap().to_vec()));
        }
        dbg!(&transfer_amts);
        /* abi encode data */
        let tokens = vec![
            miner_tip,
            Token::Array(trade_routers),
            Token::Array(sell_tokens),
            Token::Array(call_data),
            Token::Array(transfer_amts),
        ];
        Ok(Bytes::from(abi::encode(&tokens)))
    }
}
