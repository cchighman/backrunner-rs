use ethers::core::types::transaction::eip2718::TypedTransaction;

/*
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
