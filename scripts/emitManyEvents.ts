import { ethers } from "hardhat";

async function main() {
  // deploy the emitter contract
  const contract = await ethers.getContractAt(
    "Emitter",
    "0xEe1033c70701fe0ff133436AdD566c1877728e2b"
  );

  // emit 100 events
  for (let i = 0; i < 100; i++) {
    console.log(`Emitting event ${i}`);
    await contract.emitEvent(i);
  }
  process.exit(0);
}

main();
