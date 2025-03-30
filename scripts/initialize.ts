import { AnchorProvider, Program, Wallet } from "@coral-xyz/anchor";
import { clusterApiUrl, Connection, Keypair, PublicKey, Transaction } from "@solana/web3.js";
import { FinalProject } from "./idl/final_project";
import dotenv from "dotenv";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
dotenv.config();

async function initialize() {
  const devnet = true;
  const keyPair = Keypair.fromSecretKey(bs58.decode(process.env.PRIV_KEY || ""));
  const connection = new Connection(clusterApiUrl(devnet ? "devnet" : "mainnet-beta"), { commitment: 'confirmed' });
  const wallet = new Wallet(keyPair);
  const provider = new AnchorProvider(connection, wallet);
  const IDL: FinalProject = require("./idl/final_project.json");
  const program = new Program(IDL, provider);

  console.log("🚀 ~ initialize ~ program:", program.programId.toBase58())

  const [config, bump] = PublicKey.findProgramAddressSync(
    [Buffer.from("config")],
    program.programId
  );
  console.log("🚀 ~ initialize ~ config:", config.toBase58())


  const [treasury, treasuryBump] = PublicKey.findProgramAddressSync(
    [Buffer.from("treasury")],
    program.programId
  );
  console.log("🚀 ~ initialize ~ treasury:", treasury.toBase58())

  const operator = new PublicKey("GAuAerfzP7x2hqkeRgSDofq73Vm1NRSwX6fkSK6c53Kz");
  const protocolFeePercentage = 1000;
  const tipPercentage = 500;
  const tx = new Transaction().add(await program.methods.initialize(
    bump,
    treasuryBump,
    operator,
    protocolFeePercentage,
    tipPercentage,
  ).accounts({
    admin: keyPair.publicKey,
  }).instruction());

  tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
  tx.feePayer = keyPair.publicKey;
  const recoverTx = Transaction.from(tx.serialize({ requireAllSignatures: false }));
  recoverTx.sign(keyPair);

  const txSignature = await connection.sendRawTransaction(recoverTx.serialize({ requireAllSignatures: true }));
  console.log("🚀 ~ initialize ~ txSignature:", txSignature)
  let latestBlockHash = await connection.getLatestBlockhash();
  await connection.confirmTransaction({
    blockhash: latestBlockHash.blockhash,
    lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
    signature: txSignature,
  });
}

initialize();
