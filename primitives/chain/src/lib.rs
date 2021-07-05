#![cfg_attr(not(feature = "std"), no_std)]
#[ignore]
#[test]
fn print_module_account() {
    // --- substrate ---
    use sp_core::crypto::{set_default_ss58_version, Ss58AddressFormat, Ss58AddressFormat::*};
    use sp_runtime::{traits::AccountIdConversion, ModuleId};

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