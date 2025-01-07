import { ethers } from "hardhat";

async function main() {
  // deploy the emitter contract
  const Emitter = await ethers.getContractFactory("Emitter");
  const contract = await Emitter.deploy();

  console.log("contract deployed", contract.target);
}

main();
