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

  // return {
  //   user,
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
  // };

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

  it("Initialize", async () => {
    // await program.provider.connection.requestAirdrop(user.publicKey, 2e9);
    const {
      user,
      mintA,
      mintB,
      mintLp,
      vaultA,
      vaultB,
      pool,
      otherPool,
    } = await getAccounts(program.provider, program.programId);

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

  it("Deposit", async () => {
    console.log("GOT HERE");
    // await program.provider.connection.requestAirdrop(user.publicKey, 2e9);
    const {
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
    } = await getAccounts(program.provider, program.programId);

    console.log("GOT HERE");
    // Add your test here.
    const k = 200;
    let tx = await program.rpc.initialize(
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

    console.log("GOT HERE");
    await mintA.mintTo(userTokenA, user, [], 1 * (10 ** 9));
    await mintB.mintTo(userTokenB, user, [], 1 * (10 ** 6));
    console.log("GOT HERE");

    let amount = 300;
    // const depositIdx = Buffer.from(new Uint8Array([1]));
    // const amountBuffer = Buffer.from(new Uint8Array((new anchor.BN(amount)).toArray("le", 8)));
    // let depositIx = new TransactionInstruction({
    //   keys: [
    //     {
    //       pubkey: pool,
    //       isSigner: false,
    //       isWritable: false,
    //     },
    //     {
    //       pubkey: vaultA,
    //       isSigner: false,
    //       isWritable: true,
    //     },
    //     {
    //       pubkey: vaultB,
    //       isSigner: false,
    //       isWritable: true,
    //     },
    //     {
    //       pubkey: mintA.publicKey,
    //       isSigner: false,
    //       isWritable: false,
    //     },
    //     {
    //       pubkey: mintB.publicKey,
    //       isSigner: false,
    //       isWritable: false,
    //     },
    //     {
    //       pubkey: mintLp,
    //       isSigner: false,
    //       isWritable: true,
    //     },
    //     {
    //       pubkey: userTokenA,
    //       isSigner: false,
    //       isWritable: true,
    //     },
    //     {
    //       pubkey: userTokenB,
    //       isSigner: false,
    //       isWritable: true,
    //     },
    //     {
    //       pubkey: userTokenLp,
    //       isSigner: false,
    //       isWritable: true,
    //     },
    //     {
    //       pubkey: user.publicKey,
    //       isSigner: true,
    //       isWritable: true,
    //     },
    //     {
    //       pubkey: TOKEN_PROGRAM_ID,
    //       isSigner: false,
    //       isWritable: false,
    //     },
    //     {
    //       pubkey: ASSOCIATED_TOKEN_PROGRAM_ID,
    //       isSigner: false,
    //       isWritable: false,
    //     },
    //     {
    //       pubkey: web3.SYSVAR_RENT_PUBKEY,
    //       isSigner: false,
    //       isWritable: false,
    //     },
    //     {
    //       pubkey: web3.SystemProgram.programId,
    //       isSigner: false,
    //       isWritable: false,
    //     },
    //   ],
    //   programId: program.programId,
    //   data: Buffer.concat([
    //     depositIdx,
    //     amountBuffer,
    //   ]),
    // });
  
    // let depositTx = new Transaction();
    // depositTx.add(depositIx);
    // console.log("GOT HERE");

    // try {
    //   let depositTxid = await sendAndConfirmTransaction(
    //     program.provider.connection,
    //     depositTx,
    //     [user],
    //     {
    //       skipPreflight: true,
    //       preflightCommitment: "confirmed",
    //     }
    //   );
    //   console.log(`https://explorer.solana.com/tx/${depositTxid}?cluster=devnet`);
    // } catch (err) {
    //   console.log(err);
    // }

    tx = await program.rpc.deposit(
      new anchor.BN(amount),
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
    console.log("vaultA amount:", vaultAState.amount);
    let vaultBState = await mintB.getAccountInfo(vaultB);
    console.log("vaultB amount:", vaultBState.amount);
    // let userTokenLpState = await mintA.getAccountInfo(userTokenLp);
    // console.log("userTokenLp amount:", userTokenLpState.amount);
    let userTokenLpState = (await program.provider.connection.getParsedTokenAccountsByOwner(user.publicKey, { mint: mintLp })).value[0].account.data.parsed.info;
    console.log("userTokenLp amount:", userTokenLpState.tokenAmount.amount);

  });
});
