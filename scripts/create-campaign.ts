import { AnchorProvider, BN, Program, Wallet } from "@coral-xyz/anchor";
import { clusterApiUrl, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, Transaction } from "@solana/web3.js";
import { FinalProject } from "./idl/final_project";
import dotenv from "dotenv";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
dotenv.config();

async function createCampaign() {
  const devnet = true;
  const keyPair = Keypair.fromSecretKey(bs58.decode(process.env.PRIV_KEY || ""));
  const connection = new Connection(clusterApiUrl(devnet ? "devnet" : "mainnet-beta"), { commitment: 'confirmed' });
  const wallet = new Wallet(keyPair);
  const provider = new AnchorProvider(connection, wallet);
  const IDL: FinalProject = require("./idl/final_project.json");
  const program = new Program(IDL, provider);

  const tx = new Transaction();
  let lastCampaignIndex = new BN(0);
  const [creator, bump] = PublicKey.findProgramAddressSync(
    [Buffer.from("creator"), keyPair.publicKey.toBuffer()],
    program.programId
  );
  const creatorAccountInfo = await connection.getAccountInfo(creator);
  if (!creatorAccountInfo) {
    tx.add(await program.methods.initializeCreator(
      bump,
    ).accounts({
      creator: keyPair.publicKey,
    }).instruction());
  } else {
    lastCampaignIndex = (await program.account.creator.fetch(creator)).lastCampaignIndex;
  }

  const [campaign, campaignBump] = PublicKey.findProgramAddressSync(
    [Buffer.from("campaign"), keyPair.publicKey.toBuffer(), Buffer.from(lastCampaignIndex.toArray("le", 8))],
    program.programId
  );

  const campaignTokenName = "Save Water";
  const campaignTokenSymbol = "H2O";
  const campaignTokenUri = "https://arweave.net/123";
  const depositDeadline = new BN("");
  const tradeDeadline = new BN("");
  const donationGoal = new BN(1.5 * LAMPORTS_PER_SOL);
  tx.add(await program.methods.createCampaign(
    campaignBump,
    campaignTokenName,
    campaignTokenSymbol,
    campaignTokenUri,
    depositDeadline,
    tradeDeadline,
    donationGoal,
  ).accounts({
    creator: keyPair.publicKey,
    campaignAccount: campaign,
  }).instruction());

  tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
  tx.feePayer = keyPair.publicKey;
  const recoverTx = Transaction.from(tx.serialize({ requireAllSignatures: false }));
  recoverTx.sign(keyPair);

  const txSignature = await connection.sendRawTransaction(recoverTx.serialize({ requireAllSignatures: true }));
  console.log("🚀 ~ createCampaign ~ txSignature:", txSignature)
  let latestBlockHash = await connection.getLatestBlockhash();
  await connection.confirmTransaction({
    blockhash: latestBlockHash.blockhash,
    lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
    signature: txSignature,
  });
}

createCampaign();
