use crate::arb_thread_pool::spawn;
use crate::call_julia::route_cfmms;
use crate::contracts::bindings::uniswap_v2_pair::UniswapV2Pair;
use crate::crypto_math::*;
use crate::crypto_pair::CryptoPair;
use crate::swap_route::route_calldata;
use crate::swap_route::{SwapRoute, TO_ADDRESS};
use crate::uniswap_providers::ROPSTEN_PROVIDER;

use bigdecimal::BigDecimal;
use ethabi::Token;
use ethereum_types::{Address, U256, U512};
use ethers::prelude::Bytes;
use ethers::prelude::*;
use future::ready;

use futures::{executor, future, StreamExt};
use futures_signals::internal::{Map2, MapPair};
use futures_signals::signal::{Mutable, MutableSignal};
use futures_signals::{
    map_mut, map_ref,
    signal::{Signal, SignalExt},
};

use futures_util::Future;
use num_traits::{FromPrimitive, ToPrimitive};
use rayon::prelude::*;
use std::sync::Arc;

/*


pub fn arb_signal_3(pairs: &Vec<Arc<CryptoPair>>) {


        map_ref! {
               let A0 = pairs[0].left_reserves_signal(),
               let A1 = pairs[0].right_reserves_signal(),

               let B0 = pairs[1].left_reserves_signal(),
               let B1 = pairs[1].right_reserves_signal(),

               let C0 = pairs[2].left_reserves_signal(),
               let C1 = pairs[2].right_reserves_signal()
          =>
               *A0 / *A1 / *B0 / *B1 / *C0 / *C1
        }

}


pub fn arb_signal_2(pairs: &Vec<Arc<CryptoPair>>) {
    let r = map_ref!! {
                let A0 = pairs[0].left_reserves_signal(),
                let A1 = pairs[0].right_reserves_signal(),

                let B0 = pairs[1].left_reserves_signal(),
                let B1 = pairs[1].right_reserves_signal()
           =>
                *A0 / *A1 / *B0 / *B1
        };
    let future = r.for_each(|v| ready(()));
    // spawn(future);
}

pub fn arb_signal_4(pairs: &Vec<Arc<CryptoPair>>) -> MutableSignal<U256> {
   let r =  map_mut! {
            let A0 = pairs[0].left_reserves_signal(),
            let A1 = pairs[0].right_reserves_signal(),

            let B0 = pairs[1].left_reserves_signal(),
            let B1 = pairs[1].right_reserves_signal(),

            let C0 = pairs[2].left_reserves_signal(),
            let C1 = pairs[2].right_reserves_signal(),

            let D0 = pairs[3].left_reserves_signal(),
            let D1 = pairs[3].right_reserves_signal()
       =>
            *A0 / *A1 / *B0 / *B1 / *C0 / *C1 / *D0 / *D1
    }
}
pub fn arb_signal_5(pairs: &Vec<Arc<CryptoPair>>) -> MutableSignal<U256> {
    map_ref! {
            let A0 = pairs[0].left_reserves_signal(),
            let A1 = pairs[0].right_reserves_signal(),

            let B0 = pairs[1].left_reserves_signal(),
            let B1 = pairs[1].right_reserves_signal(),

            let C0 = pairs[2].left_reserves_signal(),
            let C1 = pairs[2].right_reserves_signal(),

            let D0 = pairs[3].left_reserves_signal(),
            let D1 = pairs[3].right_reserves_signal(),

             let E0 = pairs[4].left_reserves_signal(),
              let E1 = pairs[4].right_reserves_signal()
       =>
            *A0 / *A1 / *B0 / *B1 / *C0 / *C1 / *D0 / *D1 / *E0 / *E1
    }
}
pub fn arb_signal_6(pairs: &Vec<Arc<CryptoPair>>) -> MutableSignal<U256> {
    map_ref! {
            let A0 = pairs[0].left_reserves_signal(),
            let A1 = pairs[0].right_reserves_signal(),

            let B0 = pairs[1].left_reserves_signal(),
            let B1 = pairs[1].right_reserves_signal(),

            let C0 = pairs[2].left_reserves_signal(),
            let C1 = pairs[2].right_reserves_signal(),

            let D0 = pairs[3].left_reserves_signal(),
            let D1 = pairs[3].right_reserves_signal(),

             let E0 = pairs[4].left_reserves_signal(),
              let E1 = pairs[4].right_reserves_signal(),

              let F0 = pairs[5].left_reserves_signal(),
              let F1 = pairs[5].right_reserves_signal()
                           =>
            *A0 / *A1 / *B0 / *B1 / *C0 / *C1 / *D0 / *D1 / *E0 / *E1 / *F0 / *F1
    }
}



pub fn arb_index_signal(pairs: &Vec<Arc<CryptoPair>>) {

    if pairs.len() == 2 {
        arb_signal_2(pairs);
    } else if pairs.len() == 3 {
        arb_signal_3(pairs);
    } else if pairs.len() == 4 {
        arb_signal_4(pairs);
    }



            4 => {
                let r = map_ref! {
                let A0 = pairs[0].left_reserves_signal(),
                let A1 = pairs[0].right_reserves_signal(),

                let B0 = pairs[1].left_reserves_signal(),
                let B1 = pairs[1].right_reserves_signal(),

                let C0 = pairs[2].left_reserves_signal(),
                let C1 = pairs[2].right_reserves_signal(),

                let D0 = pairs[3].left_reserves_signal(),
                let D1 = pairs[3].right_reserves_signal()
                =>
                 *A0 / *A1; / *B0 / *B1 / *C0 / *C1 / *D0 / *D1
                   };
                let future = r.for_each(move |v| ready(()));
                spawn(future);
            }
            5 => {
                let r = map_ref! {
                let A0 = pairs[0].left_reserves_signal(),
                let A1 = pairs[0].right_reserves_signal(),

                let B0 = pairs[1].left_reserves_signal(),
                let B1 = pairs[1].right_reserves_signal(),

                let C0 = pairs[2].left_reserves_signal(),
                let C1 = pairs[2].right_reserves_signal(),

                let D0 = pairs[3].left_reserves_signal(),
                let D1 = pairs[3].right_reserves_signal(),

                let E0 = pairs[4].left_reserves_signal(),
                let E1 = pairs[4].right_reserves_signal()
                =>
                 *A0 / *A1 / *B0 / *B1 / *C0 / *C1 / *D0 / *D1 / *E0 / *E1
                 };
                let future = r.for_each(move |v| ready(()));
                spawn(future);
            }
            6 => {
                let r = map_ref! {
                let A0 = pairs[0].left_reserves_signal(),
                let A1 = pairs[0].right_reserves_signal(),

                let B0 = pairs[1].left_reserves_signal(),
                let B1 = pairs[1].right_reserves_signal(),

                let C0 = pairs[2].left_reserves_signal(),
                let C1 = pairs[2].right_reserves_signal(),

                let D0 = pairs[3].left_reserves_signal(),
                let D1 = pairs[3].right_reserves_signal(),

                let E0 = pairs[4].left_reserves_signal(),
                let E1 = pairs[4].right_reserves_signal(),

                let F0 = pairs[5].left_reserves_signal(),
                let F1 = pairs[5].right_reserves_signal()
                =>
                 *A0 / *A1 / *B0 / *B1 / *C0 / *C1 / *D0 / *D1 / *E0 / *E1 / *F0 / *F1
                    };
                let future = r.for_each(move |v| ready(()));
                spawn(future);
            }
let C0 = self.get_arb_signal_from_index(2, SIDE::Left),
let C1 =  self.get_arb_signal_from_index(2, SIDE::Right),

let D0 = self.get_arb_signal_from_index(3, SIDE::Left),
let D1 =  self.get_arb_signal_from_index(3, SIDE::Right),

let E0 = self.get_arb_signal_from_index(4, SIDE::Left),
let E1 =  self.get_arb_signal_from_index(4, SIDE::Right),

let F0 = self.get_arb_signal_from_index(5, SIDE::Left),
let F1 =  self.get_arb_signal_from_index(5, SIDE::Right)

//       =>
//          (*A0) // / (*C0 / *C1) / (*D0 / *D1) / (*E0 / *E1) / (*F0 / *F1)


 */
