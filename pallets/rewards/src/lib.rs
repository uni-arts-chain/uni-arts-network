#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	decl_module, decl_storage, 
	decl_event, decl_error, 
	dispatch, ensure,
	Parameter,
	traits::{Currency, Get, FindAuthor},
	// weights::Weight,
};
use frame_system::ensure_signed;

use sp_runtime::{
	// RuntimeDebug, DispatchResult, DispatchError, RuntimeAppPublic,
	traits::{
		Zero, 
		// StaticLookup, CheckedAdd, CheckedSub,
		// Saturating, Bounded, IdentifyAccount,
		Saturating,
		AtLeast32BitUnsigned, Member, MaybeSerializeDeserialize, Convert
	}
};

use codec::Codec;
use sp_std::prelude::*;
// use frame_support::{debug};

pub type BalanceOf<T> =
	<<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;
pub type AccountId<T> = <T as frame_system::Trait>::AccountId;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: pallet_aura::Trait + pallet_session::Trait {
	// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

	type Balance: Parameter + Member + AtLeast32BitUnsigned + Codec + Default + Copy +
		MaybeSerializeDeserialize;

	type Currency: Currency<AccountId<Self>> + Send + Sync;
	type RewardPerBlock: Get<BalanceOf<Self>>;
	type RewardThreshold: Get<BalanceOf<Self>>;

	type AccoundIdOf: Convert<Self::ValidatorId, Option<AccountId<Self>>>;
}


// The pallet's runtime storage items.
decl_storage! {
	trait Store for Module<T: Trait> as Rewards {
		pub MatureRewards get(fn mature_rewards): map hasher(twox_64_concat) AccountId<T> => BalanceOf<T>;
		pub ImmatureRewards get(fn immature_rewards): map hasher(twox_64_concat) AccountId<T> => (BalanceOf<T>, T::BlockNumber);
	}
}


decl_event!(
	pub enum Event<T> 
		where 
			AccountId = AccountId<T>,
			Balance = BalanceOf<T>,
	{
		ClaimReward(AccountId, Balance),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		NoReward,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;
		fn deposit_event() = default;

		#[weight = 10_000]
		pub fn claim(origin) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;
			let mut rewards = Self::mature_rewards(&who);
			ensure!(rewards > Zero::zero(), Error::<T>::NoReward);

			<MatureRewards<T>>::mutate(&who, |val|
				*val = Zero::zero()
			);
			let now = frame_system::Module::<T>::block_number();
			let (immature_rewards, n) = Self::immature_rewards(&who);
			if now > n + <T as frame_system::Trait>::BlockNumber::from(14400 * 14) {
				rewards = rewards.saturating_add(immature_rewards);
			}

			Self::payout_rewards(who.clone(), rewards);
			Self::deposit_event(RawEvent::ClaimReward(who.clone(), rewards));

			Ok(())
		}

		fn on_finalize(now: T::BlockNumber) {
			let logs = frame_system::Module::<T>::digest().logs;
			let digest = logs.iter().filter_map(|s| s.as_pre_runtime());
			let reward: BalanceOf<T> = T::RewardPerBlock::get();
			let threshold: BalanceOf<T> = T::RewardThreshold::get();

			if let Some(index) = pallet_aura::Module::<T>::find_author(digest) {
				let validator = pallet_session::Module::<T>::validators()[index as usize].clone();
				if let Some(account) = T::AccoundIdOf::convert(validator) {

					let (immature_rewards, _) = Self::immature_rewards(&account);
					if immature_rewards < threshold {
						<ImmatureRewards<T>>::mutate(&account, |val|
							*val = (val.0.saturating_add(reward), now)
						);
					} else {
						<MatureRewards<T>>::mutate(&account, |rewards|
							*rewards = rewards.saturating_add(reward)
						);
					}
				}
			}
		}
	}
}


impl<T: Trait> Module<T> {
	fn payout_rewards(author: AccountId<T>, amount: BalanceOf<T>) {
		let _ = T::Currency::deposit_into_existing(&author, amount);
	}

	fn calc_reward_per_block() -> BalanceOf<T> {
		let initial = T::RewardPerBlock::get();
		return initial
	}
}
