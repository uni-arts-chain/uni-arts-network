#![cfg_attr(not(feature = "std"), no_std)]
#[ignore]
#[test]
fn print_module_account() {
    // --- substrate ---
    use sp_core::crypto::{set_default_ss58_version, Ss58AddressFormat, Ss58AddressFormat::*};
    use sp_runtime::{traits::AccountIdConversion, ModuleId};
    use uniarts_primitives::AccountId;

    fn account_of(alias: [u8; 8], ss58_version: Ss58AddressFormat) {
        set_default_ss58_version(ss58_version);

        let alias_str = unsafe { core::str::from_utf8_unchecked(&alias) };
        let id = <ModuleId as AccountIdConversion<AccountId>>::into_account(&ModuleId(alias));

        eprintln!("{}:\n\t{}\n\t{:?}", alias_str, id, id);
    }

    // py/trsry:
    // 5EYCAe5ijiYfyeZ2JJCGq56LmPyNRAKzpG4QkoQkkQNB5e6Z
    // 6d6f646c70792f74727372790000000000000000000000000000000000000000 (5EYCAe5i...)
    account_of(*b"py/trsry", SubstrateAccount);

    // art/nftb:
    // 5EYCAe5fj5zwigs2Sr1KavTHcx1xfnpjUkN4SnAW9ngo8k4g
    // 6d6f646c6172742f6e6674620000000000000000000000000000000000000000 (5EYCAe5f...)
    account_of(*b"art/nftb", SubstrateAccount);

    // art/soci:
    // 5EYCAe5fj5zwiqofZc6Q2cXmZSJQm9kW7Q5e1np77Fyog5DQ
    // 6d6f646c6172742f736f63690000000000000000000000000000000000000000 (5EYCAe5f...)
    account_of(*b"art/soci", SubstrateAccount);

    // art/phre:
    // 5EYCAe5fj5zwikRqzNMCGpqMKcMNku4UHZTCpcGv2VmqWFAC
    // 6d6f646c6172742f706872650000000000000000000000000000000000000000 (5EYCAe5f...)
    account_of(*b"art/phre", SubstrateAccount);
}

#[ignore]
#[test]
fn print_evm_account() {
    // --- substrate ---
    use sp_core::H160;
    use sp_runtime::AccountId32;
    use std::str::FromStr;
    pub use pallet_evm::AddressMapping;

    pub struct HashedAddressMapping;

    impl AddressMapping<AccountId32> for HashedAddressMapping {
        fn into_account_id(address: H160) -> AccountId32 {
            let mut data = [0u8; 32];
            data[0..20].copy_from_slice(&address[..]);
            let accountid = AccountId32::from(Into::<[u8; 32]>::into(data));
            eprintln!("address: {:?}\naccountId: {}", address, accountid);
            accountid
        }
    }

    // address: 0x6c097fb92092793608fb3860509100be23c4f20f
    // accountId: 5EWMrHFsEnSgHVRVD72uofG8E7bfnmE1hxyVpdyLM4qVLBcx
    HashedAddressMapping::into_account_id(H160::from_str("6C097fB92092793608fB3860509100BE23c4f20F").unwrap());

    // address: 0x7184035beead581f3dcedecba5db6547a914fbb9
    // accountId: 5EdYV3AszLFPYBQdGxWhJoawFfMBXGyCUgzPZznF5LZUFKS8
    HashedAddressMapping::into_account_id(H160::from_str("7184035beEAD581f3dcEDeCBa5dB6547A914fBB9").unwrap());
}