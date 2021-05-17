#![feature(test)]

extern crate test;

use test::Bencher;
use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};
use solana_rbpf::{aligned_memory::AlignedMemory, ebpf::HOST_ALIGN};
use solana_sdk::{
    account::{ReadableAccount, WritableAccount},
    bpf_loader_deprecated,
    entrypoint::MAX_PERMITTED_DATA_INCREASE,
    instruction::InstructionError,
    keyed_account::KeyedAccount,
    pubkey::Pubkey,
};
use solana_bpf_loader_program::serialization::serialize_parameters_aligned;
use std::{
    io::prelude::*,
    mem::{align_of, size_of},
};
use solana_sdk::{
    account::{Account, AccountSharedData},
    account_info::AccountInfo,
    bpf_loader,
    entrypoint::deserialize,
};
use std::{
    cell::RefCell,
    rc::Rc,
    // Hide Result from bindgen gets confused about generics in non-generic type declarations
    slice::{from_raw_parts, from_raw_parts_mut},
};

#[bench]
fn bench_is_zeroed(bencher: &mut Bencher) {

    let program_id = solana_sdk::pubkey::new_rand();
    let dup_key = solana_sdk::pubkey::new_rand();
    let dup_key2 = solana_sdk::pubkey::new_rand();
    let keys = vec![
        dup_key,
        dup_key,
        solana_sdk::pubkey::new_rand(),
        solana_sdk::pubkey::new_rand(),
        dup_key2,
        dup_key2,
        solana_sdk::pubkey::new_rand(),
        solana_sdk::pubkey::new_rand(),
    ];
    let accounts = [
        RefCell::new(AccountSharedData::from(Account {
            lamports: 1,
            data: vec![1u8, 2, 3, 4, 5],
            owner: bpf_loader::id(),
            executable: false,
            rent_epoch: 100,
        })),
        // dup
        RefCell::new(AccountSharedData::from(Account {
            lamports: 1,
            data: vec![1u8, 2, 3, 4, 5],
            owner: bpf_loader::id(),
            executable: false,
            rent_epoch: 100,
        })),
        RefCell::new(AccountSharedData::from(Account {
            lamports: 2,
            data: vec![11u8, 12, 13, 14, 15, 16, 17, 18, 19],
            owner: bpf_loader::id(),
            executable: true,
            rent_epoch: 200,
        })),
        RefCell::new(AccountSharedData::from(Account {
            lamports: 3,
            data: vec![],
            owner: bpf_loader::id(),
            executable: false,
            rent_epoch: 3100,
        })),
        RefCell::new(AccountSharedData::from(Account {
            lamports: 4,
            data: vec![1u8, 2, 3, 4, 5],
            owner: bpf_loader::id(),
            executable: false,
            rent_epoch: 100,
        })),
        // dup
        RefCell::new(AccountSharedData::from(Account {
            lamports: 4,
            data: vec![1u8, 2, 3, 4, 5],
            owner: bpf_loader::id(),
            executable: false,
            rent_epoch: 100,
        })),
        RefCell::new(AccountSharedData::from(Account {
            lamports: 5,
            data: vec![11u8, 12, 13, 14, 15, 16, 17, 18, 19],
            owner: bpf_loader::id(),
            executable: true,
            rent_epoch: 200,
        })),
        RefCell::new(AccountSharedData::from(Account {
            lamports: 6,
            data: vec![],
            owner: bpf_loader::id(),
            executable: false,
            rent_epoch: 3100,
        })),
    ];

    let keyed_accounts: Vec<_> = keys
        .iter()
        .zip(&accounts)
        .enumerate()
        .map(|(i, (key, account))| {
            if i <= accounts.len() / 2 {
                KeyedAccount::new_readonly(&key, false, &account)
            } else {
                KeyedAccount::new(&key, false, &account)
            }
        })
        .collect();
    let instruction_data = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

    bencher.iter(|| {
         let _= serialize_parameters_aligned(
        &program_id,
        &keyed_accounts,
        &instruction_data,
    )
    .unwrap();
    });
}
