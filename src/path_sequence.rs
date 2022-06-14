use crate::{sequence_token::SequenceToken, crypto_pair::CryptoPair};
use std::sync::Arc;
use bigdecimal::BigDecimal;
use ethers::prelude::*;
use super::*;
use std::collections::HashMap;
use async_trait::*;

#[async_trait]
pub trait PathSequence {
     async fn new (pairs: Vec<Arc<CryptoPair>>, sequence_tokens: Vec<SequenceToken>)->Arc<(dyn Any + 'static + Sync + Send)>;
     async fn init(&self,  arb_ref: Arc<(dyn Any + 'static + Sync + Send)>)->Result<(),anyhow::Error>; 
     fn as_any(&self)->&dyn Any;
     fn arb_index(&self) -> BigDecimal;
     fn a1(&self)->&SequenceToken;
     fn b1(&self)->&SequenceToken;
     fn a2(&self)->&SequenceToken;
     fn b2(&self)->&SequenceToken;

}
