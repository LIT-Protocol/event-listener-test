import { ethers } from "hardhat";

async function main() {
  // deploy the emitter contract
  const Emitter = await ethers.getContractFactory("Emitter");
  const contract = await Emitter.deploy();

  // listen to the contract
  contract.on("SomethingHappened", (id: number) => {
    console.log("Something happened", id);
  });

  // emit an event
  await contract.emitEvent(1);

  console.log("listening");
}

main();
