## Ether Recovery

This utility repo is an attempt to recover ether send to a wrong none existent address. By brute forcing a salt and leveraging CREATE2 opcode this repo attempts to deploy a contract with which you can recover the ether sent to the wrong address.

### Usage

Set your `.env` file according to `.env.example`

Run:

```
set -a && source .env
```

Deploy the Deployer contract:

```
make deploy-deployer
```

From the command above, take the printed Deployer contract address as an input to running the salt brute forcing script below.

Brute force salt by running:

```
cargo run <target_address> <deployer_address> <owner_address>
```

_Note:_ <target_address> in this context refers to the address from which you want to recover ether.

_Note:_ <owner_address> refers to the owner address of the RetrieveEther contract. An owner has to be set to prevent anyone from retrieving the recovered ether at the target address.

If the salt is found, generate the calldata required to deploy the RetrieveEther contract to the target address.

Run:

```
cargo run --bin generate_calldata <owner_address> <salt>
```

_Note:_ An EVM address is 20 bytes long, so it can take up to 2^160 attempts to brute force the salt to get the target address. This is probably not feasible.

Finally deploy RetrieveEther with the generated calldata:

```
make deploy-retriever <calldata>
```

You can now simply call withdraw() method on the RetrieveEther contract from the owner address you used above.
