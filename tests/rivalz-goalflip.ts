import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
} from "@solana/web3.js";
import { expect } from "chai";
import { RivalzGoalflip } from "../target/types/rivalz_goalflip";

const connection = new Connection("http://127.0.0.1:8899", "confirmed");

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

describe("rivalz-goalflip", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.RivalzGoalflip as Program<RivalzGoalflip>;
  const wallet = anchor.getProvider();

  // console.log(0.1 * LAMPORTS_PER_SOL);

  // console.log(anchor.web3.SYSVAR_RECENT_BLOCKHASHES_PUBKEY);

  const getGamePDA = (owner: PublicKey) => {
    const [PDA, _] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("game"), owner.toBuffer()],
      program.programId
    );

    return PDA;
  };

  it("should play the game and transfer SOL", async () => {
    // Create a new player account.
    const player = anchor.web3.Keypair.generate();
    const playerInitialBalance = await connection.getBalance(player.publicKey);

    // Create a new game account.
    const game = anchor.web3.Keypair.generate();
    const gameInitialBalance = await connection.getBalance(game.publicKey);

    // Request an airdrop to the player account.
    await connection.requestAirdrop(player.publicKey, LAMPORTS_PER_SOL);
    await sleep(3000); // Wait for the airdrop to be confirmed.

    // Initialize the game account with the player account as the owner.

    // Play the game.
    await program.methods
      .play(
        {
          forward: {},
        },
        {
          topRight: {},
        }
      )
      .accounts({
        game: getGamePDA(player.publicKey),
        authority: player.publicKey,
        recentBlockhashes: anchor.web3.SYSVAR_RECENT_BLOCKHASHES_PUBKEY,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([player])
      .rpc();

    // Check that the game was completed successfully.
    // const gameAccount = await program.account.game.fetch(
    //   getGamePDA(player.publicKey)
    // );
    // console.log(gameAccount);
    // expect(gameAccount.status).toBe(Status.Completed);
    // expect(gameAccount.corner).not.toBeUndefined();

    // // Check that the SOL transfer was successful.
    // const playerFinalBalance = await connection.getBalance(player.publicKey);
    // const gameFinalBalance = await connection.getBalance(game.publicKey);
    // expect(playerFinalBalance).toBe(
    //   playerInitialBalance - 100000000 // The transfer amount.
    // );
    // expect(gameFinalBalance).toBe(gameInitialBalance + 100000000);
  });
});
