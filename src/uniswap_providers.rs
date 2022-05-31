use std::ops::Deref;
use std::sync::Arc;

use ethers::core::k256::ecdsa::SigningKey;
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::signers::{coins_bip39::English, MnemonicBuilder};
use once_cell::sync::Lazy;

pub(crate) static ROPSTEN_WALLET: Lazy<Wallet<SigningKey>> = Lazy::new(|| {
    MnemonicBuilder::<English>::default()
        .phrase("unveil spoon stable govern diesel park glory visa lucky teach aspect spy")
        .index(0u32)
        .unwrap()
        .build()
        .unwrap()
        .with_chain_id(3_u64)
});

pub(crate) static ROPSTEN_PROVIDER: Lazy<
    Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
> = Lazy::new(|| {
    Arc::new(SignerMiddleware::new(
        Provider::<Http>::try_from("https://ropsten.infura.io/v3/7b15aafb575849f4ab4eaccc2725b4a7")
            .unwrap(),
        (*ROPSTEN_WALLET.deref()).clone(),
    ))
});
