extern crate petgraph;

use arbitrage_path::ArbitragePath;
use bigdecimal::BigDecimal;
use crypto_pair::CryptoPair;
use dashmap::DashMap;
use utils::uniswapv2_utils::{populate_sushiswap_pairs, populate_uniswapv2_pairs};
use utils::uniswapv3_utils::populate_uniswapv3_pools;

use crate::arbitrage_paths::ArbitragePaths;

pub mod arbitrage_path;
pub mod arbitrage_paths;
mod blocknative_client;
pub mod crypto_pair;
pub mod dex_pool;
pub mod graphql_uniswapv2;
pub mod graphql_uniswapv3;
pub mod uniswapv2_pairs;
pub mod uniswapv3_pools;
pub mod utils;

#[allow(dead_code)]
#[async_std::main]
async fn main() {
    tracing_subscriber::fmt::init();
    /*
    TODO
    1.) Populate paths from GraphQL
    2.) Init dedicated listener for Pending/New Transactions
    3.) Flow:
         Event -> Invoke CryptoPair Update -> Evaluate Paths -> Generate Arbitrage
     */
    use jlrs::prelude::*;

    use std::time::Duration;
    // This struct contains the data our task will need. This struct must be `Send`, `Sync`, and
    // contain no borrowed data.
    struct MyTask {
        dims: isize,
        iters: isize,
    }

    // `MyTask` is a task we want to be executed, so we need to implement `AsyncTask`. This requires
    // `async_trait` because traits with async methods are not yet available in Rust. Because the
    // task itself is executed on a single thread, it is marked with `?Send`.
    #[async_trait(?Send)]
    impl AsyncTask for MyTask {
        // Different tasks can return different results. If successful, this task returns an `f64`.
        type Output = f64;

        // This is the async variation of the closure you provide `Julia::scope` when using the sync
        // runtime. The `Global` can be used to access `Module`s and other static data, while the
        // `AsyncGcFrame` lets you create new Julia values, call functions, and create nested scopes.
        async fn run<'base>(
            &mut self,
            global: Global<'base>,
            frame: &mut AsyncGcFrame<'base>,
        ) -> JlrsResult<Self::Output> {
            // Convert the two arguments to values Julia can work with.
            let dims = Value::new(&mut *frame, self.dims)?;
            let iters = Value::new(&mut *frame, self.iters)?;

            // Get `complexfunc` in `MyModule`, call it on another thread with `call_async`, and await
            // the result before casting it to an `f64` (which that function returns). A function that
            // is called with `call_async` is executed on another thread by calling
            // `Base.threads.@spawn`.
            // The module and function don't have to be rooted because the module is never redefined,
            // so they're globally rooted.
            unsafe {
                Module::main(global)
                    .submodule_ref("CFMMRouter")?
                    .wrapper_unchecked()
                    .function_ref("route!")?
                    .wrapper_unchecked()
                    .call_async(&mut *frame, &mut [dims, iters])
                    .await?
                    .into_jlrs_result()?
                    .unbox::<f64>()
            }
        }
    }

    let mut crypto_pairs: DashMap<String, Vec<CryptoPair>> = DashMap::new();
    let _arbitrage_paths: DashMap<String, ArbitragePath> = DashMap::new();

    /* 1.) Populate a map of all possible crypto pairs */
    //    populate_uniswapv2_pairs(&mut crypto_pairs);
    // populate_uniswapv3_pools(&mut crypto_pairs);
    // populate_sushiswap_pairs(&mut crypto_pairs);

    /* 2) Load some source to populate and init arb paths */
    /* 3.) Begin listening to pending / completed tx */

    let (julia, handle) = unsafe {
        AsyncJulia::init_async(4, 16, Duration::from_millis(1))
            .await
            .expect("Could not init Julia")
    };

    // Create channels for each of the tasks (this is not required but helps distinguish which
    // result belongs to which task).
    let (sender1, receiver1) = async_std::channel::bounded(1);
    let (sender2, receiver2) = async_std::channel::bounded(1);
    let (sender3, receiver3) = async_std::channel::bounded(1);
    let (sender4, receiver4) = async_std::channel::bounded(1);

    // Send four tasks to the runtime.
    julia
        .task(
            MyTask {
                dims: 4,
                iters: 100_000_000,
            },
            sender1,
        )
        .await;

    julia
        .task(
            MyTask {
                dims: 4,
                iters: 200_000_000,
            },
            sender2,
        )
        .await;

    julia
        .task(
            MyTask {
                dims: 4,
                iters: 300_000_000,
            },
            sender3,
        )
        .await;

    julia
        .task(
            MyTask {
                dims: 4,
                iters: 400_000_000,
            },
            sender4,
        )
        .await;

    // Receive the results of the tasks.
    let res1 = receiver1.recv().await.unwrap().unwrap();
    println!("Result of first task: {:?}", res1);
    let res2 = receiver2.recv().await.unwrap().unwrap();
    println!("Result of second task: {:?}", res2);
    let res3 = receiver3.recv().await.unwrap().unwrap();
    println!("Result of third task: {:?}", res3);
    let res4 = receiver4.recv().await.unwrap().unwrap();
    println!("Result of fourth task: {:?}", res4);

    // Dropping `julia` causes the runtime to shut down Julia and itself if it was the final
    // handle to the runtime.
    std::mem::drop(julia);
    handle.await.expect("Julia exited with an error");

    let paths = ArbitragePaths::new();
    //   paths.generate_arbitrage_paths(&crypto_pairs);
    //println!("{:#?}
}
