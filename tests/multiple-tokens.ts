import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MultipleTokens } from "../target/types/multiple_tokens";
import { Connection, LAMPORTS_PER_SOL, PublicKey, Keypair } from "@solana/web3.js";
import { BN } from "bn.js";
import { getAssociatedTokenAddress } from "@solana/spl-token";
import { ProgramError } from '@project-serum/anchor';

describe("multiple-tokens", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

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
  const secretKeyUint8Array: Uint8Array = new Uint8Array([
    // 32 bytes of your secret key
    112,191,163,148,172,52,241,177,108,92,63,145,24,59,229,241,148,80,195,132,237,80,51,252,157,118,126,240,127,233,244,23,12,254,187,114,212,86,81,191,250,203,190,174,196,194,209,99,120,7,112,150,230,174,117,49,170,209,237,244,222,16,20,116
  ]);
  const user = Keypair.fromSecretKey(secretKeyUint8Array);

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
    }
    catch (err){
      if (err instanceof ProgramError) {
        console.error("Program error logs:", err.logs);
      } else {
        console.error("Unexpected error:", err);
      }
    }

    ///console.log("Your transaction signature", tx);
  });
});
