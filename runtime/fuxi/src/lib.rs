#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit="256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

/// Wasm binary unwrapped. If built with `SKIP_WASM_BUILD`, the function panics.
#[cfg(feature = "std")]
pub fn wasm_binary_unwrap() -> &'static [u8] {
	WASM_BINARY.expect("Development wasm binary is not available. This means the client is \
						built with `SKIP_WASM_BUILD` flag and it is only usable for \
						production chains. Please rebuild with the flag disabled.")
}

/// Constant values used within the runtime.
pub mod constants;
pub use constants::time::*;
pub use constants::currency::*;
pub mod configs;
pub use configs::*;

/// Weights for configs used in the runtime.
mod weights;

// --- crates ---
use sp_std::prelude::*;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
use sp_runtime::{
	ApplyExtrinsicResult, generic, create_runtime_str, impl_opaque_keys,
	transaction_validity::{TransactionValidity, TransactionSource}
};
use sp_runtime::traits::{
	BlakeTwo256, Block as BlockT, NumberFor, Saturating, AccountIdConversion,
	Convert, OpaqueKeys, SaturatedConversion, Bounded
};
use frame_system::{EnsureOneOf, EnsureRoot};
use sp_api::impl_runtime_apis;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{u32_trait::{_1, _2, _3, _5}, };
use pallet_grandpa::{AuthorityId as GrandpaId, AuthorityList as GrandpaAuthorityList};
use pallet_grandpa::fg_primitives;
use sp_version::RuntimeVersion;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use orml_currencies::BasicCurrencyAdapter;
use pallet_transaction_payment::{FeeDetails, RuntimeDispatchInfo};
pub use pallet_transaction_payment::{Multiplier, TargetedFeeAdjustment, CurrencyAdapter};
use sp_inherents::{InherentData, CheckInherentsResult};

// Uni-Arts
type Uart = Balances;

// A few exports that help ease life for downstream crates.
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
#[cfg(any(feature = "std", test))]
pub use pallet_balances::Call as BalancesCall;
#[cfg(any(feature = "std", test))]
pub use frame_system::Call as SystemCall;
#[cfg(any(feature = "std", test))]
pub use pallet_staking::StakerStatus;

pub use sp_runtime::{Permill, Perbill, Percent, ModuleId};

pub use pallet_timestamp::Call as TimestampCall;
pub use pallet_balances::Call as BalancesCall;
pub use frame_support::{
	construct_runtime, parameter_types, StorageValue, ConsensusEngineId,
	traits::{OnUnbalanced, ChangeMembers, KeyOwnerProofSystem, Randomness, StorageMapShim, Currency, Imbalance,
			 Contains, ContainsLengthBound, InstanceFilter, LockIdentifier, SplitTwoWays, FindAuthor
	},
	weights::{
		Weight, IdentityFee,
		constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
	},
};

pub use uniarts_primitives::{
	BlockNumber, Signature, AccountId, AccountIndex, Balance, Index, Hash, DigestItem,
	TokenSymbol, CurrencyId, Amount, Header,
};

/// Import configs.
// pub use pallet_certificate;
pub use pallet_nft_multi;
pub use pallet_nicks;
pub use pallet_staking;
pub use pallet_validator_set;
pub use uniarts_common::*;

/// We assume that ~10% of the block weight is consumed by `on_initalize` handlers.
/// This is used to limit the maximal weight of a single extrinsic.
const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(10);
/// We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used
/// by  Operational  extrinsics.
const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);
/// We allow for 2 seconds of compute with a 6 second average block time.
const MAXIMUM_BLOCK_WEIGHT: Weight = 2 * WEIGHT_PER_SECOND;

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
	use super::*;

	pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

	/// Opaque block header type.
	pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// Opaque block type.
	pub type Block = generic::Block<Header, UncheckedExtrinsic>;
	/// Opaque block identifier type.
	pub type BlockId = generic::BlockId<Block>;

	impl_opaque_keys! {
		pub struct SessionKeys {
			pub aura: Aura,
			pub grandpa: Grandpa,
		}
	}
}

pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("uart"),
	impl_name: create_runtime_str!("uart"),
	authoring_version: 1,
	spec_version: 43,
	impl_version: 1,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 2,
};

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion {
		runtime_version: VERSION,
		can_author_with: Default::default(),
	}
}

// Module accounts of runtime
parameter_types! {
	pub const UniArtsTreasuryModuleId: ModuleId = ModuleId(*b"py/trsry");
	pub const StakingModuleId: ModuleId = ModuleId(*b"staking_");
	pub const UniArtsNftModuleId: ModuleId = ModuleId(*b"art/nftb");
	pub const UniArtsBlindBoxModuleId: ModuleId = ModuleId(*b"art/bbxb");
	pub const LotteryModuleId: ModuleId = ModuleId(*b"art/lotb");
	pub const SocietyModuleId: ModuleId = ModuleId(*b"art/soci");
	pub const ElectionsPhragmenModuleId: LockIdentifier = *b"art/phre";
	pub ZeroAccountId: AccountId = AccountId::from([0u8; 32]);
}

pub fn get_all_module_accounts() -> Vec<AccountId> {
	vec![
		UniArtsTreasuryModuleId::get().into_account(),
		ZeroAccountId::get(),
	]
}

parameter_types! {
	pub const BlockHashCount: BlockNumber = 2400;
	/// We allow for 2 seconds of compute with a 6 second average block time.
	pub const MaximumBlockWeight: Weight = 2 * WEIGHT_PER_SECOND;
	pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
	/// Assume 10% of weight for average on_initialize calls.
	pub MaximumExtrinsicWeight: Weight = AvailableBlockRatio::get()
		.saturating_sub(Perbill::from_percent(10)) * MaximumBlockWeight::get();
	pub const MaximumBlockLength: u32 = 5 * 1024 * 1024;
	pub const Version: RuntimeVersion = VERSION;
}

// Configure FRAME configs to include in runtime.
// In config mod

// Create the runtime by composing the FRAME configs that were previously configured.
construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system::{Module, Call, Config, Storage, Event<T>},
		RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Module, Call, Storage},
		Timestamp: pallet_timestamp::{Module, Call, Storage, Inherent},
		Indices: pallet_indices::{Module, Call, Storage, Config<T>, Event<T>},

		Authorship: pallet_authorship::{Module, Call, Storage},
		Session: pallet_session::{Module, Call, Storage, Event, Config<T>},
		ValidatorSet: pallet_validator_set::{Module, Call, Storage, Event<T>, Config<T>},
		Aura: pallet_aura::{Module, Config<T>},
		Grandpa: pallet_grandpa::{Module, Call, Storage, Config, Event},
		Rewards: pallet_rewards::{Module, Call, Storage, Event<T>},
		Staking: pallet_staking::{Module, Call, Storage, Event<T>},
		Vesting: pallet_vesting::{Module, Call, Storage, Event<T>, Config<T>},

		// Nicks: pallet_nicks::{Module, Call, Storage, Event<T>},
		Balances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>},
		Contracts: pallet_contracts::{Module, Call, Storage, Event<T>, Config},
		// Lotteries: pallet_lotteries::{Module, Call, Storage, Event<T>},
		// Uart: pallet_balances::<Instance0>::{Module, Call, Storage, Config<T>, Event<T>},

		// Orml
		Currencies: orml_currencies::{Module, Call, Event<T>},
		UniTokens: orml_tokens::{Module, Storage, Event<T>, Config<T>},

		// Governance
		Council: pallet_collective::<Instance0>::{Module, Call, Storage, Origin<T>, Event<T>, Config<T>},
		CouncilMembership: pallet_membership::<Instance0>::{Module, Call, Storage, Event<T>, Config<T>},
		Treasury: pallet_treasury::{Module, Call, Storage, Config, Event<T>},
		Bounties: pallet_bounties::{Module, Call, Storage, Event<T>},
		TechnicalCommittee: pallet_collective::<Instance1>::{Module, Call, Storage, Origin<T>, Config<T>, Event<T>},
		TechnicalMembership: pallet_membership::<Instance1>::{Module, Call, Storage, Config<T>, Event<T>},
		Identity: pallet_identity::{Module, Call, Storage, Event<T>},
		ElectionsPhragmen: pallet_elections_phragmen::{Module, Call, Storage, Event<T>},
		// Society module.
		Society: pallet_society::{Module, Call, Storage, Event<T>},

		// System scheduler.
		Scheduler: pallet_scheduler::{Module, Call, Storage, Event<T>},
		TransactionPayment: pallet_transaction_payment::{Module, Storage},
		Sudo: pallet_sudo::{Module, Call, Config<T>, Storage, Event<T>},

		Names: pallet_names::{Module, Call, Storage, Event<T>},
		Assets: pallet_assets::{Module, Call, Storage, Event<T>},
		Nft: pallet_nft_multi::{Module, Call, Storage, Event<T>},
		BlindBox: pallet_nft_blindbox::{Module, Call, Storage, Event<T>},
		Bridge: pallet_bridge::{Module, Call, Storage, Event<T>, Config<T>},
		Utility: pallet_utility::{Module, Call, Event},
		Proxy: pallet_proxy::{Module, Call, Storage, Event<T>},
		Multisig: pallet_multisig::{Module, Call, Storage, Event<T>},
		Recovery: pallet_recovery::{Module, Call, Storage, Event<T>},
	}
);

/// The address format for describing accounts.
pub type Address = AccountId;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	frame_system::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
	pallet_transaction_payment::ChargeTransactionPayment<Runtime>
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Call, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
	Runtime,
	Block,
	frame_system::ChainContext<Runtime>,
	Runtime,
	AllModules,
>;

impl_runtime_apis! {
	impl sp_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			Executive::execute_block(block)
		}

		fn initialize_block(header: &<Block as BlockT>::Header) {
			Executive::initialize_block(header)
		}
	}

	impl sp_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			Runtime::metadata().into()
		}
	}

	impl sp_block_builder::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> <Block as BlockT>::Header {
			Executive::finalize_block()
		}

		fn inherent_extrinsics(data: InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			data.create_extrinsics()
		}

		fn check_inherents(block: Block, data: InherentData) -> CheckInherentsResult {
			data.check_extrinsics(&block)
		}

		fn random_seed() -> <Block as BlockT>::Hash {
			RandomnessCollectiveFlip::random_seed()
		}
	}

	impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic,
		) -> TransactionValidity {
			Executive::validate_transaction(source, tx)
		}
	}

	impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &<Block as BlockT>::Header) {
			Executive::offchain_worker(header)
		}
	}

	impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
		fn slot_duration() -> u64 {
			Aura::slot_duration()
		}

		fn authorities() -> Vec<AuraId> {
			Aura::authorities()
		}
	}

	impl sp_session::SessionKeys<Block> for Runtime {
		fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
			opaque::SessionKeys::generate(seed)
		}

		fn decode_session_keys(
			encoded: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
			opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
		}
	}

	impl fg_primitives::GrandpaApi<Block> for Runtime {
		fn grandpa_authorities() -> GrandpaAuthorityList {
			Grandpa::grandpa_authorities()
		}

		fn submit_report_equivocation_unsigned_extrinsic(
			_equivocation_proof: fg_primitives::EquivocationProof<
				<Block as BlockT>::Hash,
				NumberFor<Block>,
			>,
			_key_owner_proof: fg_primitives::OpaqueKeyOwnershipProof,
		) -> Option<()> {
			None
		}

		fn generate_key_ownership_proof(
			_set_id: fg_primitives::SetId,
			_authority_id: GrandpaId,
		) -> Option<fg_primitives::OpaqueKeyOwnershipProof> {
			// NOTE: this is the only implementation possible since we've
			// defined our key owner proof type as a bottom type (i.e. a type
			// with no values).
			None
		}
	}
	
	impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
		fn account_nonce(account: AccountId) -> Index {
			System::account_nonce(account)
		}
	}

	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<
		Block,
		Balance,
	> for Runtime {
		fn query_info(uxt: <Block as BlockT>::Extrinsic, len: u32) -> RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_info(uxt, len)
		}
		fn query_fee_details(uxt: <Block as BlockT>::Extrinsic, len: u32) -> FeeDetails<Balance> {
			TransactionPayment::query_fee_details(uxt, len)
		}
	}

	impl pallet_contracts_rpc_runtime_api::ContractsApi<Block, AccountId, Balance, BlockNumber>
		for Runtime
	{
		fn call(
			origin: AccountId,
			dest: AccountId,
			value: Balance,
			gas_limit: u64,
			input_data: Vec<u8>,
		) -> pallet_contracts_primitives::ContractExecResult {
			Contracts::bare_call(origin, dest, value, gas_limit, input_data)
		}

		fn get_storage(
			address: AccountId,
			key: [u8; 32],
		) -> pallet_contracts_primitives::GetStorageResult {
			Contracts::get_storage(address, key)
		}

		fn rent_projection(
			address: AccountId,
		) -> pallet_contracts_primitives::RentProjectionResult<BlockNumber> {
			Contracts::rent_projection(address)
		}
	}

	impl pallet_staking_rpc_runtime_api::StakingApi<Block, AccountId, Balance> for Runtime {
		fn staking_module_account_id() -> AccountId {
			Staking::account_id()
		}

		fn pool_account_id(id: u32) -> AccountId {
			Staking::pool_account_id(id)
		}

		fn pending_rewards(pool_id: u32, account_id: AccountId) -> Balance {
			Staking::pending_rewards(pool_id, account_id)
		}
	}

	#[cfg(feature = "runtime-benchmarks")]
	impl frame_benchmarking::Benchmark<Block> for Runtime {
		fn dispatch_benchmark(
			config: frame_benchmarking::BenchmarkConfig
		) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
			use frame_benchmarking::{Benchmarking, BenchmarkBatch, add_benchmark, TrackedStorageKey};

			use frame_system_benchmarking::Module as SystemBench;
			impl frame_system_benchmarking::Config for Runtime {}

			let whitelist: Vec<TrackedStorageKey> = vec![
				// Block Number
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac").to_vec().into(),
				// Total Issuance
				hex_literal::hex!("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80").to_vec().into(),
				// Execution Phase
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a").to_vec().into(),
				// Event Count
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850").to_vec().into(),
				// System Events
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7").to_vec().into(),
			];

			let mut batches = Vec::<BenchmarkBatch>::new();
			let params = (&config, &whitelist);

			add_benchmark!(params, batches, frame_system, SystemBench::<Runtime>);
			add_benchmark!(params, batches, pallet_balances, Balances);
			add_benchmark!(params, batches, pallet_timestamp, Timestamp);
			add_benchmark!(params, batches, pallet_rewards, Rewards);

			if batches.is_empty() { return Err("Benchmark not found for this pallet.".into()) }
			Ok(batches)
		}
	}
}
