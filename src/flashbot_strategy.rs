use crate::contracts::bindings::ierc20::IERC20;
use crate::contracts::bindings::uniswap_v2_pair::UniswapV2Pair;
use crate::contracts::bindings::uniswap_v2_router_02::UniswapV2Router02;

use crate::sequence_token::SequenceToken;
use anyhow::Result;
use std::env;
use ethers::contract::Lazy;
use ethers::core::abi::Tokenize;
use ethers::core::types::transaction::eip2718::TypedTransaction;
use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::prelude::*;
use ethers::prelude::{Address, Signer, SignerMiddleware, Wallet, U256};
use ethers::signers::{coins_bip39::English, MnemonicBuilder};
use ethers_flashbots::BundleRequest;
use ethers_flashbots::*;
use lazy_static::__Deref;
use rand::thread_rng;

use std::str::FromStr;
use std::sync::Arc;
use url::Url;

pub(crate) static ROPSTEN_WALLET: Lazy<Wallet<SigningKey>> = Lazy::new(|| {
    MnemonicBuilder::<English>::default()
        .phrase("unveil spoon stable govern diesel park glory visa lucky teach aspect spy")
        .index(0u32)
        .unwrap()
        .build()
        .unwrap()
        .with_chain_id(3_u64)
});

pub static ROPSTEN_PROVIDER: Lazy<Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>> =
    Lazy::new(|| {
        Arc::new(SignerMiddleware::new(
            Provider::<Http>::try_from(
                "https://ropsten.infura.io/v3/7b15aafb575849f4ab4eaccc2725b4a7",
            )
            .unwrap(),
            (*ROPSTEN_WALLET.deref()).clone(),
        ))
    });

pub static TIMESTAMP_SEED: u128 = 30000_u128;
pub static MAX_AMOUNT: Lazy<U256> =
    Lazy::new(|| U256::from_str("9999999999999999999999999999999999").unwrap());

pub static TO_ADDRESS: Lazy<Address> =
    Lazy::new(|| Address::from_str("0x5C1201e06F2EB55dDf656F0a82e57cF92F634273").unwrap());

// Ropsten Uniswap v2
// Router: 0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D
static ROPSTEN_ROUTER_V2_ADDY: Lazy<Address> =
    Lazy::new(|| Address::from_str("0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D").unwrap());

pub static ROUTER_CONTRACT: Lazy<
    UniswapV2Router02<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
> = Lazy::new(|| UniswapV2Router02::new(*ROPSTEN_ROUTER_V2_ADDY, Arc::clone(&*ROPSTEN_PROVIDER)));

pub static GOERLI_WALLET: Lazy<Wallet<SigningKey>> = Lazy::new(|| {
    MnemonicBuilder::<English>::default()
        .phrase("unveil spoon stable govern diesel park glory visa lucky teach aspect spy")
        .index(0u32)
        .unwrap()
        .build()
        .unwrap()
        .with_chain_id(5_u64)
});
/*
pub static FLASHBOTS_GOERLI_PROVIDER: Lazy<
    Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
> = Lazy::new(|| {
    Arc::new(SignerMiddleware::new(
        FlashbotsMiddleware::new(
            Provider::<Http>::try_from("https://mainnet.eth.aragon.network").unwrap(),
            Url::parse("https://relay.flashbots.net").unwrap(),
            LocalWallet::new(&mut thread_rng()),
        ),
        LocalWallet::new(&mut thread_rng()),
    ))
});
//  Mainnet
// https://mainnet.infura.io/v3/20ca45667c5d4fa6b259b9a36babe5c3
// wss://mainnet.infura.io/ws/v3/20ca45667c5d4fa6b259b9a36babe5c3

// Goerli
// https://goerli.infura.io/v3/0ab0b9c9d5bf44818399aea45b5ade51
// wss://goerli.infura.io/ws/v3/0ab0b9c9d5bf44818399aea45b5ade51
*/

pub(crate) async fn do_flashbot_goerli(tx: &TypedTransaction) -> Result<()> {
    // Connect to the network - using URL used by metamask
    let provider =
        Provider::<Http>::try_from("https://goerli.infura.io/v3/0ab0b9c9d5bf44818399aea45b5ade51")?;

    let private_key = env::var("7005b56052be4776bffe00ff781879c65aa87ac3d5f8945c0452f27e11fa9236")?;
    let bundle_signer = private_key.parse::<LocalWallet>()?;
    let wallet = private_key.parse::<LocalWallet>()?;

    // Set chainId for goerli testnet
    let wallet = wallet.with_chain_id(1u64);
    let bundle_signer = bundle_signer.with_chain_id(1u64);

    let client = SignerMiddleware::new(
        FlashbotsMiddleware::new(
            provider,
            Url::parse("https://relay-goerli.flashbots.net")?,
            bundle_signer,
        ),
        wallet,
    );

    let block_number = client.inner().inner().get_block_number().await?;
    let signature = client.signer().sign_transaction(&tx).await?;
    let bundle = BundleRequest::new()
        .push_transaction(tx.rlp_signed(client.signer().chain_id(), &signature))
        .set_block(block_number + 1);

    // Simulate it
    let simulated_bundle = client.inner().simulate_bundle(&bundle).await?;
    println!("Simulated bundle: {:?}", simulated_bundle);

    // Send it
    let pending_bundle = client.inner().send_bundle(&bundle).await?;

    // You can also optionally wait to see if the bundle was included
    match pending_bundle.await {
        Ok(bundle_hash) => println!(
            "Bundle with hash {:?} was included in target block",
            bundle_hash
        ),
        Err(PendingBundleError::BundleNotIncluded) => {
            println!("Bundle was not included in target block.")
        }
        Err(e) => println!("An error occured."),
    }

    Ok(())
}

pub(crate) async fn do_flashbot_mainnet(tx: &TypedTransaction) -> Result<()> {
    // Connect to the network - using URL used by metamask
    let provider = Provider::<Http>::try_from(
        "https://mainnet.infura.io/v3/20ca45667c5d4fa6b259b9a36babe5c3",
    )?;

    let private_key = env::var("7005b56052be4776bffe00ff781879c65aa87ac3d5f8945c0452f27e11fa9236")?;
    let bundle_signer = private_key.parse::<LocalWallet>()?;
    let wallet = private_key.parse::<LocalWallet>()?;

    let wallet = wallet.with_chain_id(1u64);
    let bundle_signer = bundle_signer.with_chain_id(1u64);

    let client = SignerMiddleware::new(
        FlashbotsMiddleware::new(
            provider,
            Url::parse("https://relay.flashbots.net")?,
            bundle_signer,
        ),
        wallet,
    );
    dbg!(&client);
    let block_number = client.inner().inner().get_block_number().await?;
    let signature = client.signer().sign_transaction(&tx).await?;
    let bundle = BundleRequest::new()
        .push_transaction(tx.rlp_signed(client.signer().chain_id(), &signature))
        .set_block(block_number + 1);

    // Simulate it
    let simulated_bundle = client.inner().simulate_bundle(&bundle).await?;
    println!("Simulated bundle: {:?}", simulated_bundle);

    // Send it
    let pending_bundle = client.inner().send_bundle(&bundle).await?;

    // You can also optionally wait to see if the bundle was included
    match pending_bundle.await {
        Ok(bundle_hash) => println!(
            "Bundle with hash {:?} was included in target block",
            bundle_hash
        ),
        Err(PendingBundleError::BundleNotIncluded) => {
            println!("Bundle was not included in target block.")
        }
        Err(e) => println!("An error occured."),
    }

    Ok(())
}

/*
let bundle = get_bundle_for_test(&client).await?;
let current_block_number = client.inner().inner().get_block_number().await?;
let bundle = bundle
    .set_simulation_block(current_block_number)
    .set_simulation_timestamp(1731851886)
    .set_block(current_block_number + 1);

let raw_txs: Vec<Bytes> = bundle
    .transactions()
    .iter()
    .map(|tx| match tx {
        BundleTransaction::Signed(inner) => inner.rlp(),
        BundleTransaction::Raw(inner) => inner.clone(),
    })
    .collect();
let simulated_bundle = client.inner().simulate_bundle(&bundle).await?;
println!("Simulated bundle: {:?}", raw_txs);

// submitting multiple bundles to increase the probability on inclusion
for x in 0..10 {
    let bundle = get_bundle_for_test(&client).await?;
    let bundle = bundle.set_block(current_block_number + x);
    println!("Bundle Initialized");
    println!("{}", current_block_number + x);
    let pending_bundle = client.inner().send_bundle(&bundle).await?;
    match pending_bundle.await {
        Ok(bundle_hash) => println!(
            "Bundle with hash {:?} was included in target block",
            bundle_hash
        ),
        Err(PendingBundleError::BundleNotIncluded) => {
            println!("Bundle was not included in target block.")
        }
        Err(e) => println!("An error occured: {}", e),
    }
}

Ok(())
}

*/
async fn test_relay() -> Result<()> {
    let provider =
        Provider::<Http>::try_from("https://goerli.infura.io/v3/33ff530a5bfc4b418314cd6b5cc6fc64")?;

    // This is your searcher identity
    let bundle_signer = LocalWallet::new(&mut thread_rng());
    // This signs transactions
    let wallet = LocalWallet::new(&mut thread_rng());

    // Add signer and Flashbots middleware
    let client = SignerMiddleware::new(
        FlashbotsMiddleware::new(
            provider,
            Url::parse("https://relay.flashbots.net")?,
            bundle_signer,
        ),
        wallet,
    );

    // get last block number
    let block_number = client.get_block_number().await?;

    // Build a custom bundle that pays 0x0000000000000000000000000000000000000000
    let tx = {
        let mut inner: TypedTransaction = TransactionRequest::pay(Address::zero(), 100).into();
        client.fill_transaction(&mut inner, None).await?;
        inner
    };
    let signature = client.signer().sign_transaction(&tx).await?;
    let bundle = BundleRequest::new()
        .push_transaction(tx.rlp_signed(client.signer().chain_id(), &signature))
        .set_block(block_number + 1);

    // Simulate it
    let simulated_bundle = client.inner().simulate_bundle(&bundle).await?;
    println!("Simulated bundle: {:?}", simulated_bundle);

    // Send it
    let pending_bundle = client.inner().send_bundle(&bundle).await?;

    // You can also optionally wait to see if the bundle was included
    match pending_bundle.await {
        Ok(bundle_hash) => println!(
            "Bundle with hash {:?} was included in target block",
            bundle_hash
        ),
        Err(PendingBundleError::BundleNotIncluded) => {
            println!("Bundle was not included in target block.")
        }
        Err(e) => println!("An error occured: {}", e),
    }

    Ok(())
}

async fn get_bundle_for_test<M: 'static + Middleware, S: 'static + Signer>(
    client: &SignerMiddleware<M, S>,
) -> Result<BundleRequest> {
    let mut nounce = client.get_transaction_count(client.address(), None).await?;

    let mut tx: TypedTransaction = TransactionRequest::pay("vitalik.eth", 100).into();
    let bundle = BundleRequest::new();
    // creation bundle with multiple transaction to handle the gas spent in a bundle > 42000
    let bundle = {
        tx.set_nonce(nounce);
        client.fill_transaction(&mut tx, None).await?;
        nounce = nounce + 1;
        let signature = client.signer().sign_transaction(&tx).await?;
        let inner = bundle.push_transaction(tx.rlp_signed(client.signer().chain_id(), &signature));
        inner
    };
    let bundle = {
        tx.set_nonce(nounce);
        client.fill_transaction(&mut tx, None).await?;
        let signature = client.signer().sign_transaction(&tx).await?;
        let inner = bundle.push_transaction(tx.rlp_signed(client.signer().chain_id(), &signature));
        inner
    };
    Ok(bundle)
}
/* 
#[test]
pub fn test() {
    // Connect to the network
    let provider = Provider::<Http>::try_from("https://mainnet.eth.aragon.network").unwrap();

    // This is your searcher identity
    let bundle_signer = LocalWallet::new(&mut thread_rng());
    // This signs transactions
    let wallet = LocalWallet::new(&mut thread_rng());

    // Add signer and Flashbots middleware
    let client = SignerMiddleware::new(
        FlashbotsMiddleware::new(
            provider,
            Url::parse("https://relay.flashbots.net").unwrap(),
            bundle_signer,
        ),
        wallet,
    );

    // Pay Vitalik using a Flashbots bundle!
    let tx = TransactionRequest::pay("vitalik.eth", 100);
    let pending_tx = client.send_transaction(tx, None).await.unwrap();

    // Get the receipt
    let receipt = pending_tx
        .await
        .unwrap()
        .ok_or_else(|| anyhow::format_err!("tx not included"))
        .unwrap();
    let tx = client
        .get_transaction(receipt.transaction_hash)
        .await
        .unwrap();

    println!(
        "Sent transaction: {}\n",
        serde_json::to_string(&tx).unwrap()
    );
    println!("Receipt: {}\n", serde_json::to_string(&receipt).unwrap());
}
*/
