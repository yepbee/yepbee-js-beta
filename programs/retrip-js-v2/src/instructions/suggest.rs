use crate::{common::*, states::MainState};

#[derive(Accounts)]
pub struct Suggest<'info> {
    #[account(mut)]
    pub main_state: Account<'info, MainState>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

impl<'info> Suggest<'info> {
    /**
     * returns (finding_auth_lister, same_suggestions_count)
     * auth_lister == None -> the authority is not in the whitelist so suggestion failed
     */
    #[inline]
    fn go_around_and_suggest(&mut self, suggestion: &Suggestion) -> (Option<&mut Lister>, usize) {
        let authority_pubkey = self.authority.key();
        let whitelist = self.main_state.as_mut_whitelist();

        let mut majority_count = 1;
        let (mut find, mut index) = (false, 0);
        for (i, lister) in whitelist.listers.iter().enumerate() {
            if lister.pubkey == authority_pubkey {
                index = i;
                find = true;
                continue;
            }
            if lister.eq_suggestion(suggestion) {
                majority_count += 1;
            }
        }
        let finding_auth_lister = if find {
            // once
            whitelist.listers[index].suggest(suggestion.clone());
            Some(&mut whitelist.listers[index])
        } else {
            None
        };

        (finding_auth_lister, majority_count)
    }
    #[inline]
    pub fn run(&mut self, suggestion: Suggestion) -> Result<()> {
        let majority_number = self.main_state.as_mut_whitelist().majority_num();

        let (finding, majority_count) = self.go_around_and_suggest(&suggestion);
        let _ = finding.ok_or(Errors::NotInWhiteList)?;

        let is_passed = majority_count >= majority_number;

        let clear_when_the_suggest_succed = |white_list: &mut Whitelist| {
            for lister in white_list.listers.iter_mut() {
                if lister.eq_suggestion(&suggestion) {
                    lister.suggest(Suggestion::None);
                }
            }
        };

        let current_date = get_current_unix_timestamp_as_hour()?;

        // President Guard
        match &suggestion {
            Suggestion::ExecTransactions { txs: _ }
            | Suggestion::AddWhiteList { pubkeys: _ }
            | Suggestion::DelWhiteList { pubkeys: _ } => {
                if let Some(current_president) = self.main_state.as_mut_whitelist().president {
                    if current_president.expiration_date < current_date {
                        self.main_state.as_mut_whitelist().set_president(None);
                    } else {
                        return err!(Errors::PresidentDateNotOver);
                    }
                }
            }
            _ => {}
        }

        match &suggestion {
            Suggestion::VotePresident { vote } => {
                if let Some(voted_president) = vote {
                    self.main_state
                        .validate_whitelist_pubkey(&voted_president.pubkey)?;

                    // as hour
                    if voted_president.expiration_date <= current_date {
                        return err!(Errors::TooSmall);
                    }
                }
                if is_passed {
                    self.main_state
                        .as_mut_whitelist()
                        .set_president(vote.clone());
                    clear_when_the_suggest_succed(self.main_state.as_mut_whitelist());
                }
            }
            Suggestion::ExecTransactions { txs: _ } => {}
            Suggestion::AddWhiteList { pubkeys } => {
                require_unique_arrays!(
                    //
                    &pubkeys,
                    Errors::DuplicatedPubkeys
                );
                if is_passed {
                    self.main_state.as_mut_whitelist().push_keys(&pubkeys);
                    clear_when_the_suggest_succed(self.main_state.as_mut_whitelist());
                }
            }
            Suggestion::DelWhiteList { pubkeys } => {
                require_unique_arrays!(
                    //
                    &pubkeys,
                    Errors::DuplicatedPubkeys
                );
                if is_passed {
                    self.main_state.as_mut_whitelist().remove_keys(&pubkeys);
                    clear_when_the_suggest_succed(self.main_state.as_mut_whitelist());
                }
            }
            Suggestion::None => {} // already suggested on upper going around
        }

        Ok(())
    }
}
