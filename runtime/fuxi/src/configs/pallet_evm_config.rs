use pallet_evm::Config;
use crate::*;

use sp_core::U256;
use pallet_evm::{
    HashedAddressMapping, EnsureAddressTruncated,
};

frame_support::parameter_types! {
	pub const ChainId: u64 = 42;
	pub BlockGasLimit: U256 = U256::from(u32::max_value());
}

impl Config for Runtime {
    type FeeCalculator = pallet_dynamic_fee::Module<Self>;
    type GasWeightMapping = ();
    type CallOrigin = EnsureAddressTruncated;
    type WithdrawOrigin = EnsureAddressTruncated;
    type AddressMapping = HashedAddressMapping<BlakeTwo256>;
    type Currency = Balances;
    type Event = Event;
    type Runner = pallet_evm::runner::stack::Runner<Self>;
    type Precompiles = (
        pallet_evm_precompile_simple::ECRecover,
        pallet_evm_precompile_simple::Sha256,
        pallet_evm_precompile_simple::Ripemd160,
        pallet_evm_precompile_simple::Identity,
        pallet_evm_precompile_modexp::Modexp,
        pallet_evm_precompile_simple::ECRecoverPublicKey,
        pallet_evm_precompile_sha3fips::Sha3FIPS256,
        pallet_evm_precompile_sha3fips::Sha3FIPS512,
    );
    type ChainId = ChainId;
    type BlockGasLimit = BlockGasLimit;
    type OnChargeTransaction = ();
}
