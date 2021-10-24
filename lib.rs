#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod erc20 {
    #[cfg(not(feature = "ink-as-dependency"))]
    #[ink(storage)]
    pub struct Erc20 {
        /// The total supply.
        total_supply: Balance,
        /// The balance of each user.
        balances: ink_storage::collections::HashMap<AccountId, Balance>,
        ///Approval spender on behalf of the message's sender.
        allowances: ink_storage::collections::HashMap<(AccountId, AccountId), Balance>,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval{
        #[ink(topic)]
        owner: Option<AccountId>,
        #[ink(topic)]
        spender: Option<AccountId>,
        #[ink(topic)]
        value: Balance,
    }



    impl Erc20 {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {

            // ACTION: `insert` the `initial_supply` as the `caller` balance
            let mut balances = ink_storage::collections::HashMap::new();
            balances.insert(Self::env().caller(),initial_supply);

            Self::env().emit_event(Transfer{
                from: None,
                to: Some(Self::env().caller()),
                value: initial_supply,
                });



            // ACTION: `set` the total supply to `initial_supply`
            Self{
                total_supply:initial_supply,
                balances,
                allowances: ink_storage::collections::HashMap::new(),
            }
            

        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            // ACTION: Return the total supply
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            // ACTION: Return the balance of `owner`
            //   HINT: Use `balance_of_or_zero` to get the `owner` balance
            self.balance_of_or_zero(&owner)
        }

        #[ink(message)]

        pub fn approve(&mut self, spender: AccountId, value: Balance) -> bool {
            let owner = self.env().caller();
            self.allowances.insert((owner, spender), value);

            self.env().emit_event(Approval{
                owner: Some(owner),
                spender: Some(spender),
                value: value,
            });
            true

        }

        #[ink(message)]

        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowance_of_or_zero(&owner,&spender)
        }

        #[ink(message)]

        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> bool{
            let caller = self.env().caller();
            let allow= self.allowance_of_or_zero(&from,&caller);
            if allow<value{
                return false
            } 
            let transfer = self.transfer_from_to(from,to,value);
            if !transfer {
                return false
            }
            self.allowances.insert((from,caller),allow-value);
            true
        }

        fn balance_of_or_zero(&self, owner: &AccountId) -> Balance {
            // ACTION: `get` the balance of `owner`, then `unwrap_or` fallback to 0
            // ACTION: Return the balance
            let bal = *self.balances.get(owner).unwrap_or(&0);
            bal
        }

        fn allowance_of_or_zero(&self, owner: &AccountId, spender: &AccountId) -> Balance {
            let all = *self.allowances.get(&(*owner,*spender)).unwrap_or(&0);
            all
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> bool{
            let from = self.env().caller();
            self.transfer_from_to(from,to,value)
        }

        fn transfer_from_to(&mut self, from: AccountId, to: AccountId, value: Balance) -> bool {
            let balfrom = self.balance_of_or_zero(&from);
            let balto= self.balance_of_or_zero(&to);
            if balfrom < value {
                return false
            } else {
                self.balances.insert(from,balfrom-value);
                self.balances.insert(to,balto+value);
                true

            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use ink_lang as ink;

        #[ink::test]
        fn new_works() {
            let contract = Erc20::new(777);
            assert_eq!(contract.total_supply(), 777);
        }

        #[ink::test]
        fn balance_works() {
            let contract = Erc20::new(100);
            assert_eq!(contract.total_supply(), 100);
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 0);
        }
    }
}