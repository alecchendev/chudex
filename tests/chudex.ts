import * as anchor from "@project-serum/anchor";
import { Program, web3, Provider } from "@project-serum/anchor";
import { Chudex } from "../target/types/chudex";
import { ASSOCIATED_TOKEN_PROGRAM_ID, Token, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { expect } from "chai";

async function createWallet(provider: Provider, lamports: number): Promise<web3.Keypair> {
  const wallet = web3.Keypair.generate();
  const fundTx = new web3.Transaction().add(
      web3.SystemProgram.transfer({
          fromPubkey: provider.wallet.publicKey,
          toPubkey: wallet.publicKey,
          lamports,
      })
  );

  // const signedFundTx = await provider.wallet.signTransaction(fundTx);
  const response = await provider.send(fundTx);

  // await this.sendAndConfirmTransaction(fundTx, [this.authority]);
  return wallet;
}

async function createToken(
  user: web3.Keypair,
  provider: Provider,
  decimals: number,
): Promise<Token> {
  const token = await Token.createMint(
      provider.connection,
      user,
      user.publicKey,
      user.publicKey,
      decimals,
      TOKEN_PROGRAM_ID
  );

  return token;
}

describe("chudex", async () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Chudex as Program<Chudex>;

  it("Is initialized!", async () => {
    // await program.provider.connection.requestAirdrop(user.publicKey, 2e9);
    const user = await createWallet(program.provider, 2 * (10 ** 7));
    console.log("got here 1");
    const mintA = await createToken(user, program.provider, 9);
    const mintB = await createToken(user, program.provider, 6);
    
    // const mintA = await Token.createMint(
    //   program.provider.connection,
    //   user,
    //   user.publicKey,
    //   user.publicKey,
    //   9,
    //   TOKEN_PROGRAM_ID,
    // );

    console.log("got here 2");
    // const mintB = await Token.createMint(
    //   program.provider.connection,
    //   user,
    //   user.publicKey,
    //   user.publicKey,
    //   6,
    //   TOKEN_PROGRAM_ID,
    // );

    const [pool, poolBump] = await web3.PublicKey.findProgramAddress(
      [
        Buffer.from("pool"),
        mintA.publicKey.toBuffer(),
        mintB.publicKey.toBuffer(),
      ],
      program.programId,
    );

    const [otherPool, otherBump] = await web3.PublicKey.findProgramAddress(
      [
        Buffer.from("pool"),
        mintB.publicKey.toBuffer(),
        mintA.publicKey.toBuffer(),
      ],
      program.programId,
    );

    const [mintLp, mintLpBump ] = await web3.PublicKey.findProgramAddress(
      [
        Buffer.from("mint_lp"),
        pool.toBuffer(),
      ],
      program.programId,
    );

    const vaultA = await Token.getAssociatedTokenAddress(
      ASSOCIATED_TOKEN_PROGRAM_ID,
      TOKEN_PROGRAM_ID,
      mintA.publicKey,
      pool,
      true,
    );

    const vaultB = await Token.getAssociatedTokenAddress(
      ASSOCIATED_TOKEN_PROGRAM_ID,
      TOKEN_PROGRAM_ID,
      mintB.publicKey,
      pool,
      true,
    );

    // Add your test here.
    const k = 200;
    const tx = await program.rpc.initialize(
      new anchor.BN(k),
      {
        accounts:{
          pool: pool,
          otherPool: otherPool,
          vaultA: vaultA,
          vaultB: vaultB,
          mintA: mintA.publicKey,
          mintB: mintB.publicKey,
          mintLp: mintLp,
          user: user.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: web3.SystemProgram.programId,
          rent: web3.SYSVAR_RENT_PUBKEY,
        },
        signers: [ user ]
      }
    );
    console.log("Your transaction signature", tx);
    let poolState = await program.account.pool.fetch(pool);
    expect(poolState.k.toNumber()).to.equal(k);
  });
});
