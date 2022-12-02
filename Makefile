
# Makefile
include .env
CALLDATA := $(word 2, $(MAKECMDGOALS))
ADDRESS := $(word 2, $(MAKECMDGOALS))
OWNER_ADDRESS := $(word 3, $(MAKECMDGOALS))
PRIVATE_KEY := $(PRIVATE_KEY)
RPC_URL := $(RPC_URL)
ETHERSCAN_API_KEY:=$(ETHERSCAN_API_KEY)

deploy-deployer:
	@forge script ./script/Deploy.s.sol:DeployDeployer -vvvvv --rpc-url $(RPC_URL) --private-key $(PRIVATE_KEY) --broadcast

deploy-retriever:
	@forge script ./script/Deploy.s.sol:DeployRetrieveEther --sig $(CALLDATA) -vvvvv --rpc-url $(RPC_URL) --private-key $(PRIVATE_KEY)

verify-deployer:
	@forge verify-contract --chain-id 5 --num-of-optimizations 200 --watch \
    --compiler-version v0.8.13 $(ADDRESS) src/RetrieveEther.sol:Deployer $(ETHERSCAN_API_KEY)

verify-retriever:
	@forge verify-contract --chain-id 1 --num-of-optimizations 200 --watch --constructor-args \ 
    $(cast abi-encode "constructor(address)" $(OWNER_ADDRESS)) \
    --compiler-version v0.8.13 $(ADDRESS) src/RetrieveEther.sol:Deployer $(ETHERSCAN_API_KEY)

