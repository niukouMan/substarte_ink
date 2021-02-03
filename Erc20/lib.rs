#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod Erc20 {
    use ink_storage::collections::HashMap as StroageHashMap;

    #[cfg(not(feature = "ink-as-dependency"))]
    #[ink(storage)]
    pub struct Erc20 {
        total_supply: Balance,
        balances:StroageHashMap<AccountId,Balance>,
        allowance:StroageHashMap<(AccountId,AccountId),Balance>,
    }

    #[ink(event)]
    pub struct Transfer{
        #[ink(topic)]
        from:Option<AccountId>,
        #[ink(topic)]
        to:Option<AccountId>,
        value:Balance,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        #[ink(topic)]
        value: Balance,
    }

    #[derive(Debug,PartialEq,Eq,scale::Encode)]
    #[cfg_attr(feature="std",derive(scale_info::TypeInfo))]
    pub enum Error{
        InsufficientBalance,
    }

    pub type Result<T> = core::result::Result<T,Error>;

    impl Erc20 {

      #[ink(constructor)]
        pub fn new( initial_supply: Balance) -> Self{
          let mut _balances = ink_storage::collections::HashMap::new();
          _balances.insert(Self::env().caller(), initial_supply);
          let instatnce = Self {
              total_supply: initial_supply,
              balances: _balances,
              allowance:StroageHashMap::new(),
          };
          instatnce
      }

        ///总供应量
      #[ink(message)]
       pub fn get_total_supply(&self) -> Balance{
            self.total_supply
        }

        ///余额
      #[ink(message)]
       pub fn balance_of(&self,owner: AccountId) -> Balance{
            self.balance_or_zero(&owner)
        }


        ///转账
      #[ink(message)]
       pub fn transfer(&mut self,to:AccountId,amount:Balance)-> Result<()>{
            let from = self.env().caller();
            let from_balance = self.balance_or_zero(&from);
            if from_balance<amount{
                return Err(Error::InsufficientBalance);
            }
            let to_balance = self.balance_or_zero(&to);
            self.balances.insert(from,from_balance - amount);
            self.balances.insert(to,to_balance + amount);
            self.env().emit_event(Transfer{
                from:Some(from),
                to:Some(to),
                value:amount,
            });
            Ok(())
        }

       fn balance_or_zero(&self,owner:&AccountId)-> Balance{
            *self.balances.get(owner).unwrap_or(&0)
       }

       fn allowance_or_zero(&self,from:AccountId,to:AccountId)->Balance{
           self.allowance.get(&(from, to)).copied().unwrap_or(0)
       }

        ///授权
        #[ink(message)]
        pub fn approve(&mut self, spender:AccountId, value:Balance) -> Result<()>{
            let from = self.env().caller();
            let from_balance = self.balance_or_zero(&from);
            if from_balance< value {
                return Err(Error::InsufficientBalance);
            }

            let allowance_balance = self.allowance_or_zero(from,spender);
            self.allowance.insert((from, spender), allowance_balance + value);
            self.env().emit_event(Approval {
                owner:from,
                spender,
                value,
            });
            Ok(())
        }


        ///查询授权的数量
       #[ink(message)]
        pub fn allowance(&mut self,owner:AccountId,spender:AccountId) -> Balance{
           self.allowance_or_zero(owner,spender)
       }

        #[ink(message)]
       pub fn transfer_from(&mut self, from:AccountId, to:AccountId, value:Balance) -> Result<()>{
           //判断余额是否足够
            let allowance_balance = self.allowance.get(&(from, to)).copied().unwrap_or(0);
            if allowance_balance < value {
                return Err(Error::InsufficientBalance);
            }

            let from_balance =  self.balance_or_zero(&from);
            if from_balance<value {
                return Err(Error::InsufficientBalance);
            }
            self.balances.insert(from, from_balance - value);
            let to_balance = self.balance_or_zero(&to);
            self.balances.insert(to, to_balance + value);
            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                value,
            });
            Ok(())

       }

        ///燃烧代币
        #[ink(message)]
        pub fn burn(&mut self,from:AccountId,value:Balance)-> Result<()>{
            let from_balance =  self.balance_or_zero(&from);
            if from_balance<value {
                return Err(Error::InsufficientBalance);
            }
            self.balances.insert(from,from_balance-value);
            self.total_supply = self.total_supply-value;
            Ok(())

        }

        ///铸造代币
        #[ink(message)]
       pub fn mint(&mut self ,to:AccountId,value:Balance) -> Result<()>{
            let to_balance =  self.balance_or_zero(&to);
            self.balances.insert(to,to_balance+value);
            self.total_supply = self.total_supply+value;
            Ok(())
       }

    }

    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        #[test]
        fn default_works() {
            let erc20 = Erc20::new(1000);
            assert_eq!(
                erc20.get_total_supply(), 900);
        }

        #[test]
        fn transfer_works() {
            // Constructor works.
            let mut erc20 = Erc20::new(1000);
            // Transfer event triggered during initial construction.
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");

            assert_eq!(erc20.balance_of(accounts.bob), 0);
            // Alice transfers 10 tokens to Bob.
            assert_eq!(erc20.transfer(accounts.bob, 10), Ok(()));
            // Bob owns 10 tokens.
            assert_eq!(erc20.balance_of(accounts.bob), 10);
        }



    }


}
