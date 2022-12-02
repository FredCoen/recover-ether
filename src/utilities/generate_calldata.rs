use std::env;

use ethers::{abi::parse_abi, prelude::*};

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1..];
    println!("{}", query[0]);
    let abi = BaseContract::from(
        parse_abi(&["function run(address deployer, uint256 salt) external"]).unwrap(),
    );
    let encoded = abi
        .encode(
            "run",
            (
                query[0].to_string().parse::<Address>().unwrap(),
                U256::from_dec_str(query[1].as_str()).unwrap(),
            ),
        )
        .unwrap()
        .to_string();

    println!("{}", encoded);
}
