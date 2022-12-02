// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import "../src/RetrieveEther.sol";
import "../lib/forge-std/src/console.sol";

contract DeployDeployer is Script {
    function setUp() public {}

    function run() public {
        vm.broadcast();

        address deployer = address(Deployer(new Deployer()));
        console2.log("Deployer deployed at:", deployer);
    }
}

contract DeployRetrieveEther is Script {
    function setUp() public {}

    function run(address deployer, uint256 salt) public {
        vm.broadcast();

        address retrieveEther = Deployer(deployer).deploy(salt, msg.sender);
        console2.log("RetrieveEther deployed at:", retrieveEther);
    }
}
