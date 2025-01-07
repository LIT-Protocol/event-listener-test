import { ethers } from "hardhat";

async function main() {
  // deploy the emitter contract
  const contract = await ethers.getContractAt(
    "Emitter",
    "0xEe1033c70701fe0ff133436AdD566c1877728e2b"
  );

  // listen to the contract
  contract.on("SomethingHappened", (id: number) => {
    console.log("Something happened", id);
  });
  console.log("listening");

  // emit an event
  await contract.emitEvent(1);
}

main();
