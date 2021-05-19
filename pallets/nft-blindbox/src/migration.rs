use super::*;
use frame_support::weights::Weight;
use uniarts_primitives::CurrencyId;

#[derive(Encode, Decode, Default, Clone, PartialEq, RuntimeDebug)]
pub struct BlindboxItemV1<AccountId, BlockNumber> {
    pub id: u64,
    pub owner: AccountId,
    pub card_group: Vec<u64>,
    pub total_count: u64,
    pub remaind_count: u64,
    pub price: u64,
    pub start_time: BlockNumber,
    pub end_time: BlockNumber,
    pub has_ended: bool,
}

pub fn migrate_v1_to_t2<T: Trait>() -> Weight {
    if PalletStorageVersion::get() == StorageVersion::V1_0_0 {
        PalletStorageVersion::put(StorageVersion::V2_0_0);

        // Storage
        // pub BlindBoxList get(fn get_blind_box): map hasher(identity) u64 => BlindboxItem<T::AccountId, T::BlockNumber>;

        BlindBoxList::<T>::translate::<BlindboxItemV1<T::AccountId, T::BlockNumber>, _>(|_, p: BlindboxItemV1<T::AccountId, T::BlockNumber>|{
            let new_data: BlindboxItem<T::AccountId, T::BlockNumber> = BlindboxItem {
                id: p.id,
                owner: p.owner,
                card_group: p.card_group,
                currency_id: CurrencyId::default(),
                price: p.price,
                start_time: p.start_time,
                end_time: p.end_time,
                has_ended: p.has_ended,
                total_count: p.total_count,
                remaind_count: p.remaind_count
            };
            Some(new_data)
        });
        T::MaximumBlockWeight::get()
    } else {
        0
    }
}