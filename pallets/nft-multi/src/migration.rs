use super::*;
use frame_support::weights::Weight;
use uniarts_primitives::CurrencyId;

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct SaleOrderV1<AccountId> {
    pub order_id: u64,
    pub collection_id: u64,
    pub item_id: u64,
    pub value: u64,
    pub owner: AccountId,
    pub price: u64, // maker order's price\
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct SplitSaleOrderV1<AccountId> {
    pub order_id: u64,
    pub collection_id: u64,
    pub item_id: u64,
    pub value: u64,
    pub balance: u64,
    pub owner: AccountId,
    pub price: u64, // maker order's price\
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct SaleOrderHistoryV1<AccountId, BlockNumber> {
    pub collection_id: u64,
    pub item_id: u64,
    pub value: u64,
    pub seller: AccountId,
    pub buyer: AccountId,
    pub price: u64,
    pub buy_time: BlockNumber,
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct AuctionV1<AccountId, BlockNumber> {
    pub id: u64,
    pub collection_id: u64,
    pub item_id: u64,
    pub value: u64,
    pub owner: AccountId,
    pub start_price: u64,
    pub increment: u64,
    pub current_price: u64,
    pub start_time: BlockNumber,
    pub end_time: BlockNumber,
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct BidHistoryV1<AccountId, BlockNumber> {
    pub auction_id: u64,
    pub bidder: AccountId,
    pub bid_price: u64,
    pub bid_time: BlockNumber,
}

pub fn migrate_v1_to_t2<T: Trait>() -> Weight {
    if PalletStorageVersion::get() == StorageVersion::V1_0_0 {
        PalletStorageVersion::put(StorageVersion::V2_0_0);

        // Storage
        // pub SaleOrderList get(fn nft_trade_id): double_map hasher(blake2_128_concat) u64, hasher(blake2_128_concat) u64 => SaleOrder<T::AccountId>;
        //
        // pub SaleOrderByIdList get(fn sale_order_id): map hasher(identity) u64 => SaleOrder<T::AccountId>;
        //
        // pub SeparableSaleOrder get(fn separablet_order_id): map hasher(identity) u64 => SplitSaleOrder<T::AccountId>;
        //
        // pub HistorySaleOrderList get(fn nft_trade_history_id): double_map hasher(blake2_128_concat) u64, hasher(blake2_128_concat) u64 => Vec<SaleOrderHistory<T::AccountId, T::BlockNumber>>;
        //
        // pub AuctionList get(fn get_auction): double_map hasher(blake2_128_concat) u64, hasher(blake2_128_concat) u64 => Auction<T::AccountId, T::BlockNumber>;
        //
        // pub BidHistoryList get(fn bid_history_list): map hasher(identity) u64 => Vec<BidHistory<T::AccountId, T::BlockNumber>>;
        SaleOrderList::<T>::translate::<SaleOrderV1<T::AccountId>, _>(|_, _, p: SaleOrderV1<T::AccountId>|{
            let new_data: SaleOrder<T::AccountId> = SaleOrder {
                order_id: p.order_id,
                collection_id: p.collection_id,
                item_id: p.item_id,
                currency_id: CurrencyId::default(),
                value: p.value,
                owner: p.owner,
                price: p.price,
            };
            Some(new_data)
        });

        SaleOrderByIdList::<T>::translate::<SaleOrderV1<T::AccountId>, _>(|_, p: SaleOrderV1<T::AccountId>|{
            let new_data: SaleOrder<T::AccountId> = SaleOrder {
                order_id: p.order_id,
                collection_id: p.collection_id,
                item_id: p.item_id,
                currency_id: CurrencyId::default(),
                value: p.value,
                owner: p.owner,
                price: p.price,
            };
            Some(new_data)
        });

        SeparableSaleOrder::<T>::translate::<SplitSaleOrderV1<T::AccountId>, _>(|_, p: SplitSaleOrderV1<T::AccountId>|{
            let new_data: SplitSaleOrder<T::AccountId> = SplitSaleOrder {
                order_id: p.order_id,
                collection_id: p.collection_id,
                item_id: p.item_id,
                currency_id: CurrencyId::default(),
                value: p.value,
                balance: p.balance,
                owner: p.owner,
                price: p.price,
            };
            Some(new_data)
        });

        AuctionList::<T>::translate::<AuctionV1<T::AccountId, T::BlockNumber>, _>(|_, _, p: AuctionV1<T::AccountId, T::BlockNumber>|{
            let new_data: Auction<T::AccountId, T::BlockNumber> = Auction {
                id: p.id,
                collection_id: p.collection_id,
                item_id: p.item_id,
                currency_id: CurrencyId::default(),
                value: p.value,
                owner: p.owner,
                start_price: p.start_price,
                current_price: p.current_price,
                increment: p.increment,
                start_time: p.start_time,
                end_time: p.end_time,
            };
            Some(new_data)
        });

        HistorySaleOrderList::<T>::translate::<Vec<SaleOrderHistoryV1<T::AccountId, T::BlockNumber>>, _>(|_, _, pslist| Some(
            pslist
                .into_iter()
                .map(|history| SaleOrderHistory {
                    collection_id: history.collection_id,
                    item_id: history.item_id,
                    currency_id: CurrencyId::default(),
                    value: history.value,
                    seller: history.seller,
                    buyer: history.buyer,
                    price: history.price,
                    buy_time: history.buy_time,
                })
                .collect::<Vec<_>>()
        ));

        BidHistoryList::<T>::translate::<Vec<BidHistoryV1<T::AccountId, T::BlockNumber>>, _>(|_, pslist| Some(
            pslist
                .into_iter()
                .map(|history| BidHistory {
                    auction_id: history.auction_id,
                    currency_id: CurrencyId::default(),
                    bidder: history.bidder,
                    bid_price: history.bid_price,
                    bid_time: history.bid_time,
                })
                .collect::<Vec<_>>()
        ));
        T::MaximumBlockWeight::get()
    } else {
        0
    }
}