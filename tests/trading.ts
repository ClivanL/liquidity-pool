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

  it("Init sell order book directory", async()=>{
    let token_pair = "bc";
    let direction = "sell";
    const [sellOrderBookDirectoryPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("orderbook_directory"), Buffer.from(token_pair), Buffer.from(direction)],program.programId);
    const tx = await program.methods.createOrderBookDirectory(token_pair,direction).accounts({
        orderBookDirectory:sellOrderBookDirectoryPda,
        initializer:user.publicKey
    }).signers([user]).rpc();
    const sellOrderBookDirectory = await program.account.orderBookDirectory.fetch(sellOrderBookDirectoryPda);
    expect(sellOrderBookDirectory.lastIndex).to.equal(0);
    expect(sellOrderBookDirectory.orderbookSubseeds.length).to.equal(0);
    console.log(sellOrderBookDirectory.direction);
    console.log(sellOrderBookDirectory.tokenPair);
    console.log(tx);
  })

  it("Init buy order book directory", async()=>{
    let token_pair = "bc";
    let direction = "buy";
    const [buyOrderBookDirectoryPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("orderbook_directory"), Buffer.from(token_pair), Buffer.from(direction)],program.programId);
    const tx = await program.methods.createOrderBookDirectory(token_pair,direction).accounts({
        orderBookDirectory:buyOrderBookDirectoryPda,
        initializer:user.publicKey
    }).signers([user]).rpc();
    const buyOrderBookDirectory = await program.account.orderBookDirectory.fetch(buyOrderBookDirectoryPda);
    expect(buyOrderBookDirectory.lastIndex).to.equal(0);
    expect(buyOrderBookDirectory.orderbookSubseeds.length).to.equal(0);
    console.log(buyOrderBookDirectory.direction);
    console.log(buyOrderBookDirectory.tokenPair);
    console.log(tx);
  })

  it("Init buy order books", async()=>{
    try{
      let token_pair = "bc";
      let direction = "buy";
      const [buyOrderBookDirectoryPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("orderbook_directory"), Buffer.from(token_pair), Buffer.from(direction)],program.programId);
      let buyOrderBookDirectory = await program.account.orderBookDirectory.fetch(buyOrderBookDirectoryPda);
      console.log(buyOrderBookDirectory.lastIndex);
      let buyOrderBookSubSeed = "OB"+buyOrderBookDirectory.lastIndex;
      const [buyOrderBookPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("orderbook"), Buffer.from(token_pair), Buffer.from(direction),Buffer.from(buyOrderBookSubSeed)],program.programId);
      const tx = await program.methods.createOrderBook(token_pair,direction,buyOrderBookSubSeed).accounts({
        orderBook:buyOrderBookPda,
        orderBookDirectory:buyOrderBookDirectoryPda,
        initializer:user.publicKey
      }).signers([user]).rpc();
      let buyOrderBook = await program.account.orderBook.fetch(buyOrderBookPda);
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
        console.error("Unexpected error:", err);
      }
    }
  })

  it("Init sell order books", async()=>{
    let token_pair = "bc";
    let direction = "sell";
    const [sellOrderBookDirectoryPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("orderbook_directory"), Buffer.from(token_pair), Buffer.from(direction)],program.programId);
    let sellOrderBookDirectory = await program.account.orderBookDirectory.fetch(sellOrderBookDirectoryPda);
    console.log(sellOrderBookDirectory.lastIndex);
    let sellOrderBookSubSeed = "OB"+sellOrderBookDirectory.lastIndex;
    const [sellOrderBookPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("orderbook"), Buffer.from(token_pair), Buffer.from(direction),Buffer.from(sellOrderBookSubSeed)],program.programId);
    const tx = await program.methods.createOrderBook(token_pair,direction,sellOrderBookSubSeed).accounts({
        orderBook:sellOrderBookPda,
        orderBookDirectory:sellOrderBookDirectoryPda,
        initializer:user.publicKey
    }).signers([user]).rpc();
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
    const [buyOrderBookDirectoryPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("orderbook_directory"), Buffer.from(token_pair), Buffer.from(direction)],program.programId);
    let buyOrderBookDirectory = await program.account.orderBookDirectory.fetch(buyOrderBookDirectoryPda);
    console.log(buyOrderBookDirectory.orderbookSubseeds);
    let orderbook_seed = buyOrderBookDirectory.orderbookSubseeds[buyOrderBookDirectory.orderbookSubseeds.length-1];
    console.log(orderbook_seed);
    const [buyOrderBookPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("orderbook"), Buffer.from(token_pair), Buffer.from(direction), Buffer.from(orderbook_seed)],program.programId);
    for (let i=0;i<3;i++){
      let buyOrderBook = await program.account.orderBook.fetch(buyOrderBookPda);
      let initialOrders = buyOrderBook.orders.length;
      let [limitOrderPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("order"),Buffer.from(direction),Buffer.from("buy_"+buyOrderBook.lastIndex),Buffer.from(token_pair), Buffer.from(orderbook_seed)],program.programId);
      let quantity = 1.0;
      let exchangeRate = 5.0
      const tx = await program.methods.createLimitOrder(direction,"buy_"+buyOrderBook.lastIndex,token_pair,orderbook_seed,quantity,exchangeRate).accounts({
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

  //userTokenAccount for token_c was created and topped up in previous test case.
  it("Init multiple sell limit order for token pair b-c", async()=>{
    let token_pair = "bc";
    let direction = "sell";
    const [userTokenAccountPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("token_c"),user.publicKey.toBuffer()],program.programId);
    let userTokenAccount = await program.account.userAccount.fetch(userTokenAccountPda); 
    console.log("balance:",userTokenAccount.balance);
    const [sellOrderBookDirectoryPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("orderbook_directory"), Buffer.from(token_pair), Buffer.from(direction)],program.programId);
    let sellOrderBookDirectory = await program.account.orderBookDirectory.fetch(sellOrderBookDirectoryPda);
    console.log(sellOrderBookDirectory.orderbookSubseeds);
    let orderbook_seed = sellOrderBookDirectory.orderbookSubseeds[sellOrderBookDirectory.orderbookSubseeds.length-1];
    console.log(orderbook_seed);
    const [sellOrderBookPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("orderbook"), Buffer.from(token_pair), Buffer.from(direction), Buffer.from(orderbook_seed)],program.programId);
    for (let i=0;i<3;i++){
      let sellOrderBook = await program.account.orderBook.fetch(sellOrderBookPda);
      let initialOrders = sellOrderBook.orders.length;
      let [limitOrderPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("order"),Buffer.from(direction),Buffer.from("buy_"+sellOrderBook.lastIndex),Buffer.from(token_pair), Buffer.from(orderbook_seed)],program.programId);
      let quantity = 1.0;
      let exchangeRate = 5.0
      const tx = await program.methods.createLimitOrder(direction,"buy_"+sellOrderBook.lastIndex,token_pair,orderbook_seed,quantity,exchangeRate).accounts({
        limitOrder:limitOrderPda,
        orderBook:sellOrderBookPda,
        userTokenAccount:userTokenAccountPda,
        user:user.publicKey
    }).signers([user]).rpc();
    console.log(tx);
    let buyOrderBookAfter = await program.account.orderBook.fetch(sellOrderBookPda); 
    let newOrders = buyOrderBookAfter.orders.length;
    expect(newOrders).to.equal(initialOrders+1);
    let limitOrder = await program.account.limitOrder.fetch(limitOrderPda);
    expect(limitOrder.amountToTrade).to.equal(quantity);
    }
  })


})
