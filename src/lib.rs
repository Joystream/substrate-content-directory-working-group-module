// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
//use serde_derive::{Deserialize, Serialize};
use rstd::prelude::*;

use codec::{Codec, Decode, Encode};
use runtime_primitives::traits::{MaybeSerializeDebug, Member, One, SimpleArithmetic};
use srml_support::traits::Currency;
use srml_support::{
    decl_module, decl_storage, ensure, EnumerableStorageMap, Parameter, StorageMap, StorageValue,
};
use std::iter::Iterator;

use runtime_primitives::traits::Zero;

//use crate::sr_api_hidden_includes_decl_storage::hidden_include::traits::Imbalance;

use rstd::collections::btree_map::BTreeMap;
use rstd::collections::btree_set::BTreeSet;

mod types;
mod macroes;
mod mock;
mod test;

use hiring::*;
use system;

use stake;

pub trait Trait: system::Trait + stake::Trait + Sized {

    // RecurringReward::Trait + Hiring::Trait + Membership::Trait VersionedStorePermissions::Trait

    type OpeningId: Parameter
        + Member
        + SimpleArithmetic
        + Codec
        + Default
        + Copy
        + MaybeSerializeDebug
        + PartialEq;

    type ApplicationId: Parameter
        + Member
        + SimpleArithmetic
        + Codec
        + Default
        + Copy
        + MaybeSerializeDebug
        + PartialEq;
}

pub type BalanceOf<T> =
    <<T as stake::Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;

pub type NegativeImbalance<T> =
    <<T as stake::Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::NegativeImbalance;

decl_storage! {
    trait Store for Module<T: Trait> as ContentWorkingGroup {

/*
        /// Openings.
        pub OpeningById get(opening_by_id): linked_map T::OpeningId => Opening<BalanceOf<T>, T::BlockNumber, T::ApplicationId>;

        /// Identifier for next opening to be added.
        pub NextOpeningId get(next_opening_id): T::OpeningId;

        /// Applications
        pub ApplicationById get(application_by_id): linked_map T::ApplicationId => Application<T::OpeningId, T::BlockNumber, T::StakeId>;

        /// Identifier for next application to be added.
        pub NextApplicationId get(next_application_id): T::ApplicationId;

        /// Internal purpose of given stake, i.e. fro what application, and whether for the role or for the application.
        pub ApplicationIdByStakingId get(stake_purpose_by_staking_id): linked_map T::StakeId => T::ApplicationId;
    */

    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn on_finalize(now: T::BlockNumber) {
        }
    }
}

impl<T: Trait> Module<T> {

    /*
    pub fn add_opening(
        activate_at: ActivateOpeningAt<T>,
        max_review_period_length: T::BlockNumber,
        application_rationing_policy: Option<ApplicationRationingPolicy>,
        application_staking_policy: Option<StakingPolicy<BalanceOf<T>, T::BlockNumber>>,
        role_staking_policy: Option<StakingPolicy<BalanceOf<T>, T::BlockNumber>>,
        human_readable_text: Vec<u8>,
    ) -> Result<T::OpeningId, AddOpeningError> {
        let current_block_height = <system::Module<T>>::block_number();

        // Check that exact activation is actually in the future
        ensure!(
            match activate_at {
                ActivateOpeningAt::ExactBlock(block_number) => block_number > current_block_height,
                _ => true,
            },
            AddOpeningError::OpeningMustActivateInTheFuture
        );

        // Check that staking amounts clear minimum balance required.
        ensure_amount_valid_in_opt_staking_policy!(
            T,
            application_staking_policy,
            AddOpeningError::StakeAmountLessThanMinimumCurrencyBalance(StakePurpose::Application)
        )?;

        ensure_amount_valid_in_opt_staking_policy!(
            T,
            role_staking_policy,
            AddOpeningError::StakeAmountLessThanMinimumCurrencyBalance(StakePurpose::Role)
        )?;

        //
        // == MUTATION SAFE ==
        //

        // Construct new opening
        let opening_stage = match activate_at {
            ActivateOpeningAt::CurrentBlock => hiring::OpeningStage::Active {
                // We immediately start accepting applications
                stage: hiring::ActiveOpeningStage::AcceptingApplications {
                    started_accepting_applicants_at_block: current_block_height,
                },

                // Empty set of applicants
                applicants: BTreeSet::new(), // Map::new(),

                // All counters set to 0
                active_application_count: 0,
                unstaking_application_count: 0,
                deactivated_application_count: 0,
            },

            ActivateOpeningAt::ExactBlock(block_number) => hiring::OpeningStage::WaitingToBegin {
                begins_at_block: block_number,
            },
        };

        let new_opening = hiring::Opening {
            created: current_block_height,
            stage: opening_stage,
            max_review_period_length: max_review_period_length,
            application_rationing_policy: application_rationing_policy,
            application_staking_policy: application_staking_policy,
            role_staking_policy: role_staking_policy,
            human_readable_text: human_readable_text,
        };

        // Get Id for new opening
        let new_opening_id = <NextOpeningId<T>>::get();

        // Insert opening in storage
        <OpeningById<T>>::insert(new_opening_id, new_opening);

        // Update NextOpeningId counter
        <NextOpeningId<T>>::mutate(|id| *id += T::OpeningId::from(1));

        // Return
        Ok(new_opening_id)
    }
    */

}

/*
 *  ======== ======== ======== ======== =======
 *  ======== PRIVATE TYPES AND METHODS ========
 *  ======== ======== ======== ======== =======
 */

impl<T: Trait> Module<T> {
    
}