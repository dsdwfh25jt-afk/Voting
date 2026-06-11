
use anchor_lang::prelude::*;
 
declare_id!("45cnbPi8gjEcvinQTmg2d9b4GtZ6yYN8gx4Jam9mNxdb");

#[program]
pub mod voting {
    use super::*;

    // creating poll  
    pub fn init_poll( 
        ctx: Context<InitPoll> ,
        poll_id : u64,
        start : u64 ,
        end : u64 , 
        name : String , 
        description : String  
    ) -> Result<()> {
        // always take mutable referrence of the ctx.accounts 
        let  poll = &mut ctx.accounts.poll_account;
        poll.pollDescription = description;
        poll.pollName = name ;
        poll.pollVotingStart ; start ;
        poll.pollVotingEnd = end ;
        Ok(()) 
    }

    // initalizing cadidate 
    pub fn initialize_candidate ( 
        ctx: Context<InitializeCandidate>,
        poll_idx : u64 ,
        candidate_name : String
     ) -> Result<()> {
        ctx.accounts.candidate_account.candidateName = candidate_name ;
        ctx.accounts.candidate_account.poll_index.checked_add(1);
        Ok(())
    }

    //creating voting funtion 
    pub fn vote (
        ctx: Context<Vote>,
        poll_id : u64,
        candidate_name : String
    ) -> Result<()> {
        let candidate = &mut ctx.accounts.candidate_acccount;

        let current_time = Clock::get()?.unix_timestamp;

        if current_time >= (ctx.accounts.poll_account.pollVotingEnd as i64 ) {
            return Err(ErrorCode::VotingEnded.into());
        }

        if current_time < (ctx.accounts.poll_account.pollVotingStart as i64 ) {
            return Err(ErrorCode::VotingNotStarted.into());
        }

        candidate.candidate_votes.checked_add(1); 
        Ok(())
    }


}

#[derive(Accounts)]
#[instruction(poll_id : u64 , candidate : String)]

pub struct InitializeCandidate <'info>{    
    #[account(mut)]
    pub signer : Signer<'info>,

    #[account(
        mut ,
        seeds = [b"poll".as_ref(),poll_id.to_le_bytes().as_ref()],
        bump 
    )]
    pub poll_account : Account<'info , PollAccount>,

    #[account(
        init ,
        payer = signer,
        space = 8 + CandidateAccount::INIT_SPACE,
        seeds = [poll_id.to_le_bytes().as_ref(), candidate.as_ref()],
        bump 
    )]
    pub candidate_account : Account<'info , CandidateAccount>,
    pub system_program : Program<'info, System> 

}

// deriving the accounts for vote function
#[derive(Accounts)]
#[instruction(poll_id : u64 , candidate : String)]
pub struct Vote <'info> {
    #[account(mut)]
    pub signer : Signer<'info>,

    #[account(
        mut ,
        seeds = [b"poll".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump 
    )]
    pub poll_account : Account <'info , PollAccount>,

    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref(),candidate.as_ref()],
        bump
    )]
    pub candidate_acccount : Account<'info , CandidateAccount>
 
}


#[derive(Accounts)]
#[instruction(poll_id : u64)]
pub struct InitPoll <'info>{

    #[account(mut)]
    pub signer : Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + PollAccount::INIT_SPACE ,
        seeds = [b"poll".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump 
    )]
    pub poll_account : Account<'info , PollAccount>,

    pub system_program : Program<'info , System>,
}


// Data state 
#[account] 
#[derive(InitSpace)] 
pub struct PollAccount {
    #[max_len(32)]
    pub pollName : String ,

    #[max_len(320)]
    pub pollDescription : String,

    pub pollVotingStart : u64 ,

    pub pollVotingEnd : u64,

}

#[account]
#[derive(InitSpace)] 
pub struct CandidateAccount {
    #[max_len(32)]
    pub candidateName : String,
    pub poll_index : u64,  
    pub candidate_votes : u64,
}
 

#[error_code]
pub enum ErrorCode {
    #[msg("Voting has not stated yet")]
    VotingNotStarted,
    #[msg("Voting has been ended")]
    VotingEnded,
} 