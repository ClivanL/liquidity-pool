import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MultipleTokens } from "../target/types/multiple_tokens";
import { Connection, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";

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
  const [liquidityPoolPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("liquidity_pool")],program.programId) 

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
});
