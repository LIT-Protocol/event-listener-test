# Event listener PoC / test

This is a proof of concept for an event listener using Rust and ethers.

It is a simple contract that emits events, and a Rust program that listens for those events.

The Rust program is a simple polling program that queries the contract for events in blocks.

The TypeScript program is a simple script that emits 100 events to the contract.

## Usage

To run the tests, make sure you have a Yellowstone private key in the `LIT_ROLLUP_MAINNET_DEPLOYER_PRIVATE_KEY` environment variable. The emitter contract is already deployed, but if you want to isolate the test to ensure nobody else is emitting events, you can deploy it yourself with the `scripts/deploy.ts.ts` script. Note that the contract address is hardcoded in a bunch of places so you'll need to change that as well.

To run the test where we emit 100 events and check that we received all of the in rust, run `./test.sh`. in the `rust-ethers` directory.
