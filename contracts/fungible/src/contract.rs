// SPDX-License-Identifier: MIT
// Compatible with OpenZeppelin Stellar Soroban Contracts ^0.3.0


use soroban_sdk::{Address, contract, contractimpl, Env, String};
use stellar_default_impl_macro::default_impl;
use stellar_fungible::{Base, burnable::FungibleBurnable, FungibleToken};
use stellar_ownable::{self as ownable, Ownable};
use stellar_ownable_macro::only_owner;
use stellar_pausable::{self as pausable, Pausable};
use stellar_pausable_macros::when_not_paused;

#[contract]
pub struct AuraCoin;

#[contractimpl]
impl AuraCoin {
    pub fn __constructor(e: &Env, owner: Address) {
        Base::set_metadata(e, 18, String::from_str(e, "Aura Coin"), String::from_str(e, "AURA"));
        ownable::set_owner(e, &owner);
    }

    #[only_owner]
    #[when_not_paused]
    pub fn mint(e: &Env, account: Address, amount: i128) {
        Base::mint(e, &account, amount);
    }
}

#[default_impl]
#[contractimpl]
impl FungibleToken for AuraCoin {
    type ContractType = Base;

    #[when_not_paused]
    fn transfer(e: &Env, from: Address, to: Address, amount: i128) {
        Self::ContractType::transfer(e, &from, &to, amount);
    }

    #[when_not_paused]
    fn transfer_from(e: &Env, spender: Address, from: Address, to: Address, amount: i128) {
        Self::ContractType::transfer_from(e, &spender, &from, &to, amount);
    }
}

//
// Extensions
//

#[contractimpl]
impl FungibleBurnable for AuraCoin {
    #[when_not_paused]
    fn burn(e: &Env, from: Address, amount: i128) {
        Base::burn(e, &from, amount);
    }

    #[when_not_paused]
    fn burn_from(e: &Env, spender: Address, from: Address, amount: i128) {
        Base::burn_from(e, &spender, &from, amount);
    }
}

//
// Utils
//

#[contractimpl]
impl Pausable for AuraCoin {
    fn paused(e: &Env) -> bool {
        pausable::paused(e)
    }

    #[only_owner]
    fn pause(e: &Env, caller: Address) {
        pausable::pause(e);
    }

    #[only_owner]
    fn unpause(e: &Env, caller: Address) {
        pausable::unpause(e);
    }
}

#[default_impl]
#[contractimpl]
impl Ownable for AuraCoin {}
