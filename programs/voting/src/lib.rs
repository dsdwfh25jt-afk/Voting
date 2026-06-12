use anchor_lang::prelude::*;

declare_id!("45cnbPi8gjEcvinQTmg2d9b4GtZ6yYN8gx4Jam9mNxdb");

#[program]
pub mod voting {
    use super::*;

    // function for adding data into poll account about poll ,
    // which poll acount is derived from seed and poll id ,
    // and the data coming from the client side  
    pub fn init_poll( 
        ctx: Context<InitPoll> ,
        _poll_id : u64,
        start : u64 ,
        end : u64 , 
        name : String , 
        description : String  
    ) -> Result<()> {
        // always take mutable referrence of the ctx.accounts 
        let  poll = &mut ctx.accounts.poll_account;
        poll.poll_description = description;
        poll.poll_name = name ;
        poll.poll_voting_start = start;
        poll.poll_voting_end = end ; 
        Ok(()) 
    }

    // initalizing cadidate 
    // doing the same way as poll account 
    // fiding updated data in candidate account 
    pub fn initialize_candidate ( 
        ctx: Context<InitializeCandidate>,
        _poll_idx : u64 ,
        candidate_name : String
     ) -> Result<()> {
        ctx.accounts.candidate_account.candidate_name = candidate_name ;
        ctx.accounts.poll_account.poll_index += 1;
        Ok(())
    }

    //creating voting funtion 
    // taking poll id and candidate name from the client 
    // 
    pub fn vote (
        ctx: Context<Vote>,
        _poll_id : u64,
        _candidate_name : String
    ) -> Result<()> {
        let candidate = &mut ctx.accounts.candidate_acccount;
        let current_time = Clock::get()?.unix_timestamp;

        if current_time >= (ctx.accounts.poll_account.poll_voting_end as i64 ) {
            return Err(ErrorCode::VotingEnded.into());
        }
        if current_time < (ctx.accounts.poll_account.poll_voting_start as i64 ) {
            return Err(ErrorCode::VotingNotStarted.into());
        }

        candidate.candidate_votes += 1; 
        Ok(())
    }
}

// creating candidate account from the user input poll_id and 
// candidate name 
// and validating before passing to the initialize function 
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

// validating the candidate account for vote and the poll account 
// to ensure that dedicated candidate account is voting for 
// dedecated poll account's poll 
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
    pub candidate_acccount : Account<'info , CandidateAccount>,
}

// validating / creating accounts for creating the poll 
// by the poll_id which is taken from the client  and 
// "poll" string to ensure this is poll account  
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

// Poll Account 
// storing information about poll 
// name of the poll eg vote for snacks 
// poll discription why this poll and what we are voting for 
// and what this vote poll for something like that info 
// when the voting will start : stored in the poll vote start 
// when the voting will end   : stroed in the poll vote end
#[account] 
#[derive(InitSpace)] 
pub struct PollAccount {
    #[max_len(32)]
    pub poll_name : String ,
    #[max_len(320)]
    pub poll_description : String,
    pub poll_voting_start : u64 ,
    pub poll_voting_end : u64,
    pub poll_index : u64 ,
}


// candidate account to store information about candidate 
// whom candidate voted or the name of the candidate 
// and for which poll id this perticular candidate has voted / of will vote 
#[account]
#[derive(InitSpace)] 
pub struct CandidateAccount {
    #[max_len(32)]
    pub candidate_name : String,
    pub candidate_votes : u64,
}
 

// Error types 
// to ensure user vote in the voting time of period 
// else it will get suitable warning and logical errors 
#[error_code]
pub enum ErrorCode {
    #[msg("Voting has not stated yet")]
    VotingNotStarted,
    #[msg("Voting has been ended")]
    VotingEnded,
} 