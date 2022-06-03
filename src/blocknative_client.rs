#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types, dead_code)]
#![allow(non_snake_case, unused_imports, unused_results)]
#![allow(
    unused_doc_comments,
    unused_variables,
    unused_assignments,
    unused_must_use,
    unused_mut
)]

use std::collections::HashMap;

use futures_util::StreamExt;

use blocknative::{
    models::Blockchain,
    ws::{models::WatchConfig, ws::Ws},
};
#[test]
pub fn blocknative_initialize() {
    println!("Connecting to blocknative..");
    let ws = Ws::connect(
        "wss://api.blocknative.com/v0",
        "3b21bf22-0a3e-4908-9b2f-c9ac37c31b7b",
        Blockchain::main(),
    )
    .await
    .unwrap();

    let mut filters = HashMap::new();

    //filters.insert("contractCall.methodName".to_string(), "swap".to_string());

    let config = WatchConfig {
        scope: "0xe93527D1F8c586353b13826C501fa5a69bCE2b0E".to_string(),
        filters: vec![filters],
        watch_address: true,
    };
    println!(
        "Subscribing to filter on: {:?}",
        "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D"
    );

    let mut stream = ws.listen(config).await.unwrap();
    println!("Waiting for events..");

    while let Some(response) = stream.next().await {
        if let Some(event) = response.event {
            println!("RECEIVED - {:?}", event);
            if let Some(cc) = event.contract_call {
                println!("RECEIVED - {:?}", cc);
            }
        }
        // break;
    }
}
