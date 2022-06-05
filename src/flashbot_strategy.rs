use std::sync::Arc;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use anyhow;
use anyhow::Result;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::core::types::transaction::eip2718::TypedTransaction;
use ethers::prelude::*;
use ethers::prelude::{Signer, SignerMiddleware, U256};
use ethers_flashbots::{BundleRequest, FlashbotsMiddleware};
use ethers_flashbots::PendingBundleError;
use lazy_static::__Deref;
use crate::uniswap_providers::UNISWAP_PROVIDERS;
use rand::thread_rng;

use std::str::FromStr;

use url::Url;

pub static FLASH_STRATEGY: Lazy<Arc<FlashbotStrategy>> =
Lazy::new(|| {
    Arc::new(FlashbotStrategy::new())});

#[derive(Clone)]
pub struct FlashbotStrategy;

impl FlashbotStrategy {
    pub fn new() -> Self {
        Self {
            
        }
    }
    pub fn get_valid_timestamp() -> U256 {
        let start = SystemTime::now();
        let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();
        let time_millis = since_epoch.as_millis().checked_add(*&UNISWAP_PROVIDERS.TIMESTAMP_SEED).unwrap();
        return U256::from(time_millis);
    }

     /// Return a new flashbots bundle request for this block
     async fn new_bundle_request() -> Result<BundleRequest> {
        let provider = Provider::<Http>::try_from(
            "https://mainnet.infura.io/v3/20ca45667c5d4fa6b259b9a36babe5c3",
        ).unwrap();

        let private_key = "7005b56052be4776bffe00ff781879c65aa87ac3d5f8945c0452f27e11fa9236";

        let bundle_signer = private_key.parse::<LocalWallet>().unwrap();
        let wallet = private_key.parse::<LocalWallet>().unwrap();

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
    
        let block = client.get_block_number().await?;
        let mut bundle = BundleRequest::new();
        bundle = bundle.set_simulation_block(block);
        bundle = bundle.set_block(block + 1);
        let now = SystemTime::now();
        bundle = bundle.set_simulation_timestamp(now.duration_since(UNIX_EPOCH)?.as_secs());
        Ok(bundle)
    }

/* //  Mainnet
// https://mainnet.infura.io/v3/20ca45667c5d4fa6b259b9a36babe5c3
// wss://mainnet.infura.io/ws/v3/20ca45667c5d4fa6b259b9a36babe5c3

// Goerli
// https://goerli.infura.io/v3/0ab0b9c9d5bf44818399aea45b5ade51
// wss://goerli.infura.io/ws/v3/0ab0b9c9d5bf44818399aea45b5ade51
/* */
    pub async fn do_flashbot_goerli(tx: &mut TypedTransaction) -> Result<()> {
        // Connect to the network - using URL used by metamask
        let provider =
            Provider::<Http>::try_from("https://goerli.infura.io/v3/0ab0b9c9d5bf44818399aea45b5ade51")?;

        let private_key = "7005b56052be4776bffe00ff781879c65aa87ac3d5f8945c0452f27e11fa9236";
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

      let x: SignerMiddleware<FlashbotsMiddleware<Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>, Wallet<SigningKey>>, Wallet<SigningKey>> =   SignerMiddleware::new(
            FlashbotsMiddleware::new(
                UNISWAP_PROVIDERS.MAINNET_MIDDLEWARE.clone(),
                Url::parse("https://relay.flashbots.net").unwrap(),
                UNISWAP_PROVIDERS.MAINNET_BUNDLE_SIGNER.clone(),
            ),
            UNISWAP_PROVIDERS.MAINNET_BOT_SIGNER.clone()
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
*/

    pub async fn do_flashbot_mainnet(mut tx: TypedTransaction) -> Result<()> {
        println!("do_flashbot_mainnet");
        // Connect to the network - using URL used by metamask
        let provider = Provider::<Http>::try_from(
            "https://mainnet.infura.io/v3/20ca45667c5d4fa6b259b9a36babe5c3",
        ).unwrap();

        let private_key = "7005b56052be4776bffe00ff781879c65aa87ac3d5f8945c0452f27e11fa9236";

        let bundle_signer = private_key.parse::<LocalWallet>().unwrap();
        let wallet = private_key.parse::<LocalWallet>().unwrap();

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
    
        let mut nonce = client.get_transaction_count(client.address(), Some(BlockId::from(BlockNumber::Latest))).await?;
    
        let block_number = client.inner().inner().get_block_number().await.unwrap();
        println!("Block Number: {}", block_number);
        tx.set_nonce(nonce + U256::from(1));
        tx.set_gas_price(U256::from(300000000000u64));
        tx.set_gas(U256::from(400000));
        
        let signature = client.signer().sign_transaction(&tx).await.unwrap();
        let bundle = FlashbotStrategy::new_bundle_request().await?
    .push_transaction(tx.rlp_signed(client.signer().chain_id(), &signature));
             
        // Simulate it
        let simulated_bundle = client.inner().simulate_bundle(&bundle).await.unwrap();

        println!("Simulated bundle: {:?}", simulated_bundle);

        // Send it
        let pending_bundle = client.inner().send_bundle(&bundle).await.unwrap();

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
for x in 0..10 
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
