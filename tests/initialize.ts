import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { FinalProject } from "../target/types/final_project";
import { Keypair, PublicKey } from "@solana/web3.js";
import { expect } from "chai";

describe("initialize", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const connection = provider.connection;

  const program = anchor.workspace.FinalProject as Program<FinalProject>;

  it("Is initialized!", async () => {
    const admin = anchor.web3.Keypair.generate();
    const airdropSignature = await connection.requestAirdrop(admin.publicKey, anchor.web3.LAMPORTS_PER_SOL);
    let latestBlockHash = await connection.getLatestBlockhash("confirmed");
    await connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: airdropSignature,
    });

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

    await program.methods.initialize(
      bump,
      treasuryBump,
      operator.publicKey,
      protocolFeePercentage,
      tipPercentage,
    ).accounts({
      admin: admin.publicKey,
    }).signers([admin]).rpc();

    const configAccount = await program.account.config.fetch(config);
    expect(configAccount.bump).to.eql(bump);
    expect(configAccount.admin).to.eql(admin.publicKey);
    expect(configAccount.protocolFeePercentage).to.eql(protocolFeePercentage);
    expect(configAccount.tipPercentage).to.eql(tipPercentage);

    const treasuryAccount = await program.account.treasury.fetch(treasury);
    expect(treasuryAccount.bump).to.eql(treasuryBump);
  });
});
