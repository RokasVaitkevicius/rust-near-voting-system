use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId};
use near_sdk::collections::{LookupMap, Vector};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Voter {
    pub account_id: AccountId,
    pub choice: String
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct VotingContract {
    // choices: Vector<String>,
    total_votes: u32,
    // vote_records: LookupMap<String, Voter>
}

#[near_bindgen]
impl VotingContract {
    #[init]
    pub fn init() -> Self {
        Self {
            // choices,
            total_votes: 0,
            // vote_records: LookupMap::new(b"v".to_vec())
        }
    }

    pub fn vote(&mut self, choice: String) {
        let account_id = env::signer_account_id();
        let voter = Voter {
            account_id: account_id.clone(),
            choice: choice.clone()
        };
        // self.vote_records.insert(&account_id.to_string(), &voter);
        self.total_votes += 1;
    }

    pub fn get_total_votes(&self) -> u32 {
        self.total_votes
    }
}

impl Default for VotingContract {
    fn default() -> Self {
        Self {
            // choices: Vector::new(b"c".to_vec()),
            total_votes: 0,
            // vote_records: LookupMap::new(b"v".to_vec())
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{VMContextBuilder};
    use near_sdk::{AccountId, testing_env};

    fn get_context() -> VMContextBuilder {
        let alice: AccountId = "alice.near".parse().unwrap();
        let bob: AccountId = "bob.near".parse().unwrap();

        let mut context = VMContextBuilder::new();

        context.current_account_id(alice);
        context.predecessor_account_id(bob);
        context
    }

    #[test]
    fn test_vote_and_get_votes() {
        let context = get_context().build();
        testing_env!(context);

        // let mut contract = VotingContract::init(vec!["Option A".to_string(), "Option B".to_string()]);
        let mut contract = VotingContract::init();

        // Cast votes
        contract.vote("Option A".to_string());
        contract.vote("Option B".to_string());
        contract.vote("Option A".to_string());

        // Check vote count
        assert_eq!(contract.get_total_votes(), 3);
    }
}
