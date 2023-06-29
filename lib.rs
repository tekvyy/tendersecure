#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod tendersecure {

    use ink::prelude::string::String;
    use ink::env::hash::Keccak256;
    use ink::storage::{Mapping};
    use ink::prelude::vec::Vec;

    #[ink(storage)]
    pub struct Tendersecure {
        owner: AccountId,
        submit_proposal_phase_started: bool,
        bidders: Vec<AccountId>,
        bidder_proposals: Mapping<AccountId, String>,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        /// Returned if bidding is not started.
        BiddingNotStarted,
        /// Returned if caller is not owner while required to.
        CallerNotOwner,
        /// Returned if transfer failed.
        ErrorTransferringAmount,
        /// Returned if the bidder has already submitted a proposal.
        BidderAlreadySubmittedProposal,
        /// Returned if there are no entries.
        NoEntries,
    }

    #[ink(event)]
    pub struct ProposalSubmitted {
        bidder: AccountId,
        value: String,
    }

    #[ink(event)]
    pub struct Won {
        /// The winner.
        winner: AccountId,
        /// The winning amount.
        amount: Balance,
    }

    /// Type alias for the contract's result type.
    pub type Result<T> = core::result::Result<T, Error>;

    impl Tendersecure {
        /// Creates a new tender secure contract initialized.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Self::env().caller(),
                submit_proposal_phase_started: false,
                bidders: Vec::new(),
                bidder_proposals: Mapping::default(),
            }
        }

        /// Returns the owner of the Contract
        #[ink(message)]
        pub fn owner(&self) -> AccountId {
            self.owner
        }

        #[ink(message)]
        pub fn get_tender_amount(&self) -> Balance {
            self.env().balance()
        }

        #[ink(message)]
        pub fn can_submit_proposal(&self) -> bool {
            self.submit_proposal_phase_started
        }

        #[ink(message, payable)]
        pub fn submit_tender_amount(&self) -> Result<Balance> {
            if self.env().caller() != self.owner {
                return Err(Error::CallerNotOwner);
            }

            Ok(Self::env().balance())
        }

        /// Returns the list of bidders
        #[ink(message)]
        pub fn get_bidders(&self) -> Vec<AccountId> {
            self.bidders.clone()
        }

        /// Retrieve the balance of the account.
        #[ink(message)]
        pub fn get_proposal_for_bidder(&self, caller: AccountId) -> Option<String> {
            self.bidder_proposals.get(&caller)
        }

        #[ink(message, payable)]
        pub fn enter(&mut self, url:String) -> Result<()> {
            if !self.submit_proposal_phase_started {
                return Err(Error::BiddingNotStarted);
            }
            let caller = self.env().caller();

            self.bidders.push(caller);
            self.bidder_proposals.insert(caller, &url);

            self.env().emit_event(ProposalSubmitted {
                bidder: caller,
                value: url,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn pick_bidder(&mut self, winner_id: AccountId) -> Result<()> {
            if self.bidders.len() == 0 {
                return Err(Error::NoEntries);
            }

            let winner = winner_id;
            let amount: Balance = self.env().balance();

            if self.env().transfer(winner, amount).is_err() {
                return Err(Error::ErrorTransferringAmount);
            }

            for bidder in self.bidders.iter() {
                self.bidder_proposals.remove(bidder);
            }

            self.bidders = Vec::new();

            self.env().emit_event(Won { winner, amount });

            Ok(())
        }

        #[ink(message)]
        pub fn start_bidding_for_tender(&mut self) -> Result<()> {
            if self.env().caller() != self.owner {
                return Err(Error::CallerNotOwner);
            }
            self.submit_proposal_phase_started = true;

            Ok(())
        }

        #[ink(message)]
        pub fn stop_bidding_for_tender(&mut self) -> Result<()> {
            if self.env().caller() != self.owner {
                return Err(Error::CallerNotOwner);
            }
            self.submit_proposal_phase_started = false;

            Ok(())
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {

    }


    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {

    }
}
