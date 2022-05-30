use crate::contracts::bindings::uniswap_v2_router_02::UniswapV2Router02;
use crate::crypto_pair::CryptoPair;
use crate::uniswap_providers::ROPSTEN_PROVIDER;
use ethabi::{Bytes, Token};
use ethereum_types::{Address, U256};
use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::prelude::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::signers::Wallet;
use once_cell::sync::Lazy;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

static TIMESTAMP_SEED: u128 = 30000_u128;
static MAX_AMOUNT: Lazy<U256> =
    Lazy::new(|| U256::from_str("9999999999999999999999999999999999").unwrap());

pub(crate) static TO_ADDRESS: Lazy<Address> =
    Lazy::new(|| Address::from_str("0x5C1201e06F2EB55dDf656F0a82e57cF92F634273").unwrap());

// Ropsten Uniswap v2
// Router: 0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D
static ROPSTEN_ROUTER_V2_ADDY: Lazy<Address> =
    Lazy::new(|| Address::from_str("0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D").unwrap());

static ROUTER_CONTRACT: Lazy<
    UniswapV2Router02<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
> = Lazy::new(|| UniswapV2Router02::new(*ROPSTEN_ROUTER_V2_ADDY, Arc::clone(&*ROPSTEN_PROVIDER)));

#[derive(Clone)]
pub struct SwapRoute {
    pub pair: Arc<CryptoPair>,
    pub source_amount: U256,
    pub dest_amount: U256,
}

impl SwapRoute {
    /* TODO - For each pair */

    pub fn swap_eth_for_exact_tokens(&self) -> ethers::prelude::Bytes {
        return (*ROUTER_CONTRACT)
            .swap_eth_for_exact_tokens(
                self.dest_amount,
                vec![self.pair.left_id().clone(), self.pair.right_id().clone()],
                *TO_ADDRESS,
                self.get_valid_timestamp(),
            )
            .calldata()
            .unwrap();
    }

    pub fn swap_tokens_for_exact_eth(&self) -> ethers::prelude::Bytes {
        return (*ROUTER_CONTRACT)
            .swap_tokens_for_exact_eth(
                self.dest_amount,
                *MAX_AMOUNT,
                vec![self.pair.left_id().clone(), self.pair.right_id().clone()],
                *TO_ADDRESS,
                self.get_valid_timestamp(),
            )
            .calldata()
            .unwrap();
    }

    pub fn swap_tokens_for_exact_tokens(&self) -> ethers::prelude::Bytes {
        return (*ROUTER_CONTRACT)
            .swap_tokens_for_exact_tokens(
                self.dest_amount,
                *MAX_AMOUNT,
                vec![self.pair.left_id().clone(), self.pair.right_id().clone()],
                *TO_ADDRESS,
                self.get_valid_timestamp(),
            )
            .calldata()
            .unwrap();
    }
    /*
    Provided some amount for some pair, return abi-encoded data for swap
     */
    pub fn calldata(&self) -> ethers::prelude::Bytes {
        match (
            self.pair.left_symbol().as_str(),
            self.pair.right_symbol().as_str(),
        ) {
            ("WETH", _) => self.swap_tokens_for_exact_tokens(),
            ("ETH", _) => self.swap_eth_for_exact_tokens(),
            (_, "WETH") => self.swap_tokens_for_exact_tokens(),
            (_, "ETH") => self.swap_tokens_for_exact_eth(),
            (_, _) => self.swap_tokens_for_exact_tokens(),
        }
    }
    fn get_valid_timestamp(&self) -> U256 {
        let start = SystemTime::now();
        let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();
        let time_millis = since_epoch.as_millis().checked_add(TIMESTAMP_SEED).unwrap();

        return U256::from(time_millis);
    }
}

pub fn route_calldata(swap_routes: &[SwapRoute]) -> ethers::prelude::Bytes {
    /* For each pair, get abi-encoded swap call */
    let miner_tip = Token::Uint(U256::from(0));

    let mut trade_routers = Vec::<Token>::new();
    let mut sell_tokens = Vec::<Token>::new();
    let mut swap_data = Vec::<Token>::new();

    /* Build data */
    for trade in swap_routes {
        trade_routers.push(Token::Address(trade.pair.router().clone()));
        sell_tokens.push(Token::Address(trade.pair.right_id().clone()));
        swap_data.push(Token::Bytes(trade.calldata().clone().to_vec()));
    }

    /* abi encode data */
    let tokens = vec![
        miner_tip,
        Token::Array(trade_routers),
        Token::Array(sell_tokens),
        Token::Array(swap_data),
    ];
    return ethers::prelude::Bytes::from(ethers::abi::encode(&tokens));
}
