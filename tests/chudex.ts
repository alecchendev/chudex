import * as anchor from "@project-serum/anchor";
import { Program, web3, Provider } from "@project-serum/anchor";
import { Keypair, PublicKey, TransactionInstruction, Transaction, sendAndConfirmTransaction } from "@solana/web3.js";
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

type AllAccounts = {
  user: Keypair,
  mintA: Token,
  mintB: Token,
  mintLp: PublicKey,
  vaultA: PublicKey,
  vaultB: PublicKey,
  userTokenA: PublicKey,
  userTokenB: PublicKey,
  userTokenLp: PublicKey,
  pool: PublicKey,
  otherPool: PublicKey
}

async function getAccounts(
  provider: Provider,
  programId: web3.PublicKey,
): Promise<AllAccounts> {
  const user = await createWallet(provider, 2 * (10 ** 7));
  const mintA = await createToken(user, provider, 9);
  const mintB = await createToken(user, provider, 6);

  const [pool, poolBump] = await web3.PublicKey.findProgramAddress(
    [
      Buffer.from("pool"),
      mintA.publicKey.toBuffer(),
      mintB.publicKey.toBuffer(),
    ],
    programId,
  );

  const [otherPool, otherBump] = await web3.PublicKey.findProgramAddress(
    [
      Buffer.from("pool"),
      mintB.publicKey.toBuffer(),
      mintA.publicKey.toBuffer(),
    ],
    programId,
  );

  const [mintLp, mintLpBump ] = await web3.PublicKey.findProgramAddress(
    [
      Buffer.from("mint_lp"),
      pool.toBuffer(),
    ],
    programId,
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

  const userTokenA = await mintA.createAssociatedTokenAccount(
    user.publicKey
  );

  const userTokenB = await mintB.createAssociatedTokenAccount(
    user.publicKey
  );

  const userTokenLp = await Token.getAssociatedTokenAddress(
    ASSOCIATED_TOKEN_PROGRAM_ID,
    TOKEN_PROGRAM_ID,
    mintLp,
    user.publicKey,
  );

  return {
    user,
    mintA,
    mintB,
    mintLp,
    vaultA,
    vaultB,
    userTokenA,
    userTokenB,
    userTokenLp,
    pool,
    otherPool,
  };
}

describe("chudex", async () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Chudex as Program<Chudex>;
  const provider = program.provider;
  const programId = program.programId;

  let user: Keypair;
  let mintA: Token;
  let mintB: Token;
  let mintLp: PublicKey;
  let vaultA: PublicKey;
  let vaultB: PublicKey;
  let userTokenA: PublicKey;
  let userTokenB: PublicKey;
  let userTokenLp: PublicKey;
  let pool: PublicKey;
  let otherPool: PublicKey;

  before(async () => {

    user = await createWallet(provider, 2 * (10 ** 7));
    mintA = await createToken(user, provider, 9);
    mintB = await createToken(user, provider, 6);

    [pool, ] = await web3.PublicKey.findProgramAddress(
      [
        Buffer.from("pool"),
        mintA.publicKey.toBuffer(),
        mintB.publicKey.toBuffer(),
      ],
      programId,
    );

    [otherPool, ] = await web3.PublicKey.findProgramAddress(
      [
        Buffer.from("pool"),
        mintB.publicKey.toBuffer(),
        mintA.publicKey.toBuffer(),
      ],
      programId,
    );

    [mintLp, ] = await web3.PublicKey.findProgramAddress(
      [
        Buffer.from("mint_lp"),
        pool.toBuffer(),
      ],
      programId,
    );

    vaultA = await Token.getAssociatedTokenAddress(
      ASSOCIATED_TOKEN_PROGRAM_ID,
      TOKEN_PROGRAM_ID,
      mintA.publicKey,
      pool,
      true,
    );

    vaultB = await Token.getAssociatedTokenAddress(
      ASSOCIATED_TOKEN_PROGRAM_ID,
      TOKEN_PROGRAM_ID,
      mintB.publicKey,
      pool,
      true,
    );

    userTokenA = await mintA.createAssociatedTokenAccount(
      user.publicKey
    );

    userTokenB = await mintB.createAssociatedTokenAccount(
      user.publicKey
    );

    userTokenLp = await Token.getAssociatedTokenAddress(
      ASSOCIATED_TOKEN_PROGRAM_ID,
      TOKEN_PROGRAM_ID,
      mintLp,
      user.publicKey,
    );

    
  });

  it("Initialize", async () => {
    // await program.provider.connection.requestAirdrop(user.publicKey, 2e9);

    // Add your test here.
    const tx = await program.rpc.initialize(
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
    expect(poolState.k.toNumber()).to.equal(0);
  });

  it("Deposit", async () => {
    // await program.provider.connection.requestAirdrop(user.publicKey, 2e9);
    // const {
    //   // user,
    //   mintA,
    //   mintB,
    //   mintLp,
    //   vaultA,
    //   vaultB,
    //   userTokenA,
    //   userTokenB,
    //   userTokenLp,
    //   pool,
    //   otherPool,
    // } = await getAccounts(program.provider, program.programId);

    // // Add your test here.
    // let tx = await program.rpc.initialize(
    //   {
    //     accounts:{
    //       pool: pool,
    //       otherPool: otherPool,
    //       vaultA: vaultA,
    //       vaultB: vaultB,
    //       mintA: mintA.publicKey,
    //       mintB: mintB.publicKey,
    //       mintLp: mintLp,
    //       user: user.publicKey,
    //       tokenProgram: TOKEN_PROGRAM_ID,
    //       associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    //       systemProgram: web3.SystemProgram.programId,
    //       rent: web3.SYSVAR_RENT_PUBKEY,
    //     },
    //     signers: [ user ]
    //   }
    // );

    await mintA.mintTo(userTokenA, user, [], 1 * (10 ** 9));
    await mintB.mintTo(userTokenB, user, [], 1 * (10 ** 6));

    let amountA = 30000;
    let amountB = 200;
    let tx = await program.rpc.deposit(
      new anchor.BN(amountA),
      new anchor.BN(amountB),
      {
        accounts:{
          pool: pool,
          vaultA: vaultA,
          vaultB: vaultB,
          mintA: mintA.publicKey,
          mintB: mintB.publicKey,
          mintLp: mintLp,
          userTokenA: userTokenA,
          userTokenB: userTokenB,
          userTokenLp: userTokenLp,
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

    let vaultAState = await mintA.getAccountInfo(vaultA);
    // console.log("vaultA amount:", vaultAState.amount);
    let vaultBState = await mintB.getAccountInfo(vaultB);
    // console.log("vaultB amount:", vaultBState.amount);
    let userTokenLpState = (await program.provider.connection.getParsedTokenAccountsByOwner(user.publicKey, { mint: mintLp })).value[0].account.data.parsed.info;
    // console.log("userTokenLp amount:", userTokenLpState.tokenAmount.amount);

    expect(new anchor.BN(vaultAState.amount).toNumber()).to.equal(amountA);
    expect(new anchor.BN(vaultBState.amount).toNumber()).to.equal(amountB);
    expect(parseInt(userTokenLpState.tokenAmount.amount)).to.equal(amountA);

  });
});
