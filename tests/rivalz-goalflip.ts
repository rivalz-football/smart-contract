import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Connection, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { RivalzGoalflip } from "../target/types/rivalz_goalflip";

const connection = new Connection("http://127.0.0.1:8899", "confirmed");

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

describe("rivalz-goalflip", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.RivalzGoalflip as Program<RivalzGoalflip>;
  const wallet = anchor.getProvider();

  console.log(0.1 * LAMPORTS_PER_SOL);

  console.log(anchor.web3.SYSVAR_RECENT_BLOCKHASHES_PUBKEY);

  it("Is initialized!", async () => {
    const game = new anchor.web3.Keypair();
    const player = new anchor.web3.Keypair();

    //airdrop
    const airdropSignature = await connection.requestAirdrop(
      player.publicKey,
      LAMPORTS_PER_SOL
    );

    await connection.confirmTransaction(airdropSignature);

    const airdropSignature2 = await connection.requestAirdrop(
      game.publicKey,
      LAMPORTS_PER_SOL
    );

    await connection.confirmTransaction(airdropSignature2);

    await sleep(5000);

    const tx = await program.methods
      .play(
        {
          forward: {},
        },
        {
          topLeft: {},
        }
      )
      .accounts({
        authority: player.publicKey,
        game: game.publicKey,
        recentBlockhashes: anchor.web3.SYSVAR_RECENT_BLOCKHASHES_PUBKEY,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([game, player])
      .rpc();

    console.log(tx);
    const gameAccount = await program.account.game.fetch(game.publicKey);
    console.log("Your game account", gameAccount);
    // console.log("Your transaction signature", tx);
  });
});
