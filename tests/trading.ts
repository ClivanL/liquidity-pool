import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MultipleTokens } from "../target/types/multiple_tokens";
import { PublicKey } from "@solana/web3.js";
import { BN } from "bn.js";
import { getAssociatedTokenAddress } from "@solana/spl-token";
import { ProgramError } from '@project-serum/anchor';
import { getKeypairFromEnvironment } from "@solana-developers/helpers";
import dotenv from "dotenv";
import { expect } from "chai";
import {
  AggregatorAccount,
  SwitchboardProgram,
} from "@switchboard-xyz/solana.js";
import { Big } from "big.js";



describe("trading", () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);
  dotenv.config(); 

  const program = anchor.workspace.MultipleTokens as Program<MultipleTokens>;

  it("Init buy order books", async()=>{
    let token_pair = "ab";
    let direction = "buy";
    const [buyOrderBookPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("orderbook"), Buffer.from(token_pair), Buffer.from(direction)],program.programId);
    const tx = await program.methods.createOrderBook(token_pair,direction).accounts({
        orderBook:buyOrderBookPda
    }).rpc();
    const buyOrderBook = await program.account.orderBook.fetch(buyOrderBookPda);
    expect(buyOrderBook.lastIndex).to.equal(0);
    console.log(buyOrderBook.direction);
    console.log(buyOrderBook.tokenPair);
    console.log(tx);
  })

  it("Init sell order books", async()=>{
    let token_pair = "ab";
    let direction = "sell";
    const [sellOrderBookPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("orderbook"), Buffer.from(token_pair), Buffer.from(direction)],program.programId);
    const tx = await program.methods.createOrderBook(token_pair,direction).accounts({
        orderBook:sellOrderBookPda
    }).rpc();
    const sellOrderBook = await program.account.orderBook.fetch(sellOrderBookPda);
    expect(sellOrderBook.lastIndex).to.equal(0);
    console.log(sellOrderBook.direction);
    console.log(sellOrderBook.tokenPair);
    console.log(tx);
  })

})
