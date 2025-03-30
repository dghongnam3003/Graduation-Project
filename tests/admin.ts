import * as anchor from "@coral-xyz/anchor";
import { FinalProject } from "../target/types/final_project";
import { expect } from "chai";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";

async function fundAccount(connection: anchor.web3.Connection, wallet: anchor.web3.PublicKey, amount: number) {
  const airdropSignature = await connection.requestAirdrop(wallet, amount * anchor.web3.LAMPORTS_PER_SOL);
  const latestBlockHash = await connection.getLatestBlockhash("confirmed");
  await connection.confirmTransaction({
    blockhash: latestBlockHash.blockhash,
    lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
    signature: airdropSignature,
  });
}

describe("admin", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const connection = provider.connection;

  const program = anchor.workspace.FinalProject as anchor.Program<FinalProject>;

  const admin = anchor.web3.Keypair.generate();
  const [config, bump] = PublicKey.findProgramAddressSync(
    [Buffer.from("config")],
    program.programId
  );
  const [treasury, treasuryBump] = PublicKey.findProgramAddressSync(
    [Buffer.from("treasury")],
    program.programId
  );
  const operator = Keypair.generate();
  const protocolFeePercentage = 1000;
  const tipPercentage = 500;

  beforeEach(async () => {
    await fundAccount(connection, admin.publicKey, 1);

    const configAccountInfo = await connection.getAccountInfo(config);
    if (configAccountInfo === null) {
      await program.methods.initialize(
        bump,
        treasuryBump,
        operator.publicKey,
        protocolFeePercentage,
        tipPercentage,
      ).accounts({
        admin: admin.publicKey,
      }).signers([admin]).rpc();
    }
  });

  it("should failed when update config without permission", async () => {
    const fakeAdmin = anchor.web3.Keypair.generate();
    await fundAccount(connection, fakeAdmin.publicKey, 1);

    const newProtocolFeePercentage = 1500;

    try {
      await program.methods.setFee(
        newProtocolFeePercentage,
        tipPercentage,
      ).accounts({
        admin: fakeAdmin.publicKey,
      }).signers([fakeAdmin]).rpc();

      expect(false).to.be.true;
    } catch (err) {
      expect(err.toString()).include("AnchorError caused by account: config. Error Code: ConstraintHasOne.");
    }
  });

  it("should failed when update config with invalid fee", async () => {
    const newProtocolFeePercentage = 10001;
    const newTipPercentage = 10001;

    const configAccount = await program.account.config.fetch(config);

    try {
      await program.methods.setFee(
        newProtocolFeePercentage,
        configAccount.tipPercentage,
      ).accounts({
        admin: admin.publicKey,
      }).signers([admin]).rpc();

      expect(false).to.be.true;
    } catch (err) {
      expect(err.toString()).include("Error Number: 6001. Error Message: Invalid protocol fee percentage.");
    }

    try {
      await program.methods.setFee(
        configAccount.protocolFeePercentage,
        newTipPercentage,
      ).accounts({
        admin: admin.publicKey,
      }).signers([admin]).rpc();

      expect(false).to.be.true;
    } catch (err) {
      expect(err.toString()).include("Error Number: 6002. Error Message: Invalid tip percentage.");
    }
  });

  it("should success when update config as admin", async () => {
    const newProtocolFeePercentage = 1100;
    const newTipPercentage = 600;

    let configAccount = await program.account.config.fetch(config);
    expect(configAccount.protocolFeePercentage).to.be.eq(protocolFeePercentage);
    expect(configAccount.tipPercentage).to.be.eq(tipPercentage);

    await program.methods.setFee(
      newProtocolFeePercentage,
      newTipPercentage,
    ).accounts({
      admin: admin.publicKey,
    }).signers([admin]).rpc();

    configAccount = await program.account.config.fetch(config);
    expect(configAccount.protocolFeePercentage).to.be.eq(newProtocolFeePercentage);
    expect(configAccount.tipPercentage).to.be.eq(newTipPercentage);
  });

  it("should failed when update admin without permission", async () => {
    const fakeAdmin = anchor.web3.Keypair.generate();
    await fundAccount(connection, fakeAdmin.publicKey, 1);

    try {
      await program.methods.setAdmin(
        fakeAdmin.publicKey,
      ).accounts({
        admin: fakeAdmin.publicKey,
      }).signers([fakeAdmin]).rpc();

      expect(false).to.be.true;
    } catch (err) {
      expect(err.toString()).include("AnchorError caused by account: config. Error Code: ConstraintHasOne.");
    }
  });

  it("should success when update admin as admin", async () => {
    await program.methods.setAdmin(
      operator.publicKey,
    ).accounts({
      admin: admin.publicKey,
    }).signers([admin]).rpc();


    let configAccount = await program.account.config.fetch(config);
    expect(configAccount.admin.toBase58()).to.eql(operator.publicKey.toBase58());

    await program.methods.setAdmin(
      admin.publicKey,
    ).accounts({
      admin: operator.publicKey,
    }).signers([operator]).rpc();


    configAccount = await program.account.config.fetch(config);
    expect(configAccount.admin.toBase58()).to.eql(admin.publicKey.toBase58());
  });

  it("should failed when update operator without permission", async () => {
    const fakeAdmin = anchor.web3.Keypair.generate();
    await fundAccount(connection, fakeAdmin.publicKey, 1);

    try {
      await program.methods.setOperator(
        fakeAdmin.publicKey,
      ).accounts({
        admin: fakeAdmin.publicKey,
      }).signers([fakeAdmin]).rpc();

      expect(false).to.be.true;
    } catch (err) {
      expect(err.toString()).include("AnchorError caused by account: config. Error Code: ConstraintHasOne.");
    }
  });

  it("should success when update operator as admin", async () => {
    await program.methods.setOperator(
      admin.publicKey,
    ).accounts({
      admin: admin.publicKey,
    }).signers([admin]).rpc();


    let configAccount = await program.account.config.fetch(config);
    expect(configAccount.operator.toBase58()).to.eql(admin.publicKey.toBase58());
  });
});
