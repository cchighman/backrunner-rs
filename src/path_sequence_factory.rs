use crate::{sequence_token::SequenceToken, crypto_pair::CryptoPair};
use crate::two_path_sequence::TwoPathSequence;
use crate::three_path_sequence::ThreePathSequence;
use std::sync::Arc;
use ethers::prelude::*;
use super::*;
use std::collections::HashMap;
use async_trait::*;
use anyhow::Error;


pub async fn create (
    crypto_path: Vec<CryptoPair>,
    crypto_pairs: &HashMap<Address, Arc<CryptoPair>>)->Result<Arc<(dyn Any + 'static + Sync + Send)>, anyhow::Error> {
        match crypto_path.len() {
         //   2 => Ok(two_path_sequence::cyclic_order(crypto_path, crypto_pairs).await.unwrap()),
            3 => Ok(three_path_sequence::cyclic_order(crypto_path, crypto_pairs).await.unwrap()),
            _ => Err(anyhow::format_err!("Path Sequence Doesnt Exist"))
        }
    }
   