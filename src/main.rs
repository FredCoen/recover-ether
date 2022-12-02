use ethers::abi::{ethereum_types, Token};
use ethers::prelude::*;
use ethers::utils::{get_create2_address, hex, keccak256};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1..];

    let target_address = query[0].to_string().parse::<Address>().unwrap();
    let mut iterator_count = 0;
    let mut count: u128 = 0;

    loop {
        let deployer_address = query[1].to_string().parse::<Address>().unwrap();

        let seed = abi::encode(&[Token::Uint(ethereum_types::U256::from_big_endian(
            &count.to_be_bytes(),
        ))]);

        let salt = keccak256(seed);
        let bytecode_in_hex = "60a060405234801561001057600080fd5b506040516101e13803806101e183398101604081905261002f91610040565b6001600160a01b0316608052610070565b60006020828403121561005257600080fd5b81516001600160a01b038116811461006957600080fd5b9392505050565b60805161014b61009660003960008181604a015281816093015260ca015261014b6000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c80633ccfd60b1461003b5780638da5cb5b14610045575b600080fd5b610043610088565b005b61006c7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b03909116815260200160405180910390f35b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146100bd57600080fd5b6040516001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016904780156108fc02916000818181858888f19350505050158015610112573d6000803e3d6000fd5b5056fea264697066735822122080660803ba6e606e3ddfe0d8dcd45ec7e06e79f0f75d73879d6989cace1509ec64736f6c634300080d0033000000000000000000000000";
        let bytecode_in_hex_with_wrgs = format!("{}{}", bytecode_in_hex, query[2]);
        let init_code = Bytes::from(hex::decode(bytecode_in_hex_with_wrgs).unwrap());

        let create2_addresss = get_create2_address(deployer_address, salt, init_code);
        println!("{}", create2_addresss);
        if count % 100_000 == 0 {
            iterator_count += 1;
            println!("Attempted salts: {}*100K", iterator_count);
        }

        if target_address == create2_addresss {
            println!("Brute forced salt: {}", count);
            break;
        }
        count += 1;
    }
}
