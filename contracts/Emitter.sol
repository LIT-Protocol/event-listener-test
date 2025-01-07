// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

// Uncomment this line to use console.log
// import "hardhat/console.sol";

contract Emitter {
    event SomethingHappened(uint256 indexed id);

    function emitEvent(uint256 id) public {
        emit SomethingHappened(id);
    }
}
