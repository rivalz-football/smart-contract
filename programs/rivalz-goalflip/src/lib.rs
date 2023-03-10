mod access_control;
mod errors;
pub mod randomness_tools;
pub mod recent_blockhashes;

use crate::errors::ErrorCode;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar;

declare_id!("rivi27uFE2UGJCR2WmzddviqS6RWiTzciC8KnVp2rhi");
const ADMIN_PUBKEY: &[u8] = b"aut69244nPQ5A23MKwScxMiZxvsYeepBkNuaxK2TqSd";

#[program]
pub mod rivalz_goalflip {
    use crate::access_control::is_admin;
    use super::*;

    #[access_control(is_admin(& ctx.accounts.admin))]
    pub fn init_game(ctx: Context<GameContext>) -> Result<()> {
        ctx.accounts.game.commission = 0;
        ctx.accounts.game.init_at = Clock::get()?.unix_timestamp as u64;
        ctx.accounts.game.multiplier = 2;
        ctx.accounts.game.name = b"myGame".to_vec();

        Ok(())
    }

    pub fn play(ctx: Context<PlayContext>, position: String, corner: String, bet_amount: u64) -> Result<()> {
        ctx.accounts.game.latest_match = ctx.accounts.game_match.key();
        ctx.accounts.game.last_play_date = Clock::get()?.unix_timestamp as u64;
        ctx.accounts.game.total_volume += bet_amount;


        ctx.accounts.game_match.position = match parse_position(&position) {
            Some(position) => position,
            None => return Err(ErrorCode::InvalidPosition.into()),
        };

        ctx.accounts.game_match.player_corner = match parse_corner(&corner) {
            Some(corner) => corner,
            None => return Err(ErrorCode::InvalidCorner.into()),
        };


        let randomness =
            recent_blockhashes::last_blockhash_accessor(&ctx.accounts.recent_blockhashes)?;

        // Select a random corner
        let random_corner: Corner = match randomness_tools::expand(randomness, 2) {
            0 => Corner::Left,
            _ => Corner::Right,
        };

        //balance check
        if ctx.accounts.player.lamports() < bet_amount {
            return err!(ErrorCode::NoEnoughFund);
        }
        ctx.accounts.game_match.won = random_corner != ctx.accounts.game_match.player_corner;

        if ctx.accounts.game.commission > 0 { // FEE
            **ctx
                .accounts
                .player
                .to_account_info()
                .try_borrow_mut_lamports()? -= ctx.accounts.game.commission;

            **ctx
                .accounts
                .game
                .to_account_info()
                .try_borrow_mut_lamports()? += ctx.accounts.game.commission;
        }


        if ctx.accounts.game_match.won {
            msg!("you won!");

            ctx.accounts.game.win_count += 1;
            ctx.accounts.game_match.won_amount = bet_amount * ctx.accounts.game.multiplier as u64;

            let transfer_amount = ctx.accounts.game_match.won_amount - bet_amount;

            **ctx
                .accounts
                .game
                .to_account_info()
                .try_borrow_mut_lamports()? -= transfer_amount;

            **ctx
                .accounts
                .player
                .to_account_info()
                .try_borrow_mut_lamports()? += transfer_amount;
        } else {
            msg!("you lost :(");

            ctx.accounts.game.lose_count += 1;
            **ctx
                .accounts
                .player
                .to_account_info()
                .try_borrow_mut_lamports()? -= bet_amount;

            **ctx
                .accounts
                .game
                .to_account_info()
                .try_borrow_mut_lamports()? += bet_amount;
        }


        Ok(())
    }
}

#[account]
pub struct GameMatch {
    pub game: Pubkey,
    //32
    pub player: Pubkey,
    //32
    pub won: bool,
    //1
    pub position: Position,
    //1
    pub player_corner: Corner,
    //1
    pub bet_amount: u64,
    //8
    pub won_amount: u64,
    //8
    pub match_date: u64,//8
}

#[account]
pub struct Game {
    pub multiplier: u8,
    // 1
    // lamport cinsinden
    pub commission: u64,
    // 8
    pub init_at: u64,
    // 8
    pub last_play_date: u64,
    //8
    pub lose_count: u64,
    //8
    pub win_count: u64,
    //8
    pub total_volume: u64,
    //32
    pub latest_match: Pubkey,
    //32
    //8
    pub name: Vec<u8>, //  30
}

#[derive(Accounts)]
pub struct PlayContext<'info> {
    #[account(
    init,
    payer = player,
    space = 8 + 64 + 27,
    )]
    pub game_match: Account<'info, GameMatch>,

    #[account(mut)]
    pub game: Account<'info, Game>,

    /// CHECK: ?
    #[account(mut)]
    player: Signer<'info>,
    /// CHECK: sysvar address check is hardcoded, we want to avoid the default deserialization
    #[account(address = sysvar::recent_blockhashes::ID)]
    pub recent_blockhashes: UncheckedAccount<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GameContext<'info> {
    #[account(
    init,
    payer = admin,
    space = 8 + 79 + 32,
    )]
    pub game: Account<'info, Game>,

    #[account(mut)]
    admin: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum Position {
    Goalkeeper,
    Forward,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum Corner {
    Left,
    Right,
}


fn parse_position(s: &str) -> Option<Position> {
    match s {
        "Goalkeeper" => Some(Position::Goalkeeper),
        "Forward" => Some(Position::Forward),
        _ => None,
    }
}

fn parse_corner(s: &str) -> Option<Corner> {
    match s {
        "Left" => Some(Corner::Left),
        "Right" => Some(Corner::Right),
        _ => None,
    }
}