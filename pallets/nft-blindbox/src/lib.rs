#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit="256"]
use codec::{Decode, Encode};
pub use frame_support::{
    construct_runtime, decl_event, decl_module, decl_storage, decl_error,
    dispatch::DispatchResult,
    ensure, parameter_types, Parameter,
    traits::{
        Currency, LockableCurrency, ExistenceRequirement, Get, Imbalance, KeyOwnerProofSystem, OnUnbalanced,
        Randomness, WithdrawReason, WithdrawReasons
    },
    weights::{
        constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
        DispatchInfo, GetDispatchInfo, IdentityFee, Pays, PostDispatchInfo, Weight,
        WeightToFeePolynomial,
    },
    IsSubType, StorageValue, debug,
};

use frame_system::{self as system, ensure_signed};
use sp_runtime::sp_std::prelude::Vec;
use sp_runtime::{
    ModuleId,
    traits::{AccountIdConversion}, RuntimeDebug,
};
use sp_std::prelude::*;
use module_support::NftManager;
use uniarts_primitives::CurrencyId;
use orml_traits::MultiCurrency;
use pallet_nft_multi as pallet_nft;

mod default_weight;
pub mod migration;

pub trait WeightInfo {
    fn create_blind_box() -> Weight;
    fn blind_box_add_card_group() -> Weight;
    fn blind_box_remove_card_group() -> Weight;
    fn buy_blind_box() -> Weight;
    fn close_blind_box() -> Weight;
    fn open_blind_box() -> Weight;
    fn cancel_blind_box() -> Weight;
}

/// Storage version.
#[derive(Encode, Decode, Eq, PartialEq)]
pub enum StorageVersion {
    /// Initial version.
    V1_0_0,
    /// multi-currency.
    V2_0_0,
}

impl Default for StorageVersion {
    fn default() -> Self {
        StorageVersion::V1_0_0
    }
}

#[derive(Encode, Decode, Default, Clone, PartialEq, RuntimeDebug)]
pub struct NftCard {
    pub group_id: u64,
    pub collection_id: u64,
    pub item_id: u64,
    pub value: u64,
    pub remaind_value: u64,
    pub draw_start: u64,
    pub draw_end: u64,
}

#[derive(Encode, Decode, Default, Clone, PartialEq, RuntimeDebug)]
pub struct BlindboxItem<AccountId, BlockNumber> {
    pub id: u64,
    pub owner: AccountId,
    pub card_group: Vec<u64>,
    pub total_count: u64,
    pub remaind_count: u64,
    pub currency_id: CurrencyId,
    pub price: u64,
    pub start_time: BlockNumber,
    pub end_time: BlockNumber,
    pub has_ended: bool,
}

pub trait Trait: system::Trait + pallet_nft::Trait {
    /// The NFT's module id, used for deriving its sovereign account ID.
    type LockModuleId: Get<ModuleId>;

    /// Nft manager.
    type NftHandler: NftManager<Self::AccountId, Self::BlockNumber>;

    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

    /// Something that provides randomness in the runtime.
    type Randomness: Randomness<Self::Hash>;

    /// Weight information for the extrinsics in this module.
    type WeightInfo: WeightInfo;

}

decl_storage! {
    trait Store for Module<T: Trait> as NftBlindBox {
        /// Next CardGroup id
        pub NextCardGroupID: u64 = 1;

        /// Next BlindBox id
        pub NextBlindBoxID: u64 = 1;

        /// Next Seed
        pub NextSeed: u32 = 1;

        /// CardGroup List
        pub CardGroupList get(fn get_card_group): map hasher(identity) u64 => NftCard;

        /// BlindBox List
        pub BlindBoxList get(fn get_blind_box): map hasher(identity) u64 => BlindboxItem<T::AccountId, T::BlockNumber>;

        /// Pallet Storage Version
        pub PalletStorageVersion get(fn pallet_storage_version)
			build(|_| StorageVersion::V2_0_0): StorageVersion = StorageVersion::V1_0_0;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
        CurrencyId = CurrencyId,
    {
        BlindBoxCreated(u64, u64, AccountId, CurrencyId),
        BlindBoxAddCardGroup(u64, u64, u64, u64, u64, AccountId),
        BlindBoxRemoveCardGroup(u64, u64, u64, u64, u64, AccountId),
        BlindBoxDraw(u64, u64, u64, u64, AccountId, AccountId, u64, CurrencyId),
        BlindBoxClose(u64, AccountId),
        BlindBoxOpen(u64, AccountId),
        BlindBoxCancel(u64, AccountId),
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {
        BlindBoxNotExists,
        BlindBoxNotInSalesPeriod,
        BlindBoxIsEnded,
        BlindBoxIsNotEnded,
        BlindBoxNotEnough,
        BlindBoxOnlyOwerBuy,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

        /// The NFT's module id, used for deriving its sovereign account ID.
		const ModuleId: ModuleId = T::LockModuleId::get();

        fn deposit_event() = default;

        fn on_runtime_upgrade() -> Weight {
			migration::migrate_v1_to_t2::<T>()
		}

        #[weight = <T as Trait>::WeightInfo::create_blind_box()]
        pub fn create_blind_box(origin, start_time: T::BlockNumber, end_time: T::BlockNumber, currency_id: CurrencyId, price: u64) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let blind_box = BlindboxItem {
                id: NextBlindBoxID::get(),
                owner: sender.clone(),
                card_group: vec![],
                currency_id: currency_id,
                price: price,
                start_time: start_time,
                end_time: end_time,
                has_ended: false,
                total_count: 0,
                remaind_count: 0
            };

            let blind_box_id = blind_box.id;
            <BlindBoxList<T>>::insert(blind_box_id, blind_box);
            NextBlindBoxID::mutate(|id| *id += 1);

            // call event
            Self::deposit_event(RawEvent::BlindBoxCreated(blind_box_id, price, sender, currency_id));
            Ok(())
        }

        #[weight = <T as Trait>::WeightInfo::blind_box_add_card_group()]
        pub fn blind_box_add_card_group(origin, blind_box_id: u64, collection_id: u64, item_id: u64, value: u64 ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let blind_box = Self::get_blind_box(blind_box_id);
            let mut card_value: u64 = value;

            let blind_box_owner = Self::is_blind_box_owner(sender.clone(), blind_box_id);
            if !blind_box_owner
            {
                let mes = "Account is not blind box owner";
                panic!(mes);
            }

            let target_collection = pallet_nft::Module::<T>::collection(collection_id);
            let locker = Self::nft_account_id();

            if let pallet_nft::CollectionMode::NFT(_) = target_collection.mode {
                card_value = 1;
            };

            match target_collection.mode
            {
                pallet_nft::CollectionMode::NFT(_) => T::NftHandler::transfer_nft(collection_id, item_id, sender.clone(), locker)?,
                pallet_nft::CollectionMode::Fungible(_)  => T::NftHandler::transfer_fungible(collection_id, item_id, value, sender.clone(), locker)?,
                pallet_nft::CollectionMode::ReFungible(_, _)  => T::NftHandler::transfer_refungible(collection_id, item_id, value, sender.clone(), locker)?,
                _ => ()
            };

            let blind_box_id = blind_box.id;
            let group_id = NextCardGroupID::get();
            let total_count: u64 = blind_box.total_count.checked_add(card_value).unwrap();
            let remaind_count: u64 = blind_box.remaind_count.checked_add(card_value).unwrap();
            let draw_start: u64 = blind_box.total_count.checked_add(1).unwrap();
            let draw_end: u64 = blind_box.total_count.checked_add(value).unwrap();

            let nft_card = NftCard {
                group_id: group_id,
                collection_id: collection_id,
                item_id: item_id,
                value: value,
                remaind_value: value,
                draw_start: draw_start,
                draw_end: draw_end,
            };

            CardGroupList::insert(group_id, nft_card.clone());
            NextCardGroupID::mutate(|id| *id += 1);

            let mut card_group = blind_box.clone().card_group;
            card_group.push(nft_card.group_id);

            <BlindBoxList<T>>::mutate(blind_box_id, |blind_box| {
                blind_box.card_group = card_group;
                blind_box.total_count = total_count;
                blind_box.remaind_count = remaind_count;
            });

            // call event
            Self::deposit_event(RawEvent::BlindBoxAddCardGroup(blind_box_id, group_id, collection_id, item_id, value, sender));
            Ok(())
        }

        #[weight = <T as Trait>::WeightInfo::blind_box_remove_card_group()]
        pub fn blind_box_remove_card_group(origin, blind_box_id: u64, card_group_id: u64) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let blind_box = Self::get_blind_box(blind_box_id);
            let card_group = Self::get_card_group(card_group_id);
            let collection_id: u64 = card_group.collection_id;
            let card_value: u64 = card_group.remaind_value;
            let total_count: u64 = blind_box.total_count.checked_sub(card_value).unwrap();
            let remaind_count: u64 = blind_box.remaind_count.checked_sub(card_value).unwrap();

            let blind_box_owner = Self::is_blind_box_owner(sender.clone(), blind_box_id);
            if !blind_box_owner
            {
                let mes = "Account is not blind box owner";
                panic!(mes);
            }

            let target_collection = pallet_nft::Module::<T>::collection(collection_id);
            let locker = Self::nft_account_id();
            if card_value > 0 {
                match target_collection.mode {
                    pallet_nft::CollectionMode::NFT(_) => T::NftHandler::transfer_nft(card_group.collection_id, card_group.item_id, locker.clone(), sender.clone())?,
                    pallet_nft::CollectionMode::Fungible(_)  => T::NftHandler::transfer_fungible(card_group.collection_id, card_group.item_id, card_value, locker.clone(), sender.clone())?,
                    pallet_nft::CollectionMode::ReFungible(_, _)  => T::NftHandler::transfer_refungible(card_group.collection_id, card_group.item_id, card_value, locker.clone(), sender.clone())?,
                    _ => ()
                };
                CardGroupList::remove(card_group_id);

                let mut card_group = blind_box.clone().card_group;

                let index = card_group.iter().position(|x| *x == card_group_id).unwrap();
                card_group.remove(index);
                <BlindBoxList<T>>::mutate(blind_box_id, |blind_box| {
                    blind_box.card_group = card_group;
                    blind_box.total_count = total_count;
                    blind_box.remaind_count = remaind_count;
                });
            }

            // call event
            Self::deposit_event(RawEvent::BlindBoxRemoveCardGroup(blind_box_id, card_group_id, collection_id, card_group.item_id, card_value, sender));
            Ok(())
        }

        #[weight = <T as Trait>::WeightInfo::buy_blind_box()]
        pub fn buy_blind_box(origin, blind_box_id: u64, receive: Option<T::AccountId>) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(blind_box_id > 0, Error::<T>::BlindBoxNotExists);

            let blind_box = Self::get_blind_box(blind_box_id);
            let currency_id = blind_box.currency_id;
            let now = <system::Module<T>>::block_number();
            ensure!(now >= blind_box.start_time, Error::<T>::BlindBoxNotInSalesPeriod);
            ensure!(now <= blind_box.end_time, Error::<T>::BlindBoxNotInSalesPeriod);
            ensure!(blind_box.has_ended == false, Error::<T>::BlindBoxIsEnded);
            ensure!(blind_box.remaind_count > 0, Error::<T>::BlindBoxNotEnough);
            if blind_box.price == 0 {
                ensure!(blind_box.owner == sender, Error::<T>::BlindBoxOnlyOwerBuy);
            }

            let mut receive_user = receive.clone().unwrap();
            if receive.is_none() {
                receive_user = sender.clone()
            }

            let mut winner_number = Self::get_winner_number(blind_box.total_count as u32);

            // debug::info!("------ winner_number {:?}", winner_number);

            // Two drawing modes are adopted, It's guaranteed to win
            // 1. Global mode: draw lots no matter whether there are cards sold or not
            // 2. On sale mode: draw lots from all cards on sale
            let mut mode1_choose_group_id: u64 = 0;
            let mut mode2_choose_group_id: u64 = 0;
            let mut mode2_card_group_ids: Vec<u64> = Vec::new();

            // mode1
            for card_group_id in blind_box.card_group.iter() {
                let card_group = Self::get_card_group(card_group_id);
                if winner_number >= card_group.draw_start && winner_number <= card_group.draw_end && card_group.remaind_value > 0 {
                    mode1_choose_group_id = card_group.group_id;
                }
                if card_group.remaind_value > 0 {
                    mode2_card_group_ids.push(card_group.group_id);
                }
            }

            // mode2
            if mode1_choose_group_id == 0 {
                let mut group_total: u64 = 0;
                winner_number = Self::get_winner_number(blind_box.remaind_count as u32);
                for card_group_id in mode2_card_group_ids.iter() {
                    let card_group = Self::get_card_group(card_group_id);
                    let group_start = group_total.checked_add(1).unwrap();
                    let group_end = group_total.checked_add(card_group.remaind_value).unwrap();
                    group_total = group_total.checked_add(card_group.remaind_value).unwrap();
                    if winner_number >= group_start && winner_number <= group_end {
                        mode2_choose_group_id = card_group_id.clone();
                        break;
                    }
                }
            }

            // send card
            // debug::info!("------ mode1_choose_group_id {:?}", mode1_choose_group_id);
            // debug::info!("------ mode2_choose_group_id {:?}", mode2_choose_group_id);

            let mut choose_group_id = mode1_choose_group_id;
            if mode1_choose_group_id == 0 {
                choose_group_id = mode2_choose_group_id;
            }
            let card_group = Self::get_card_group(choose_group_id);
            let locker = Self::nft_account_id();

            if blind_box.price > 0 {
                <T as pallet_nft::Trait>::MultiCurrency::transfer(currency_id, &sender, &blind_box.owner, blind_box.price.into())?;
            }

            let target_collection = pallet_nft::Module::<T>::collection(card_group.collection_id);
            match target_collection.mode
            {
                pallet_nft::CollectionMode::NFT(_) => T::NftHandler::transfer_nft(card_group.collection_id, card_group.item_id, locker, receive_user.clone())?,
                pallet_nft::CollectionMode::Fungible(_)  => T::NftHandler::transfer_fungible(card_group.collection_id, card_group.item_id, 1, locker, receive_user.clone())?,
                pallet_nft::CollectionMode::ReFungible(_, _)  => T::NftHandler::transfer_refungible(card_group.collection_id, card_group.item_id, 1, locker, receive_user.clone())?,
                _ => ()
            };

            CardGroupList::mutate(choose_group_id, |card_group| {
                card_group.remaind_value = card_group.remaind_value - 1;
            });

            <BlindBoxList<T>>::mutate(blind_box_id, |blind_box| {
                blind_box.remaind_count = blind_box.remaind_count - 1;
            });

            // call event
            Self::deposit_event(RawEvent::BlindBoxDraw(blind_box_id, choose_group_id, card_group.collection_id, card_group.item_id, receive_user, blind_box.owner, blind_box.price, currency_id));
            Ok(())
        }

        #[weight = <T as Trait>::WeightInfo::close_blind_box()]
        pub fn close_blind_box(origin, blind_box_id: u64) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let blind_box_owner = Self::is_blind_box_owner(sender.clone(), blind_box_id);
            if !blind_box_owner
            {
                let mes = "Account is not blind box owner";
                panic!(mes);
            }

            <BlindBoxList<T>>::mutate(blind_box_id, |blind_box| {
                blind_box.has_ended = true;
            });

            // call event
            Self::deposit_event(RawEvent::BlindBoxClose(blind_box_id, sender));
            Ok(())
        }

        #[weight = <T as Trait>::WeightInfo::open_blind_box()]
        pub fn open_blind_box(origin, blind_box_id: u64) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let blind_box_owner = Self::is_blind_box_owner(sender.clone(), blind_box_id);
            if !blind_box_owner
            {
                let mes = "Account is not blind box owner";
                panic!(mes);
            }

            <BlindBoxList<T>>::mutate(blind_box_id, |blind_box| {
                blind_box.has_ended = false;
            });

            // call event
            Self::deposit_event(RawEvent::BlindBoxOpen(blind_box_id, sender));
            Ok(())
        }

        #[weight = <T as Trait>::WeightInfo::cancel_blind_box()]
        pub fn cancel_blind_box(origin, blind_box_id: u64) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let blind_box_owner = Self::is_blind_box_owner(sender.clone(), blind_box_id);
            if !blind_box_owner
            {
                let mes = "Account is not blind box owner";
                panic!(mes);
            }
            let blind_box = Self::get_blind_box(blind_box_id);
            ensure!(blind_box.has_ended == true, Error::<T>::BlindBoxIsNotEnded);
            let locker = Self::nft_account_id();

            for card_group_id in blind_box.card_group.iter() {
                let card_group = Self::get_card_group(card_group_id);
                if card_group.remaind_value > 0 {
                    let target_collection = pallet_nft::Module::<T>::collection(card_group.collection_id);
                    match target_collection.mode {
                        pallet_nft::CollectionMode::NFT(_) => T::NftHandler::transfer_nft(card_group.collection_id, card_group.item_id, locker.clone(), sender.clone())?,
                        pallet_nft::CollectionMode::Fungible(_)  => T::NftHandler::transfer_fungible(card_group.collection_id, card_group.item_id, card_group.remaind_value, locker.clone(), sender.clone())?,
                        pallet_nft::CollectionMode::ReFungible(_, _)  => T::NftHandler::transfer_refungible(card_group.collection_id, card_group.item_id, card_group.remaind_value, locker.clone(), sender.clone())?,
                        _ => ()
                    };
                }
                CardGroupList::remove(card_group_id);
           }
           <BlindBoxList<T>>::remove(blind_box_id);

            // call event
            Self::deposit_event(RawEvent::BlindBoxCancel(blind_box_id, sender));
            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    /// The account ID of the NFT.
	///
	/// This actually does computation. If you need to keep using it, then make sure you cache the
	/// value and only call this once.
    pub fn nft_account_id() -> T::AccountId {
        T::LockModuleId::get().into_account()
    }

    fn is_blind_box_owner(owner: T::AccountId, blind_box_id: u64) -> bool {
        let target_blind_box = <BlindBoxList<T>>::get(blind_box_id);

        target_blind_box.owner == owner
    }

    fn get_winner_number(total_count: u32) -> u64 {
        let seed = NextSeed::get();
        let mut random_number = Self::generate_random_number(seed);
        NextSeed::mutate(|id| *id += 1);

        for i in 1 .. 20 {
            if random_number < u32::MAX - u32::MAX % total_count {
                break;
            }
            random_number = Self::generate_random_number(i);
        }
        let winner_number = (random_number % total_count + 1) as u64;
        winner_number
    }

    fn generate_random_number(seed: u32) -> u32 {
        let random_seed = T::Randomness::random(&(Self::nft_account_id(), seed).encode());
        let random_number = <u32>::decode(&mut random_seed.as_ref())
            .expect("secure hashes should always be bigger than u32; qed");
        random_number
    }
}