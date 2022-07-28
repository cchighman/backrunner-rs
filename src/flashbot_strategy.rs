use super::uniswap_providers::*;
use anyhow;
use anyhow::Result;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::core::types::transaction::eip2718::TypedTransaction;
use ethers::prelude::*;
use ethers::prelude::{Signer, SignerMiddleware, U256};
use ethers_flashbots::PendingBundleError;
use ethers_flashbots::{BundleRequest, FlashbotsMiddleware};
use std::sync::Arc;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use std::str::FromStr;

use url::Url;
pub mod utils {
    use super::*;
    pub static timestamp_seed: u128 = 30000_u128;

    pub fn valid_timestamp() -> U256 {
        let start = SystemTime::now();
        let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();
        let time_millis = since_epoch.as_millis().checked_add(timestamp_seed).unwrap();
        U256::from(time_millis)
    }

    /// Return a new flashbots bundle request for this block
    pub async fn new_bundle_request() -> Result<BundleRequest> {
        let block = mainnet::flashbots_client.get_block_number().await?;
        let mut bundle = BundleRequest::new();
        bundle = bundle.set_simulation_block(block);
        bundle = bundle.set_block(block + 1);
        let now = SystemTime::now();
        bundle = bundle.set_simulation_timestamp(now.duration_since(UNIX_EPOCH)?.as_secs());
        Ok(bundle)
    }

    pub async fn send_flashswap_bundle(txs: Vec<TypedTransaction>) -> Result<H256, anyhow::Error> {

        let nonce = mainnet::flashbots_client
            .get_transaction_count(
                mainnet::wallet.address(),
                Some(BlockId::from(BlockNumber::Latest)),
            )
            .await?;

        let block_number = mainnet::flashbots_client
            .inner()
            .inner()
            .get_block_number()
            .await
            .unwrap();
        //println!("Block Number: {}", block_number);

        let mut bundle = new_bundle_request().await?;

        for mut tx in txs {
            if tx.nonce().is_none() {
                tx.set_nonce(nonce);
                tx.set_gas_price(U256::from(300000000000u64));
                tx.set_gas(U256::from(400000));
            }

            let signature = mainnet::flashbots_client
                .signer()
                .sign_transaction(&tx)
                .await?;
                /*
            bundle = bundle.push_transaction(
                tx.rlp_signed(mainnet::flashbots_client.signer().chain_id(), &signature),
            );
             */
        }

        // Simulate it
        let simulated_bundle = mainnet::flashbots_client
            .inner()
            .simulate_bundle(&bundle)
            .await?;
        println!("Simulated bundle: {:?}", simulated_bundle);

        // Send it
        let pending_bundle = mainnet::flashbots_client
            .inner()
            .send_bundle(&bundle)
            .await?;

        match pending_bundle.await {
            Ok(bundle_hash) => {
                println!(
                    "Bundle with hash {:?} was included in target block",
                    bundle_hash
                );
                Ok(bundle_hash)
            }
            Err(PendingBundleError::BundleNotIncluded) => {
                println!("Bundle was not included in target block.");
                Err(anyhow::anyhow!("Bundle not included"))
            }
            Err(e) => Err(anyhow::anyhow!("PendingBundleError occured: {:#}", e)),
        }
    }
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
                services.MAINNET_MIDDLEWARE.clone(),
                Url::parse("https://relay.flashbots.net").unwrap(),
                services.MAINNET_BUNDLE_SIGNER.clone(),
            ),
            services.MAINNET_BOT_SIGNER.clone()
        );

        let block_number = client.inner().inner().block_number().await?;
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

/*
let bundle = bundle_for_test(&client).await?;
let current_block_number = client.inner().inner().block_number().await?;
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
    let bundle = bundle_for_test(&client).await?;
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
    let block_number = client.block_number().await?;

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

async fn bundle_for_test<M: 'static + Middleware, S: 'static + Signer>(
    client: &SignerMiddleware<M, S>,
) -> Result<BundleRequest> {
    let mut nounce = client.transaction_count(client.address(), None).await?;

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
        .transaction(receipt.transaction_hash)
        .await
        .unwrap();

    println!(
        "Sent transaction: {}\n",
        serde_json::to_string(&tx).unwrap()
    );
    println!("Receipt: {}\n", serde_json::to_string(&receipt).unwrap());
}
 */
