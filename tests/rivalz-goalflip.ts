import * as anchor from "@project-serum/anchor";
import { ACCOUNT_DISCRIMINATOR_SIZE, Program } from "@project-serum/anchor";
import {
  Connection,
  GetProgramAccountsFilter,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
} from "@solana/web3.js";
import { expect } from "chai";
import { RivalzGoalflip } from "../target/types/rivalz_goalflip";

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

//anchor enum convert
enum Position {
  Goalkeeper = "Goalkeeper",
  Forward = "Forward",
}

enum Corner {
  TopLeft = "Left",
  TopRight = "Right",
}

const isWon = (gameMatch) =>
  (gameMatch.position === Position.Forward &&
    gameMatch.corner === Corner.TopRight) ||
  (gameMatch.position === Position.Goalkeeper &&
    gameMatch.corner === Corner.TopLeft);

describe("rivalz-goalflip", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.RivalzGoalflip as Program<RivalzGoalflip>;

  const wallet = anchor.getProvider();
  // Initialize a new player account.
  // pLyDRcM2xSBoWYkfRLzwwBCg4unu6g8syqjuoTpGqEJ.json
  const player = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array([
      238, 239, 97, 218, 164, 9, 95, 26, 133, 227, 252, 103, 91, 248, 139, 226,
      137, 45, 169, 52, 192, 110, 86, 13, 48, 64, 92, 49, 151, 251, 152, 224,
      12, 32, 224, 149, 189, 249, 171, 92, 64, 166, 67, 68, 93, 116, 209, 143,
      250, 156, 162, 14, 184, 220, 30, 255, 41, 11, 26, 33, 190, 5, 229, 203,
    ])
  );

  // Create a new game account.
  const game = anchor.web3.Keypair.generate();
  const gameMatch = anchor.web3.Keypair.generate();

  it("initialize the game", async () => {
    return;
    try {
      const initGameContext = {
        game: game.publicKey.toBase58(),
        admin: wallet.publicKey.toBase58(),
        systemProgram: anchor.web3.SystemProgram.programId.toBase58(),
      };
      console.log("init game", initGameContext);
      const tx = await program.methods
        .initGame()
        .accounts(initGameContext)
        .signers([game])
        .rpc();
      console.log("Game initialized  ", game.publicKey.toBase58(), tx);
    } catch (e) {
      console.log(e);
      throw e;
    }
  });

  it("should play the game", async () => {
    try {
      const playContext = {
        game: "5riLkKTNTLz28ByDG4RCvfmf7qBff2wyefSzbMSRPm6T",
        player: player.publicKey.toBase58(),
        gameMatch: gameMatch.publicKey.toBase58(),
        systemProgram: anchor.web3.SystemProgram.programId,
      };

      console.log({
        playContext,
      });

      const tx = await program.methods
        .play(
          Position.Forward,
          Corner.TopLeft,
          new anchor.BN(LAMPORTS_PER_SOL / 2)
        )
        .accounts(playContext)
        .signers([gameMatch, player])
        .rpc();

      const gameMatchInfo = await program.account.gameMatch.fetch(
        gameMatch.publicKey
      );

      console.log(gameMatchInfo.betAmount.toNumber() / LAMPORTS_PER_SOL);
      console.log("Game Match:  ", gameMatch.publicKey.toBase58(), tx);
    } catch (e) {
      console.log(e);
      throw e;
    }
  });

  it("should result the game match", async () => {
    try {
      program.addEventListener("ResultGameMatchEvent", (event) => {
        console.log("ResultGameMatch", event);
      });

      //find the player from gamematch here
      let playedGameMatch = await program.account.gameMatch.fetch(
        gameMatch.publicKey.toBase58()
      );

      const resultGameContext = {
        game: "5riLkKTNTLz28ByDG4RCvfmf7qBff2wyefSzbMSRPm6T",
        gameMatch: gameMatch.publicKey.toBase58(),
        player: playedGameMatch.player.toBase58(),
        admin: wallet.publicKey.toBase58(),
        recentBlockhashes: anchor.web3.SYSVAR_RECENT_BLOCKHASHES_PUBKEY,
        systemProgram: anchor.web3.SystemProgram.programId,
      };

      console.log({ resultGameContext });

      const result_match = await program.methods
        .resultGameMatch()
        .accounts(resultGameContext)
        .rpc();

      console.log(
        "Game Match Resulted:  ",
        gameMatch.publicKey.toBase58(),
        result_match
      );
    } catch (e) {
      console.log(e);
      throw e;
    }
  });

  it("should history the game matches", async () => {
    const filters: GetProgramAccountsFilter[] = [
      {
        memcmp: {
          offset: ACCOUNT_DISCRIMINATOR_SIZE,
          bytes: "5riLkKTNTLz28ByDG4RCvfmf7qBff2wyefSzbMSRPm6T",
        },
      },
    ];

    const prefetchedList = await program.account.gameMatch.all(filters);

    console.log(prefetchedList);
  });
});
