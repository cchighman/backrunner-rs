//use ethers_flashbots::*;
/*
pub(crate) static GOERLI_WALLET: Lazy<Wallet<SigningKey>> = Lazy::new(|| {
    MnemonicBuilder::<English>::default()
        .phrase("unveil spoon stable govern diesel park glory visa lucky teach aspect spy")
        .index(0u32)
        .unwrap()
        .build()
        .unwrap()
        .with_chain_id(5_u64)
});

pub(crate) static FLASHBOTS_GOERLI_PROVIDER: Lazy<
    Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
> = Lazy::new(|| {
    Arc::new(SignerMiddleware::new(FlashbotsMiddleware::new(
        provider,
        Url::parse("https://relay-goerli.flashbots.net")?,
        *GOERLI_WALLET,
    )))
});

pub(crate) async fn execute_flashbot_strategy(tx: &TypedTransaction) {
    for x in 0..10 {
        let block_number = FLASHBOTS_GOERLI_PROVIDER.get_block_number().await?;

        let signature = GOERLI_WALLET.signer().sign_transaction(&tx).await?;
        let bundle = BundleRequest::new()
            .push_transaction(tx.rlp_signed(GOERLI_WALLET.signer().chain_id()))
            .set_block(block_number + x);

        /* Bundle Simulation */
        let simulated_bundle = FLASHBOTS_GOERLI_PROVIDER
            .inner()
            .simulate_bundle(&bundle)
            .await?;
        println!("Simulated bundle: {:?}", simulated_bundle);

        // Send it
        let pending_bundle = FLASHBOTS_GOERLI_PROVIDER
            .inner()
            .send_bundle(&bundle)
            .await?;
    }

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



 */
