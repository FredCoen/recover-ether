// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract RetrieveEther {
    address payable public immutable owner;

    constructor(address _owner) {
        owner = payable(_owner);
    }

    function withdraw() public {
        require(msg.sender == owner);
        owner.transfer(address(this).balance);
    }
}

contract Deployer {
    event Deploy(address addr);

    function deploy(uint256 salt, address owner) external returns (address) {
        RetrieveEther _addr = new RetrieveEther{
            salt: keccak256(abi.encode(salt))
        }(owner);
        emit Deploy(address(_addr));
        return address(_addr);
    }
}
