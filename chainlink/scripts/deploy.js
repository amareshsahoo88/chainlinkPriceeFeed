
const hre = require("hardhat");

async function main() {
 const [deployer] = await ethers.getSigners();

 const ChainlinkPriceFeed = await ethers.getContractFactory("ChainlinkPriceFeed");
 const chainlinkPriceFeed = await ChainlinkPriceFeed.deploy();
 console.log("Contract address :", chainlinkPriceFeed.address);
 console.log("Price of Eth :", chainlinkPriceFeed.priceFeed);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
