use super::*;
use crate::three_path_sequence::ThreePathSequence;
use crate::two_path_sequence::TwoPathSequence;
use crate::{crypto_pair::CryptoPair, sequence_token::SequenceToken};
use anyhow::Error;
use async_trait::*;
use ethers::prelude::*;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

pub async fn create(
    crypto_path: Vec<CryptoPair>,
    crypto_pairs: &HashMap<Address, Arc<CryptoPair>>,
) -> Result<Arc<(dyn Any + 'static + Sync + Send)>, anyhow::Error> {
    match crypto_path.len() {
        //   2 => Ok(two_path_sequence::cyclic_order(crypto_path, crypto_pairs).await.unwrap()),
        3 => three_path_sequence::cyclic_order(crypto_path, crypto_pairs)
            .await
            ,
        _ => Err(anyhow::format_err!("Path Sequence Doesnt Exist")),
    }
}
