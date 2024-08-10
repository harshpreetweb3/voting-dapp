use anchor_lang::prelude::*;

declare_id!("6iGpXVq6yoCRJG97RxDTpwkAoJnRPiiNzZEyRyJJNskt");

#[program]
pub mod voting_system {
    use super::*;

    pub const MAX_DESCRIPTION_SIZE: usize = 280; // Maximum characters allowed in the description
    
    pub fn create_proposal(ctx: Context<CreateProposal>, description: String, expiration: i64) -> Result<()> {

        let proposal = &mut ctx.accounts.proposal;
        //proposal account accessed mutably

        proposal.authority = ctx.accounts.authority.key();
        //propoosal account now has an owner which is a program

        proposal.description = description;
        //what the propsal is about?

        proposal.expiration = expiration;
        //expiration date

        proposal.yes_votes = 0;

        proposal.no_votes = 0;

        Ok(())
    }

    pub fn cast_vote(ctx: Context<CastVote>, vote: bool) -> Result<()> {

        let proposal = &mut ctx.accounts.proposal;

        let voter_info = &mut ctx.accounts.voter_info;
    
        // Check if the proposal has expired
        require!(Clock::get()?.unix_timestamp < proposal.expiration, VotingError::ProposalExpired);
    
        // Ensure the voter has not already voted
        require!(!voter_info.voted, VotingError::AlreadyVoted);
    
        // Cast the vote
        if vote {
            proposal.yes_votes += 1;
        } else {
            proposal.no_votes += 1;
        }
    
        // Mark that the voter has voted
        voter_info.voted = true;
    
        Ok(())
    }

    pub fn get_results(ctx: Context<GetResults>) -> Result<(u64, u64)> {
        
        let proposal = &ctx.accounts.proposal;
        
        // Return the current results as a tuple of (yes_votes, no_votes)
        Ok((proposal.yes_votes, proposal.no_votes))
    }
    

}

// proposal creation instruction uses below 3 accounts
#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 4 + MAX_DESCRIPTION_SIZE + 8 + 8 + 8)]
    //for account creation and maintainance, authority will pay
    //but who's authority?
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub authority: Signer<'info>, //who's owner of an authority account? // a program account
    pub system_program: Program<'info, System>, //program account
}

//proposal account data or can hold data defined in a Proposal Struct
//we do define what data proposal account is supposed to store
#[account]
pub struct Proposal {
    pub authority: Pubkey,
    pub description: String,
    pub expiration: i64,
    pub yes_votes: u64,
    pub no_votes: u64,
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut, has_one = authority)]
    pub proposal: Account<'info, Proposal>,                    //proposal account
    #[account(init, payer = authority, space = 8 + 1)]
    pub voter_info: Account<'info, VoterInfo>,                 //voter_info account is being created
    #[account(mut)]
    pub authority: Signer<'info>,                              //program's private key
    pub system_program: Program<'info, System>,                //program helps in voter_info account creation
}

#[account]
pub struct VoterInfo {
    pub voted: bool,     //vote information will be stored in an account right after voter_info account creation
}

#[error_code]
pub enum VotingError {
    #[msg("The proposal has already expired.")]
    ProposalExpired,
    #[msg("The voter has already cast a vote.")]
    AlreadyVoted,
}

#[derive(Accounts)]
pub struct GetResults<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
}




//create proposal account
//access proposal account mutably
//store proposal data in propoosal account

//why do we do with authority account (a 2nd account in instruction accounts)?
//created proposal account has no authority which can mean propoosal account has no owner or proposal account is owner-less
//we just give an owner to the proposal account

// who's the owner of proposal account? //Signer
// a program
// i believe our program which has it's own keypair, needs to store the proposal 
// and for storage purpose, program created an account where it will store the proposals

//Now i need to see how can i create a proposal 
//I can also go with writing a test for it 
//let's go and test (using ts) the instruction witten 