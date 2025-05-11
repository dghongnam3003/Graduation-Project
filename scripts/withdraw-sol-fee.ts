import { AnchorProvider, BN, Program, Wallet } from "@coral-xyz/anchor";
import { clusterApiUrl, Connection, Keypair, LAMPORTS_PER_SOL, Transaction } from "@solana/web3.js";
import { FinalProject } from "./idl/final_project";
import dotenv from "dotenv";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
dotenv.config();

async function withdrawSolFee() {
  const devnet = true;
  const keyPair = Keypair.fromSecretKey(bs58.decode(process.env.PRIV_KEY || ""));
  const connection = new Connection(clusterApiUrl(devnet ? "devnet" : "mainnet-beta"), { commitment: 'confirmed' });
  const wallet = new Wallet(keyPair);
  const provider = new AnchorProvider(connection, wallet);
  const IDL: FinalProject = require("./idl/final_project.json");
  const program = new Program(IDL, provider);

  const tx = new Transaction();
  const amount = new BN(10 * LAMPORTS_PER_SOL);

  tx.add(await program.methods.withdrawFee(amount).accounts({
    admin: keyPair.publicKey,
  }).instruction());

  tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
  tx.feePayer = keyPair.publicKey;
  const recoverTx = Transaction.from(tx.serialize({ requireAllSignatures: false }));
  recoverTx.sign(keyPair);

  const txSignature = await connection.sendRawTransaction(recoverTx.serialize({ requireAllSignatures: true }));
  console.log("🚀 ~ withdrawSolFee ~ txSignature:", txSignature)
  let latestBlockHash = await connection.getLatestBlockhash();
  await connection.confirmTransaction({
    blockhash: latestBlockHash.blockhash,
    lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
    signature: txSignature,
  });
}

withdrawSolFee();
