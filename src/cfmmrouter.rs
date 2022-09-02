use base64::encode;
use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;
use crossbeam::channel::{bounded, unbounded};
use crossbeam::thread::scope;
use ethereum_types::U256;
use once_cell::sync::Lazy;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::sync::Mutex;
use std::time::Duration;
use std::collections::{HashSet};

use std::thread;

extern "C" {
    pub fn init_julia(argc: i32, argv: *const *const c_char);
    pub fn shutdown_julia(retcode: i32);
    pub fn route(str: *const *const c_char) -> *const c_char;
}

pub static cfmm_tx: Lazy<(Sender<String>, Receiver<String>)> = Lazy::new(|| bounded(0));
pub static cfmm_rx: Lazy<(
    Sender<Result<String, anyhow::Error>>,
    Receiver<Result<String, anyhow::Error>>,
)> = Lazy::new(|| bounded(0));
pub static send_mutex: Lazy<Mutex<&Sender<String>>> = Lazy::new(|| Mutex::new(&cfmm_tx.0));

pub fn optimal_route(routes: String) -> Result<String, anyhow::Error> {
    send_mutex.lock().unwrap();
    cfmm_tx_send().send(routes).unwrap();
    cfmm_rx_recv().recv().unwrap()
}

pub fn cfmm_tx_recv() -> &'static Receiver<String> {
    return &cfmm_tx.1;
}

pub fn cfmm_tx_send() -> &'static Sender<String> {
    return &cfmm_tx.0;
}

pub fn cfmm_rx_recv() -> &'static Receiver<Result<String, anyhow::Error>> {
    return &cfmm_rx.1;
}

pub fn cfmm_rx_send() -> &'static Sender<Result<String, anyhow::Error>> {
    return &cfmm_rx.0;
}

pub fn init() {
    unsafe {
        init_julia(0, &vec![].as_ptr());
    }
    loop {
        thread::sleep(Duration::from_secs(1));
        let r = cfmm_tx_recv().recv().unwrap();
        let resp = optimal_route_impl(r);
        cfmm_rx_send().send(resp).unwrap();
    }
}

pub fn optimal_route_impl(routes: String) -> Result<String, anyhow::Error> {
    unsafe {
        let c_str = CString::new(routes)?;
        let c_str_ptr = c_str.as_ptr() as *const *const i8;
        let paths = route(c_str_ptr);
        let rust_c_str = CStr::from_ptr(paths);
        let rust_str = rust_c_str.to_str()?;
        let rust_string = String::from(rust_str);

        return Ok(rust_string);
    }
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub(crate) struct OptimalRoute {
    pub(crate) routes: Vec<Vec<HashMap<String,U256>>>
}

#[test]
fn test_optimal_route() {
    /*
        [
            amt_in, src_coin, dest_coin, fee, reserve1, reserve2, type
            [0,193031404173640516668,14420532476509322142170146,1,2,1,0],
            [0,2624508734533931152523647,821209347455760118751496,2,1,2,0],
            [0,66964897265466266339,15812993334687703134711152,3,2,1,0]
        ]


        {
            cfmm: {
                received: {
                    coin: amt
                },
                tendered: {
                    coin: amt
                }
            }
        }
    HashMap<String, HashMap<String, HashMap<String, Float64>>>
        {
          "1": {
            "received": {
              "2": 1.432914950819128e+25
            },
            "tendered": {
              "1": 3.0267958035174186e+22
            }
          },
          "2": {
            "received": {
              "2": 1.5864164685771538e+24
            },
            "tendered": {
              "3": 6.274875922288971e+23
            }
          },
          "3": {
            "received": {
              "1": 1.580214634086905e+25
            }
            "tendered": {
              "3": 3.251865982353407e+22
            }
          }
        }

        [
            amt_in, src_coin, dest_coin, fee, reserve1, reserve2, type
            [0,193031404173640516668,14420532476509322142170146,1,2,1,0],
            [0,2624508734533931152523647,821209347455760118751496,2,1,2,0],
            [0,66964897265466266339,15812993334687703134711152,3,2,1,0]
        ]

        routes_json: "{\"2\":{\"received\":{\"2\":1156424573023238584532992},\"tendered\":{\"3\":646874814054932190068736}},\"3\":{\"received\":{\"1\":15794205790284410806861824},\"tendered\":{\"3\":56295668312606800609280}},\"1\":{\"received\":{\"2\":14329149508191279309127680},\"tendered\":{\"1\":30267958035174186287104}}}"

        routes_map: {"2": {"received": {"2": 1.1564245730232386e24}, "tendered": {"3": 6.468748140549322e23}}, "1": {"received": {"2": 1.432914950819128e25}, "tendered": {"1": 3.0267958035174186e22}}, "3": {"tendered": {"3": 5.62956683126068e22}, "received": {"1": 1.579420579028441e25}}}

        */

    unsafe {
        init_julia(0, &vec![].as_ptr());
    }

    let mut route_vec: Vec<Vec<U256>> = Vec::default();
    route_vec.push(vec![
        U256::zero(),
        U256::from_dec_str("193031404173640516668").unwrap(),
        U256::from_dec_str("14420532476509322142170146").unwrap(),
        U256::from(1),
        U256::from(1),
        U256::from(2),
        U256::zero(),
    ]);
    route_vec.push(vec![
        U256::zero(),
        U256::from_dec_str("2624508734533931152523647").unwrap(),
        U256::from_dec_str("821209347455760118751496").unwrap(),
        U256::from(1),
        U256::from(2),
        U256::from(3),
        U256::zero(),
    ]);
    route_vec.push(vec![
        U256::zero(),
        U256::from_dec_str("66964897265466266339").unwrap(),
        U256::from_dec_str("15812993334687703134711152").unwrap(),
        U256::from(1),
        U256::from(3),
        U256::from(1),
        U256::zero(),
    ]);

    let route_vec2 = [
        [
            "0x0",
            "0x30133821532eebdbf",
            "0x4cb95190bed3e",
            "0x1",
            "0x2",
            "0x1",
            "0x0",
        ],
        [
            "0x0",
            "0xe7791ded7c2",
            "0x20f6a79afb3361f23d1",
            "0x1",
            "0x3",
            "0x2",
            "0x0",
        ],
        [
            "0x0",
            "0x573f1c117f4e4",
            "0xee7723148f",
            "0x1",
            "0x1",
            "0x3",
            "0x0",
        ],
    ];

    println!(
        "route_vec: {:?}\n\nencoded: {:?}",
        &route_vec2,
        encode(serde_json::to_string(&route_vec2).unwrap())
    );

    let routes_json = optimal_route(encode(serde_json::to_string(&route_vec2).unwrap())).unwrap();
    let routes_map: HashMap<String, HashMap<String, HashMap<String, String>>> =
        serde_json::from_str(&routes_json).unwrap();
    println!(
        "routes_json: {:?}\n\nroutes_map: {:?}\n\n",
        &routes_json, &routes_map
    );

    unsafe {
        shutdown_julia(0);
    }
}

#[test]
fn test_cross() {
    let (s, r) = bounded(0);
    let (s2, r2) = bounded(0);
    let send_lock = Mutex::new(&s);

    scope(|scope| {
        // Spawn a thread that receives a message and then sends one.
        scope.spawn(|_| {
            println!("Spawned Receiver");
            thread::sleep(Duration::from_secs(1));
            loop {
                let r = r.recv().unwrap();
                println!("Received: {:?}", r);
                s2.send(r).unwrap();
            }
        });

        scope.spawn(|_| {
            send_lock.lock().unwrap();

            println!("Spawned Sender1");
            s.send(1).unwrap();

            let resp = r2.recv().unwrap();
            println!("Sender1 Complete: {}", resp);
        });

        scope.spawn(|_| {
            send_lock.lock().unwrap();

            println!("Spawned Sender2");
            s.send(2).unwrap();

            let resp = r2.recv().unwrap();
            println!("Sender2 Complete: {}", resp);
        });
    })
    .unwrap();

    loop {
        thread::sleep(Duration::from_secs(2));
    }
}
