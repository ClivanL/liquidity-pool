import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MultipleTokens } from "../target/types/multiple_tokens";
import { PublicKey, Keypair } from "@solana/web3.js";
import { BN } from "bn.js";
import { getAssociatedTokenAddress } from "@solana/spl-token";
import { ProgramError } from '@project-serum/anchor';
import { getKeypairFromEnvironment } from "@solana-developers/helpers";
import dotenv from "dotenv";
import { expect } from "chai";
import { loadDataFeed } from "./oracle";
import {
  AggregatorAccount,
  SwitchboardProgram,
} from "@switchboard-xyz/solana.js";
//import { Big } from "big.js";



describe("multiple-tokens", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);
  dotenv.config(); 

  const program = anchor.workspace.MultipleTokens as Program<MultipleTokens>;

  const mintA = new PublicKey("8ujet5mjeD9LsrTVQWoqKgtAqG9UDrhaGQzPDq6zVBaG");
  const mintB = new PublicKey("5pVceB5fkyq9pvruXYs3JizY1pzEs3FuiDKsuLwmoKb7");
  const mintC = new PublicKey("E2oRctBw6tgsdXa7SgDwu2ANx4hfB8LjRfWm9EA5so2s");
  const mintD = new PublicKey("CpTtdKyFBdEd6aGoDXzfP5jGPNSEZBeySSiHjZxZUUfq");
  const mintE = new PublicKey("CL9C8QdZGgsMumycxZvKYoS9RoytefRt3cnFrFpoPSc");

  const tokenAccountA = new PublicKey("CqvQiFgRhBJHSdqFkoEFirevRgCm1SFo3F7LEZLv79Ve");
  const tokenAccountB = new PublicKey("FeAVqaCntsYMx7fEiT9M6pFFDZLGmLjeEPWD6PHzzm9Y");
  const tokenAccountC = new PublicKey("9tqn98NRKibaypvQcBszD5pi26tSB7UG1LWaUcyUGLqY");
  const tokenAccountD = new PublicKey("HvxMgtSCxT1AaFzuyZ3wurD65Bnj1Ugis31WFS1kwQWH");
  const tokenAccountE = new PublicKey("3qCXtfBbYJADfM91eZyHx5SRyEPb9b98w6PcDQTkCehP");
  const [liquidityPoolPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("liquidity_pool")],program.programId) 
  const [lpMintPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("lp_mint")],program.programId)
  const user = getKeypairFromEnvironment("SECRET_KEY");

  const ORACLE_FEED_ADDRESS = new PublicKey("q7AQr3jWfuKjeSy5anC4jz79gkgKrHnCNGd2xxTi9Mo");

  it("Liquidity pool initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.createLiquidityPool().rpc();
    console.log("Your transaction signature", tx);
  });

  it("Create token vault A, B and C!", async () => {
    // Add your test here.
    const tx = await program.methods.createTokenVaultAbc().accounts(
      {liquidityPool:liquidityPoolPda,
        tokenAMint:mintA,
        tokenBMint:mintB,
        tokenCMint:mintC
      }
    ).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Create token vault D and E!", async () => {
    const tx = await program.methods.createTokenVaultDe().accounts(
      {liquidityPool:liquidityPoolPda,
        tokenDMint:mintD,
        tokenEMint:mintE
      }
    ).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Create account for user for token C!", async () => {
    // Add your test here.
    const tx = await program.methods.createAccount("token_c").accounts(
      {userTokenVault:tokenAccountC,
        user:user.publicKey
    }).signers([user]).rpc();
    console.log("Your transaction signature", tx);
    const [userTokenAccountPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("token_c"),user.publicKey.toBuffer()],program.programId);
    const userTokenAccount = await program.account.userAccount.fetch(userTokenAccountPda);
    console.log(userTokenAccount.user);
    let expectedBalance = 0;
    let receivedBalance = userTokenAccount.balance.toNumber();
    console.log(userTokenAccount.tokenName.toString());
    expect(receivedBalance).to.equal(expectedBalance);
  });

  it("Create account for user for invalid token name!", async () => {
    // Add your test here.
    try{
      const tx = await program.methods.createAccount("token_invalid").accounts(
        {userTokenVault:tokenAccountA,
          user:user.publicKey
      }).signers([user]).rpc();
      console.log("Your transaction signature", tx);
      const [userTokenAccountPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("token_a"),user.publicKey.toBuffer()],program.programId);
      const userTokenAccount = await program.account.userAccount.fetch(userTokenAccountPda);
      console.log(userTokenAccount.user);
      let expectedBalance = 0;
      let receivedBalance = userTokenAccount.balance.toNumber();
      expect(receivedBalance).to.equal(expectedBalance);
      throw new Error ("Account created using invalid token name");
    }
    catch (err){
      expect(err.message).to.include("The token name does not exist.");
    }

  });

  it("Add liquidity to pool A, C, E!", async () => {
    const tokenAVaultAddress = await getAssociatedTokenAddress(mintA, liquidityPoolPda,true);
    const tokenBVaultAddress = await getAssociatedTokenAddress(mintB, liquidityPoolPda,true);
    const tokenCVaultAddress = await getAssociatedTokenAddress(mintC, liquidityPoolPda,true);
    const tokenDVaultAddress = await getAssociatedTokenAddress(mintD, liquidityPoolPda,true);
    const tokenEVaultAddress = await getAssociatedTokenAddress(mintE, liquidityPoolPda,true);
    const addA = new BN(5);
    const addB = new BN(0);
    const addC = new BN(5);
    const addD = new BN(0);
    const addE = new BN(5);
    console.log(tokenEVaultAddress);
    try{
      const tx = await program.methods.addLiquidity(addA,addB,addC,addD,addE).accounts(
        {liquidityPool:liquidityPoolPda,
          userTokenA:tokenAccountA,
          userTokenB:tokenAccountB,
          userTokenC:tokenAccountC,
          userTokenD:tokenAccountD,
          userTokenE:tokenAccountE,
          tokenAVault:tokenAVaultAddress,
          tokenBVault:tokenBVaultAddress,
          tokenCVault:tokenCVaultAddress,
          tokenDVault:tokenDVaultAddress,
          tokenEVault:tokenEVaultAddress,
          user:user.publicKey,
          lpMint:lpMintPda
        }
      ).signers([user]).rpc();

      console.log("Your transaction signature", tx);
    }
    catch (err){
      if (err instanceof ProgramError) {
        console.error("Program error logs:", err.logs);
      } else {
        console.error("Unexpected error:", err);
      }
    }
  });

  it("Create account for user for token B!", async () => {
    // Add your test here.
    const tx = await program.methods.createAccount("token_b").accounts(
      {userTokenVault:tokenAccountA,
        user:user.publicKey
    }).signers([user]).rpc();
    console.log("Your transaction signature", tx);
    const [userTokenAccountPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("token_b"),user.publicKey.toBuffer()],program.programId);
    const userTokenAccount = await program.account.userAccount.fetch(userTokenAccountPda);
    console.log(userTokenAccount.user);
    let expectedBalance = 0;
    let receivedBalance = userTokenAccount.balance.toNumber();
    expect(receivedBalance).to.equal(expectedBalance);
  });

  it("Add liquidity to pool B, add_liquidity_v2!", async () => {
    const tokenBVaultAddress = await getAssociatedTokenAddress(mintB, liquidityPoolPda,true);

    const add = new BN(5);
    const [userTokenAccountPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("token_b"),user.publicKey.toBuffer()],program.programId);
    try{
      const userTokenAccount = await program.account.userAccount.fetch(userTokenAccountPda);
      let existingBalance = userTokenAccount.balance.toNumber();
      const tx = await program.methods.addLiquidityV2(add).accounts(
        {
          userTokenAccount:userTokenAccountPda,
          userToken:tokenAccountB,
          tokenVault:tokenBVaultAddress,
          user:user.publicKey,
        }
      ).signers([user]).rpc();

      console.log("Your transaction signature", tx);
      const updatedUserTokenAccount = await program.account.userAccount.fetch(userTokenAccountPda);
      let newBalance = updatedUserTokenAccount.balance.toNumber();
      expect(newBalance).to.equal(existingBalance+add.toNumber());
    }
    catch (err){
      if (err instanceof ProgramError) {
        console.error("Program error logs:", err.logs);
      } else {
        console.error("Unexpected error:", err);
      }
    }
  });

  it("Init stake records", async()=>{
    const tx = await program.methods.initStakeRecords().rpc();
    console.log(tx);
    const [stakeRecordsPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("stake_records")],program.programId);
    const stakeRecords = await program.account.stakeRecords.fetch(stakeRecordsPda);
    expect(stakeRecords.tokenAStake.toNumber()).to.equal(0);
  })

  it("Create token vault for lp token!", async () => {
    const tx = await program.methods.createLpTokenVault().accounts(
      {liquidityPool:liquidityPoolPda,
        lpMint:lpMintPda
      }
    ).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Stake tokens from token B account to exchange for lp token, with sufficient balance", async()=>{
    
    const [userTokenAccountBPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("token_b"),user.publicKey.toBuffer()],program.programId);
    const [stakeRecordsPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("stake_records")],program.programId);
    const lpTokenVaultAddress = await getAssociatedTokenAddress(lpMintPda, liquidityPoolPda,true);
    let [userLpTokenAccountPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("lp_token"),user.publicKey.toBuffer()],program.programId);

    let initialStakeRecords = await program.account.stakeRecords.fetch(stakeRecordsPda);
    let initialTokenBStakeBalance = initialStakeRecords.tokenBStake.toNumber();

    let initialUserTokenBAccount = await program.account.userAccount.fetch(userTokenAccountBPda);
    let initialTokenBAccountBalance = initialUserTokenBAccount.balance.toNumber();

    let stakeAmountInt = 5;
    let stakeAmount = new BN(stakeAmountInt);

    // const switchboardProgram = await SwitchboardProgram.load(
    //   new anchor.web3.Connection("https://api.devnet.solana.com"),
    //   user,
    // );
    // const aggregatorAccount = new AggregatorAccount(
    //   switchboardProgram,
    //   ORACLE_FEED_ADDRESS,
    // );
    // const aggregatorAccounts = await aggregatorAccount.fetchAccounts();
    // //console.log(aggregatorAccounts)

    // const prices:Big|null = await aggregatorAccount.fetchLatestValue();
    // console.log(prices);
    // const test = await aggregatorAccount.loadData();

    // const jobPubkeys = test.jobPubkeysData;
    // for (let i = 0; i < jobPubkeys.length; i++) {
    //   const jobPubkey = jobPubkeys[i];
    //   if (jobPubkey.toBase58() !== anchor.web3.PublicKey.default.toBase58()) {
    //     const jobAccount = new JobAccount(switchboardProgram, jobPubkey);
    //     //const jobData = await jobAccount.loadData();
    //     let price:Big|null = await jobAccount.loadData();
    //     console.log(`Job ${i + 1} result:`, price);
    //   }
    // }

    // const results = await aggregatorAccount.getConfirmedRoundResults(test);
    // console.log(results[1].value);
    // console.log(results[0].value);
    

    const tx = await program.methods.stakeTokens(stakeAmount).accounts({
      liquidityPool:liquidityPoolPda,
      userTokenAccount:userTokenAccountBPda,
      stakeRecords:stakeRecordsPda,
      tokenLpVault:lpTokenVaultAddress,
      user:user.publicKey,
      //aggregatorAccount:aggregatorAccount,
      lpMint:lpMintPda,
      userLpTokenAccount:userLpTokenAccountPda
    }).signers([user]).rpc();
    console.log(tx);

    const stakeRecords = await program.account.stakeRecords.fetch(stakeRecordsPda);
    let newTokenBStakeBalance = stakeRecords.tokenBStake.toNumber();
    expect(newTokenBStakeBalance).to.equal(initialTokenBStakeBalance+stakeAmountInt);

    let newUserTokenBAccount = await program.account.userAccount.fetch(userTokenAccountBPda);
    let newTokenBAccountBalance = newUserTokenBAccount.balance.toNumber();

    expect(newTokenBAccountBalance).to.equal(initialTokenBAccountBalance-stakeAmountInt);
  })

});
