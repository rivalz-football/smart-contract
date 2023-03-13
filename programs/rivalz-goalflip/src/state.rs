use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar;

#[account]
pub struct GameMatch {
    pub game: Pubkey,
    pub player: Pubkey,
    pub won: bool,
    pub position: Position,
    pub player_corner: Corner,
    pub bet_amount: u64,
    pub commission_amount: u64,
    pub won_amount: u64,
    pub created_at: u64,
    pub status: GameMatchStatus,
}

#[event]
pub struct ResultGameMatchEvent {
    pub game: Pubkey,
    pub player: Pubkey,
    pub won: bool,
    pub position: Position,
    pub player_corner: Corner,
    pub bet_amount: u64,
    pub commission_amount: u64,
    pub won_amount: u64,
}

#[account]
pub struct Game {
    pub multiplier: u16,
    pub commission_rate: u16,
    pub created_at: u64,
    pub last_play_date: u64,
    pub lose_count: u64,
    pub win_count: u64,
    pub total_volume: u64,
    pub latest_match: Pubkey,
    pub name: Vec<u8>,
}

#[derive(Accounts)]
pub struct PlayContext<'info> {
    #[account(
    init,
    payer = player,
    space = 8 + 105,
    )]
    pub game_match: Account<'info, GameMatch>,

    #[account(mut)]
    pub game: Account<'info, Game>,

    /// CHECK: ?
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ResultGameMatchContext<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,

    #[account(mut)]
    pub game_match: Account<'info, GameMatch>,

    /// CHECK: we will check this in the instruction
    #[account(mut)]
    pub player: AccountInfo<'info>,

    /// CHECK: we will check this in the instruction
    #[account(mut)]
    pub admin: Signer<'info>,

    /// CHECK: sysvar address check is hardcoded, we want to avoid the default deserialization
    #[account(address = sysvar::recent_blockhashes::ID)]
    pub recent_blockhashes: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GameContext<'info> {
    #[account(
    init,
    payer = admin,
    space = 8 + 92 + 1,
    )]
    pub game: Account<'info, Game>,

    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
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

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum GameMatchStatus {
    Pending,
    Won,
    Lost,
}

pub fn parse_position(s: &str) -> Option<Position> {
    match s {
        "Goalkeeper" => Some(Position::Goalkeeper),
        "Forward" => Some(Position::Forward),
        _ => None,
    }
}

pub fn parse_corner(s: &str) -> Option<Corner> {
    match s {
        "Left" => Some(Corner::Left),
        "Right" => Some(Corner::Right),
        _ => None,
    }
}
