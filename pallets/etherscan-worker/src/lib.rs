#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::{
	self as system,
	offchain::{
		AppCrypto, CreateSignedTransaction,
	}
};
use frame_support::{
	debug, decl_module, decl_storage, decl_event,
	traits::Get,
};
use sp_core::crypto::KeyTypeId;
use sp_runtime::{
	transaction_validity::{
		ValidTransaction, TransactionValidity, TransactionSource,
		TransactionPriority,
	},
	offchain::{http},
};
use sp_runtime::{traits::{Hash}};
use ethereum_types::{H64, H128, H160, U256, H256, H512};

#[derive(Encode, Decode)]
pub struct RpcUrl {
	url: Vec<u8>,
}

///information about a erc20 transfer.
#[derive(Clone, Encode, Decode)]
pub struct TransferInfo {
	pub block_number: U256,
	pub time_stamp: u128,
	pub tx_hash: H256,
	pub nonce: u16,
	pub block_hash: H256,
	pub from_address: H160,
	pub to_address: H160,
	pub contract_address: H160,
	pub value: U256,
	pub token_name: Vec<u8>,
	pub token_symbol: Vec<u8>,
	pub transaction_index: u8,
	pub gas: U256,
	pub gas_price: U256,
	pub gas_used: U256,
	pub cumulative_gas_used: U256,
	pub input: Vec<u8>,
	pub confirmations: U256,
}

/// This pallet's configuration trait
pub trait Trait: CreateSignedTransaction<Call<Self>> {
	/// The identifier type for an offchain worker.
	type AuthorityId: AppCrypto<Self::Public, Self::Signature>;

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
	/// The overarching dispatch call type.
	type Call: From<Call<Self>>;

	// Configuration parameters

	/// A grace period after we send transaction.
	///
	/// To avoid sending too many transactions, we only attempt to send one
	/// every `GRACE_PERIOD` blocks. We use Local Storage to coordinate
	/// sending between distinct runs of this offchain worker.
	type GracePeriod: Get<Self::BlockNumber>;

	/// Number of blocks of cooldown after unsigned transaction is included.
	///
	/// This ensures that we only accept unsigned transactions once, every `UnsignedInterval` blocks.
	type UnsignedInterval: Get<Self::BlockNumber>;

	/// A configuration for base priority of unsigned transactions.
	///
	/// This is exposed so that it can be tuned for particular runtime, when
	/// multiple pallets send unsigned transactions.
	type UnsignedPriority: Get<TransactionPriority>;
}

decl_storage! {
	trait Store for Module<T: Trait> as EtherscanWorkerModule {
		/// Current synchronization block height.
		pub SyncBlockNumber get(fn sync_block_number): Option<U256>;

		/// Ethereum Erc20 Token Name
		pub Erc20TokenName get(fn erc20_token_name): Option<Vec<u8>>;

		/// Ethereum Erc20 Token Address
		pub Erc20TokenAddress get(fn erc20_token_address): Option<H160>;

		/// Mapping Token hash
		pub MappingTokenHash get(fn mapping_token_hash): Option<Hash>;

		/// Start synchronization block height
		pub SyncBeginBlockHeight get(fn sync_begin_block_heigh): Option<U256>;

		/// We store block erc20 transfer tx hash
		pub BlockTransfers get(fn block_number_transfers): map hasher(blake2_128_concat) U256 => H160;

		/// We store full information about the erc20 transfer
		pub Erc20TransferList get(fn transfer_id): double_map hasher(blake2_128_concat) H256, hasher(blake2_128_concat) u64 => TransferInfo;

		/// All erc20 transfer information in a transaction
		pub AllTransferByTxHash get(fn all_transfer): map hasher(twox_64_concat) U256 => Vec<TransferInfo>;

		/// RpcUrls set by anyone
		pub RpcUrls get(fn rpc_urls): map hasher(twox_64_concat) T::AccountId => Option<RpcUrl>;

	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		NewHeader(u32, AccountId),
	}
);

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// // Errors must be initialized if they are used by the pallet.
		// type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		#[weight = 0]
		fn init(
			origin,
			erc20_token_name: Vec<u8>,
			erc20_token_address: H160,
			mapping_token_hash: Hash,
			sync_begin_block_heigh: U256,
			rpc_urls: RpcUrl,
		) {
			let _signer = ensure_signed(origin)?;
			ensure!(Self::erc20_token_name().is_none(), "Already initialized");
			ensure!(Self::erc20_token_address().is_none(), "Already initialized");
			ensure!(Self::mapping_token_hash().is_none(), "Already initialized");
			ensure!(Self::sync_begin_block_heigh().is_none(), "Already initialized");
			ensure!(Self::rpc_urls().is_none(), "Already initialized");


			<Erc20TokenName>::set(Some(erc20_token_name));
			<Erc20TokenAddress>::set(Some(erc20_token_address));
			<MappingTokenHash>::set(Some(mapping_token_hash));
			<SyncBeginBlockHeight>::set(Some(sync_begin_block_heigh));
			<RpcUrls>::set(Some(rpc_urls));
		}

		/// Offchain Worker entry point.
		///
		/// By implementing `fn offchain_worker` within `decl_module!` you declare a new offchain
		/// worker.
		/// This function will be called when the node is fully synced and a new best block is
		/// succesfuly imported.
		/// Note that it's not guaranteed for offchain workers to run on EVERY block, there might
		/// be cases where some blocks are skipped, or for some the worker runs twice (re-orgs),
		/// so the code should be able to handle that.
		/// You can use `Local Storage` API to coordinate runs of the worker.
		fn offchain_worker(block_number: T::BlockNumber) {
			// It's a good idea to add logs to your offchain workers.
			// Using the `frame_support::debug` module you have access to the same API exposed by
			// the `log` crate.
			// Note that having logs compiled to WASM may cause the size of the blob to increase
			// significantly. You can use `RuntimeDebug` custom derive to hide details of the types
			// in WASM or use `debug::native` namespace to produce logs only when the worker is
			// running natively.
			debug::native::info!("Hello World from offchain workers!");

			// Since off-chain workers are just part of the runtime code, they have direct access
			// to the storage and other included pallets.
			//
			// We can easily import `frame_system` and retrieve a block hash of the parent block.
			let parent_hash = <system::Module<T>>::block_hash(block_number - 1.into());
			debug::debug!("Current block: {:?} (parent hash: {:?})", block_number, parent_hash);
		}
	}
}

impl<T: Trait> Module<T> {
	fn fetch_block_header(block_number: U256) -> Result<types::BlockHeader, http::Error> {
		// Make a post request to etherscan
		let url = format!("https://api-cn.etherscan.com/api?module=account&action=tokentx&contractaddress=0x9f8f72aa9304c8b593d555f12ef6589cc3a579a2&startblock={}&endblock={}&sort=asc&apikey={}", block_number, block_number, "YourApiKeyToken");
		let request: http::Request = http::Request::get(url);
		let pending = request.send().unwrap();

		// wait indefinitely for response (TODO: timeout)
		let mut response = pending.wait().unwrap();
		let headers = response.headers().into_iter();
		assert_eq!(headers.current(), None);

		// and collect the body
		let body = response.body().collect::<Vec<u8>>();
		let body_str = sp_std::str::from_utf8(&body).map_err(|_| {
			debug::warn!("No UTF8 body");
			http::Error::Unknown
		}).unwrap();
		// decode JSON into object
		let val: JsonValue = lite_json::parse_json(&body_str).unwrap();
		let header: Vec<TransferInfo> = Self::json_to_rlp(val);
		Ok(header)
	}

	pub fn json_to_rlp(json: JsonValue) -> Vec<TransferInfo> {
		// get { "result": VAL }
		let transfers = vec!();
		let transfer: Option<Vec<(Vec<char>, JsonValue)>> = match json {
			JsonValue::Object(obj) => {
				obj.into_iter()
					.find(|(k, _)| k.iter().map(|c| *c as u8).collect::<Vec<u8>>() == b"result".to_vec())
					.and_then(|v| {
						match v.1 {
							JsonValue::Object(transfer) => Some(transfer),
							_ => None,
						}
					})
			},
			_ => None
		};

		transfers
	}
}

#[allow(deprecated)] // ValidateUnsigned
impl<T: Trait> frame_support::unsigned::ValidateUnsigned for Module<T> {
	type Call = Call<T>;

	/// Validate unsigned call to this module.
	///
	/// By default unsigned transactions are disallowed, but implementing the validator
	/// here we make sure that some particular calls (the ones produced by offchain worker)
	/// are being whitelisted and marked as valid.
	fn validate_unsigned(
		_source: TransactionSource,
		_call: &Self::Call,
	) -> TransactionValidity {
		ValidTransaction::with_tag_prefix("EtherscanWorker")
		// We set base priority to 2**20 and hope it's included before any other
		// transactions in the pool. Next we tweak the priority depending on how much
		// it differs from the current average. (the more it differs the more priority it
		// has).
		.priority(T::UnsignedPriority::get())
		// The transaction is only valid for next 5 blocks. After that it's
		// going to be revalidated by the pool.
		.longevity(5)
		// It's fine to propagate that transaction to other peers, which means it can be
		// created even by nodes that don't produce blocks.
		// Note that sometimes it's better to keep it for yourself (if you are the block
		// producer), since for instance in some schemes others may copy your solution and
		// claim a reward.
		.propagate(true)
		.build()
	}
}