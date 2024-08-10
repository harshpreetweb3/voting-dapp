use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
// use anchor_spl::token::accessor::authority;

declare_id!("6iGpXVq6yoCRJG97RxDTpwkAoJnRPiiNzZEyRyJJNskt");

#[program]
pub mod voting_app {
    use super::*;

    pub fn create_proposal(ctx: Context<CreateProposal>, description: String, expiration: i64) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        proposal.author = *ctx.accounts.author.key;
        proposal.description = description;
        proposal.expiration = expiration;
        proposal.yes_votes = 0;
        proposal.no_votes = 0;
        Ok(())
    }

    pub fn cast_vote(ctx: Context<CastVote>, vote: bool) -> Result<()> {

        let proposal = &mut ctx.accounts.proposal;

        let clock = Clock::get().unwrap();

        require!(clock.unix_timestamp < proposal.expiration, VotingError::VoteExpired); //best
        
        let voter_token_account = &ctx.accounts.voter_token_account;

        let voter_balance = voter_token_account.amount;
        
        require!(voter_balance > 0, VotingError::InsufficientTokens);
        
        if vote {
            proposal.yes_votes += voter_balance;
        } else {
            proposal.no_votes += voter_balance;
        }
        Ok(())
    }

    // pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    //     msg!("Greetings from: {:?}", ctx.program_id);
    //     Ok(())
    // }
}

//Proposal struct will be used to define the layout of data within a proposal account.

// #[account] Macro: This attribute macro provided by Anchor tells the compiler that the Proposal
// struct will be used to define the layout of data within a Solana account.
// In Solana, smart contracts interact with data stored in accounts, and
// each account can hold data structured according to custom definitions like Proposal.

// below data will be stored in a proposal account
#[account]
pub struct Proposal {
    author: Pubkey, // This field will store the public key of the user who initializes this proposal account.
    // (How many user can initiate the proposal account)
    // (can mutiple user initiate proposal account)
    description: String, //describes what the proposal is about?
    expiration: i64, //Unix timestamp at which the proposal will expire and no longer accept votes.
    yes_votes: u64,
    no_votes: u64,
}

// #[derive(Accounts)] //List of Accounts to be used by a particular instruction
// pub struct Initialize {}

//there must be a create_proposal instruction for which below accounts will be used
//this CreateProposal struct will be exposed to Context
#[derive(Accounts)] //validates and constrains accounts passed to instruction handler and deserialise them
pub struct CreateProposal<'info> {
    //'info -> lifetime (safety feature)

    //Accounts involved in a transaction

    //The 'info is a lifetime specifier that ensures all account references within the CreateProposal struct live
    //as long as the struct itself
    //It prevents dangling references and ensures data integrity during the smart contract's operation.
    //A "dangling reference" occurs when a reference points to a memory location of an account that has been deallocated or moved
    //ja location change ho gai ja location khali krwa le gai
    #[account(init, payer = author, space = 8 + 32 + 4 + 512 + 8 + 8 + 8)]
    // This defines the amount of space in bytes to allocate for the proposal account as data gets stored in an account in Solana
    // we gave or allocated space to proposal account for it's data
    pub proposal: Account<'info, Proposal>,
    //an account
    //init -> proposal account should be created (initialized) if does not exist

    //cost of allocating a space to newly created account will be paid by signer / author
    //transaction fees as well
    //pays rent as well
    #[account(mut)]
    pub author: Signer<'info>,
    //'info says account references (points to account memory location) must stay valid
    pub system_program: Program<'info, System>,
}

// 8: Typically used for storing an enum or identifier.
// 32: Commonly for public keys or addresses.
// 4: For storing small data types, such as an integer or status code.
// 512: Could be used for text or a larger data array.
// The additional 8 + 8 + 8: Usually for timestamps, counters, or additional identifiers.



// #[derive(Accounts)]
// pub struct CastVote<'info> {
//     #[account(mut)]
//     pub proposal: Account<'info, Proposal>,
//     #[account(mut)]
//     pub voter_token_account: Account<'info, TokenAccount>,
// }

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    /// CHECK: This is safe because we'll validate the account below
    #[account(mut)]
    pub voter_token_account: AccountInfo<'info>,
    #[account(constraint = token::accessor::authority(&voter_token_account)? == voter.key())]
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum VotingError {
    #[msg("The voting period for this proposal has expired.")]
    VoteExpired,
    #[msg("Insufficient tokens to cast a vote.")]
    InsufficientTokens,
}
