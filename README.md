## Attempt to recover ether mistakenly sent to none existent address

This repo offers utilities to recover ether mistakenly sent to a none existent address. By brute forcing a salt and leveraging CREATE2 opcode a contract is deployed to the desired address. The contract contains a withdraw method with which you can recover the ether balance.

### Usage

Set your `.env` file according to `.env.example`

Run:

```
source .env
```

Deploy the Deployer contract by running:

```
make deploy-deployer
```

_Note:_ This will print the deployer contract address to which you need to pass as an input to the script below.

Brute force salt by running:

```
cargo run <target_address> <deployer_contract_address> <owner_address>
```

_Note:_ <target_address> in this context refers to the address from which you want to recover ether.

_Note:_ <owner_address> refers to the owner address of the RetrieveEther contract. An owner has to be set to prevent anyone from retrieving the recovered ether at the target address.

_Note:_ An EVM address is 20 bytes long, so it can take up to 2^160 attempts to brute force the salt to get the target address. This is not feasible. However 2^80 gives you a 50% chance of finding it. That is around one septillion times, or 1.2089258196146291747 x 10^24. I have not yet found out how long this would take to run, but running this in parallel on a mutli core machine wil definitely speed it up. Feel free to submit a PR !

If the salt is found, generate the calldata required to deploy the RetrieveEther contract to the target address.

Run:

```
cargo run --bin generate_calldata <owner_address> <salt>
```

Finally deploy RetrieveEther by running:

```
make deploy-retriever <calldata>
```

Call the withdraw() method on the RetrieveEther contract from the owner you have set above.
