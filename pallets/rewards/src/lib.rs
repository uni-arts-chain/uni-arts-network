#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	decl_module, decl_storage, 
	// decl_event, decl_error, dispatch, 
	Parameter,
	traits::{Currency, Get, FindAuthor},
	// weights::Weight,
};
// use frame_system::ensure_signed;

use sp_runtime::{
	// RuntimeDebug, DispatchResult, DispatchError, RuntimeAppPublic,
	traits::{
		// Zero, StaticLookup, CheckedAdd, CheckedSub,
		// Saturating, Bounded, IdentifyAccount,
		AtLeast32BitUnsigned, Member, MaybeSerializeDeserialize, Convert
	}
};

// use pallet_aura::AuraAuthorId;

use codec::Codec;
use sp_std::prelude::*;


pub type BalanceOf<T> =
	<<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;
pub type AccountId<T> = <T as frame_system::Trait>::AccountId;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: pallet_aura::Trait + pallet_session::Trait {
	// Because this pallet emits events, it depends on the runtime's definition of an event.
	// type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

	type Balance: Parameter + Member + AtLeast32BitUnsigned + Codec + Default + Copy +
		MaybeSerializeDeserialize;

	type Currency: Currency<AccountId<Self>> + Send + Sync;
	type RewardPerBlock: Get<BalanceOf<Self>>;

	type AccoundIdOf: Convert<Self::ValidatorId, Option<AccountId<Self>>>;
}


// The pallet's runtime storage items.
decl_storage! {
	trait Store for Module<T: Trait> as Rewards {
	}
}


// decl_event!(
// 	pub enum Event<T> 
// 		where 
// 			AccountId = <T as frame_system::Trait>::AccountId
// 	{
// 		Reward(AccountId),
// 	}
// );

// Errors inform users that something went wrong.
// decl_error! {
// 	pub enum Error for Module<T: Trait> {

// 	}
// }

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// type Error = Error<T>;
		// fn deposit_event() = default;

		fn on_finalize() {
			let logs = frame_system::Module::<T>::digest().logs;
			let digest = logs.iter().filter_map(|s| s.as_pre_runtime());
			let rewards: BalanceOf<T> = T::RewardPerBlock::get();
			if let Some(index) = pallet_aura::Module::<T>::find_author(digest) {
				let validator = pallet_session::Module::<T>::validators()[index as usize].clone();
				if let Some(account) = T::AccoundIdOf::convert(validator) {
					Self::payout_rewards(account, rewards);
				}
			}
		}
	}
}


impl<T: Trait> Module<T> {
	fn payout_rewards(author: AccountId<T>, amount: BalanceOf<T>) {
		let _ = T::Currency::deposit_into_existing(&author, amount);
	}
}
