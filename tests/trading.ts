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
import {SendTransactionError} from '@solana/web3.js'



describe("trading", () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);
  dotenv.config(); 

  const program = anchor.workspace.MultipleTokens as Program<MultipleTokens>;
  const user = getKeypairFromEnvironment("SECRET_KEY");

  it("Init buy order books", async()=>{
    try{
      let token_pair = "bc";
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
    }
    catch (err){
      if (err instanceof SendTransactionError) {
        const logs = await err.getLogs(provider.connection)
        console.error("Program error logs:", logs);
      } else {
        console.error("Unexpected error:", err.getLogs(provider.connection));
      }
    }
  })

  it("Init sell order books", async()=>{
    let token_pair = "bc";
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

  //userTokenAccount for token_b was created and topped up in previous test case.
  it("Init multiple buy limit order for token pair b-c", async()=>{
    let token_pair = "bc";
    let direction = "buy";
    const [userTokenAccountPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("token_b"),user.publicKey.toBuffer()],program.programId);
    let userTokenAccount = await program.account.userAccount.fetch(userTokenAccountPda); 
    console.log("balance:",userTokenAccount.balance);
    const [buyOrderBookPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("orderbook"), Buffer.from(token_pair), Buffer.from(direction)],program.programId);
    for (let i=0;i<3;i++){
      let buyOrderBook = await program.account.orderBook.fetch(buyOrderBookPda);
      let initialOrders = buyOrderBook.orders.length;
      let [limitOrderPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("order"),Buffer.from(direction),Buffer.from("buy_"+buyOrderBook.lastIndex),Buffer.from(token_pair)],program.programId);
      let quantity = 1.0;
      let exchangeRate = 5.0
      const tx = await program.methods.createLimitOrder(direction,"buy_"+buyOrderBook.lastIndex,token_pair,quantity,exchangeRate).accounts({
        limitOrder:limitOrderPda,
        orderBook:buyOrderBookPda,
        userTokenAccount:userTokenAccountPda,
        user:user.publicKey
    }).signers([user]).rpc();
    console.log(tx);
    let buyOrderBookAfter = await program.account.orderBook.fetch(buyOrderBookPda); 
    let newOrders = buyOrderBookAfter.orders.length;
    expect(newOrders).to.equal(initialOrders+1);
    let limitOrder = await program.account.limitOrder.fetch(limitOrderPda);
    expect(limitOrder.amountToTrade).to.equal(quantity);
    }
  })
})
