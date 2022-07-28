use std::ops::Mul;
use std::str::FromStr;
use std::sync::Arc;

use bigdecimal::BigDecimal;
use bigdecimal::FromPrimitive;
use bigdecimal::ToPrimitive;
use ethabi::Token;
use ethers::prelude::U256;
use ethers::types::transaction::eip2718::TypedTransaction;
use future::ready;
use futures::{future, StreamExt};
use futures_signals::{map_ref, signal::SignalExt};
use rayon::prelude::*;

use super::path_sequence::PathSequence;
use super::uniswap_providers::*;
use crate::arb_thread_pool::spawn;
use crate::crypto_math::*;
use crate::flashbot_strategy::utils::*;
use crate::swap_route::SwapRoute;
use crate::three_path_sequence::ThreePathSequence;
use crate::two_path_sequence::TwoPathSequence;
use crate::uniswap_transaction::*;
use std::any::*;
/*
#[derive(Debug, Clone)]
pub struct ArbitragePath {
    sequence: Arc<(dyn Any + 'static + Sync + Send)>
}

impl ArbitragePath {
    pub async fn new(sequence:Arc<(dyn Any + 'static + Sync + Send)>)-> Result<Arc<Self>,anyhow::Error> {
        if sequence.is::<ThreePathSequence>() {
            let path = sequence.downcast_ref::<ThreePathSequence>().unwrap();
            path.init(sequence.clone()).await;
            return Ok(Arc::new(Self{sequence: Arc::new(*path) }));
        }
        else if sequence.is::<TwoPathSequence>() {
            let path = sequence.downcast_ref::<TwoPathSequence>().unwrap();
            path.init(Arc::new(path.clone())).await;
            return Ok(Arc::new(Self{sequence: Arc::new(*path) }));
        }
        return Err(anyhow::format_err!("Bad Type"));
    }
}


#[test]
pub fn test_abi_encoding() {
    let tokens = vec![Token::String("test".to_string())];
    let call_data = ethers::abi::encode(&tokens);

}
 */
