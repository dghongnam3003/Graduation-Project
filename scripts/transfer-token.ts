import { AnchorProvider, BN, Program, Wallet } from "@coral-xyz/anchor";
import { clusterApiUrl, Connection, Keypair, PublicKey, Transaction } from "@solana/web3.js";
import { findMetadataPda, MPL_TOKEN_METADATA_PROGRAM_ID } from "@metaplex-foundation/mpl-token-metadata";
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults';
import { publicKey } from "@metaplex-foundation/umi";
import { FinalProject } from "./idl/final_project";
import dotenv from "dotenv";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";
dotenv.config();

async function transferCampaignToken() {
  const devnet = true;
  const keyPair = Keypair.fromSecretKey(bs58.decode(process.env.PRIV_KEY || ""));
  const connection = new Connection(clusterApiUrl(devnet ? "devnet" : "mainnet-beta"), { commitment: 'confirmed' });
  const wallet = new Wallet(keyPair);
  const provider = new AnchorProvider(connection, wallet);
  const IDL: FinalProject = require("./idl/final_project.json");
  const program = new Program(IDL, provider);

  const creatorAddress = new PublicKey(keyPair.publicKey); // REPLACE WITH CREATOR ADDRESS
  const campaignIndex = new BN(1); // REPLACE WITH CAMPAIGN INDEX
  const tx = new Transaction();

  const [config,] = PublicKey.findProgramAddressSync(
    [Buffer.from("config")],
    program.programId
  );

  const [campaign,] = PublicKey.findProgramAddressSync(
    [Buffer.from("campaign"), creatorAddress.toBuffer(), Buffer.from(campaignIndex.toArray("le", 8))],
    program.programId
  );

  const mint = new PublicKey("B1bvSU7avyAt8XNpaWSZkCoS8CT6csQyNGKTobT4xQKA");

  const associatedCampaign = getAssociatedTokenAddressSync(mint, campaign, true);
  const accounts = {
    admin: keyPair.publicKey,
    creator: keyPair.publicKey,
    campaignAccount: campaign,
    mint,
  }
  tx.add(await program.methods.transferToken(
    new BN(100000),
  ).accounts(accounts).instruction());

  tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
  tx.feePayer = keyPair.publicKey;
  const recoverTx = Transaction.from(tx.serialize({ requireAllSignatures: false }));
  recoverTx.sign(keyPair);

  const txSignature = await connection.sendRawTransaction(recoverTx.serialize({ requireAllSignatures: true }));
  console.log("🚀 ~ transferCampaignToken ~ txSignature:", txSignature)
  let latestBlockHash = await connection.getLatestBlockhash();
  await connection.confirmTransaction({
    blockhash: latestBlockHash.blockhash,
    lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
    signature: txSignature,
  });
}

transferCampaignToken();
