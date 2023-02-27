import { expect, use } from "chai";
import chaiAsPromised from "chai-as-promised";
import NftFactory from "./typedContract/constructors/nft";
import Nft from "./typedContract/contracts/nft";
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { IdBuilder } from "./typedContract/types-arguments/nft";

use(chaiAsPromised);

// Create a new instance of contract
const wsProvider = new WsProvider("ws://127.0.0.1:9944");
// Create a keyring instance
const keyring = new Keyring({ type: "sr25519" });

describe("nft test", () => {
  const id0 = IdBuilder.U8(0)
  const id1 = IdBuilder.U8(1)
  let nftFactory: NftFactory;
  let api: ApiPromise;
  let deployer: KeyringPair;
  let signer1: KeyringPair;
  let contract: Nft;  

  before(async function setup(): Promise<void> {
    api = await ApiPromise.create({ provider: wsProvider });
    deployer = keyring.addFromUri("//Alice");
    signer1 = keyring.addFromUri("//Bob");

    nftFactory = new NftFactory(api, deployer);

    contract = new Nft(
      (await nftFactory.new()).address,
      deployer,
      api
    );
  });

  after(async function tearDown() {
    await api.disconnect();
  });

  it("Mint nft", async () => {           
    await contract.withSigner(signer1).tx.mint(deployer.address, id0)
    await contract.withSigner(signer1).tx.mint(deployer.address, id1)
    
    expect((await contract.query.totalSupply()).value.unwrap().toNumber()).to.equal(2)
    expect(((await contract.query.ownerOf(id0)).value.unwrap()?.toString())).to.equal(deployer.address)
  })
});
