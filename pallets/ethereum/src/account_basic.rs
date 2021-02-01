use pallet_evm::{Account as EVMAccount, AccountBasicMapping, AddressMapping};
use frame_support::traits::Currency;
use sp_core::{H160, U256};
use sp_runtime::traits::{UniqueSaturatedInto};

pub struct UVMAccountBasicMapping<T>(sp_std::marker::PhantomData<T>);

impl<T: crate::Trait + pallet_balances::Trait> AccountBasicMapping
for UVMAccountBasicMapping<T>
{
    /// Get the account basic in EVM format.
    fn account_basic(address: &H160) -> EVMAccount {
        let account_id = <T as pallet_evm::Trait>::AddressMapping::into_account_id(*address);
        let nonce = frame_system::Module::<T>::account_nonce(&account_id);
        let helper = U256::from(10)
            .checked_pow(U256::from(9))
            .unwrap_or(U256::from(0));

        // Get balance from T::Currency
        let balance: U256 = T::Currency::free_balance(&account_id)
            .unique_saturated_into()
            .into();

        // Final balance = balance * 10^9 + remaining_balance
        let final_balance = U256::from(balance * helper);

        EVMAccount {
            nonce: nonce.unique_saturated_into().into(),
            balance: final_balance,
        }
    }

    /// Mutate the basic account
    fn mutate_account_basic(address: &H160, new: EVMAccount) {
        let account_id = <T as pallet_evm::Trait>::AddressMapping::into_account_id(*address);
        let current = T::AccountBasicMapping::account_basic(address);
        let helper = U256::from(10)
            .checked_pow(U256::from(9))
            .unwrap_or(U256::MAX);

        if current.nonce < new.nonce {
            // ASSUME: in one single EVM transaction, the nonce will not increase more than
            // `u128::max_value()`.
            for _ in 0..(new.nonce - current.nonce).low_u128() {
                frame_system::Module::<T>::inc_account_nonce(&account_id);
            }
        }

        if current.balance > new.balance {
            let diff = current.balance - new.balance;
            let (diff_balance, diff_remaining_balance) = diff.div_mod(helper);
            // If the dvm storage < diff remaining balance, we can not do sub operation directly.
            // Otherwise, slash T::Currency, dec dvm storage balance directly.

            // todo
        } else if current.balance < new.balance {
            let diff = new.balance - current.balance;
            let (diff_balance, diff_remaining_balance) = diff.div_mod(helper);

            // todo
        }
    }
}