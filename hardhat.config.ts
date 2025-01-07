import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";

const config: HardhatUserConfig = {
  solidity: "0.8.28",
  networks: {
    yellowstone: {
      url: "https://yellowstone-rpc.litprotocol.com",
      ...(process.env["LIT_ROLLUP_MAINNET_DEPLOYER_PRIVATE_KEY"] && {
        accounts: [process.env["LIT_ROLLUP_MAINNET_DEPLOYER_PRIVATE_KEY"]],
      }),
      chainId: 175188,
      // @ts-ignore
      stylusContractsForTests: {
        p256:
          process.env.LIT_STYLUS_P256_CONTRACT_ADDRESS ||
          "0x8ea150155c63b3a2e34b61409fb65e19f1bd48e7",
        k256:
          process.env.LIT_STYLUS_K256_CONTRACT_ADDRESS ||
          "0x28ca4b9b360ed4f918081c921b8a299fd491e96a",
      },
      wlitAddress: "0xd78089bAAe410f5d0eae31D0D56157c73a3Ff98B",
    },
  },
};

export default config;
