use std::process::Command;

pub fn route_cfmms(cfmms: &Vec<String>) -> Vec<String> {
    let mut cmd = Command::new("Julia");
    cmd.current_dir("c:/users/chhighma/source/repos/CFMMRouter.jl/examples");
    cmd.arg("arbitrage.jl");
    cmd.args(cfmms);
    dbg!("cmd: {#:?}", &cmd);
    match cmd.output() {
        Ok(o) => unsafe {
            let out = String::from_utf8_unchecked(o.stdout);
            println!("Str: {}", out);
            let out_vec: Vec<String> = out.split("|").map(|s| s.to_string()).collect();

            dbg!("Out: {#:?}", out_vec.clone());
            return out_vec;
        },
        Err(e) => {
            println!("There was an error {}", e);
        }
    }
    return Default::default();
}

#[test]
pub fn test_route() {
    let cfmm1 = "4,3,1,1,2".to_string();
    let cfmm2 = "4,4,1,2,3".to_string();
    let cfmm3 = "1,3,1,1,3".to_string();
    let cfmms = vec![cfmm1, cfmm2, cfmm3];

    route_cfmms(&cfmms);
}
