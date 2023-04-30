use futures::future::join_all;
use fx_processed_profile::{
    table_address::TableAddress, IndexIntoCategoryList, IndexIntoStackTable, Milliseconds,
    SamplesTable,
};
use std::path::Path;

use profile_table_iterator::TableLookup;
use serde_json::value::Index;
use wholesym::{SymbolManager, SymbolManagerConfig};

use crate::fx_processed_profile::{table_address::Address, IndexIntoFrameTable, StackTable};

pub mod fx_processed_profile;
pub mod profile_table_iterator;

pub async fn gather_samples(profile: fx_processed_profile::Profile) -> () {
    println!("Gathering samples.");
    let libs = &profile.libs;

    let sm = SymbolManager::with_config(SymbolManagerConfig::new());
    // Start off by getting the symbols with samply.
    let clibs = &profile.libs.clone();
    let symbol_managers = join_all(clibs.iter().map(|lib| {
        println!("Loading: {:?} @ {:?}", lib.name, lib.path);
        sm.load_symbol_map_for_binary_at_path(Path::new(&lib.path), None)
    }))
    .await;

    // symbol_managers.iter().for_each(|m| {
    //     m.for
    // });

    // let mut acc = vec![];
    // Start going through the profile, threads first:
    profile.threads.iter().for_each(|thread| {
        let stack_table = &thread.stackTable;
        let frame_table = &thread.frameTable;
        let string_table = &thread.stringTable;
        let symbol_table = &thread.nativeSymbols;
        // Within a thread, walk samples
        let sample_table = &thread.samples;

        for s in sample_table.iter() {
            match s.stack {
                Some(i) => {

                    let stack_table_entry: IndexIntoFrameTable = stack_table.frame[i as usize];

                    let frame_table_entry = frame_table.lookup(stack_table_entry as usize);
                    let string_table_index = frame_table_entry.implementation;


                    frame_table_entry
                        .nativeSymbol
                        .map(|ix| symbol_table.lookup(ix as usize))
                        .map(|nste| {
                            // Try to look up the name:
                            match &symbol_managers[nste.libIndex as usize] {
                                Ok(sm) => {
                                    println!("\tAddress: {:?}", nste.address);
                                    println!("\tLibrary: {:?}", libs[nste.libIndex as usize]);
                                    println!("\tStack frame: {:?}", i);
                                    println!("Sample @ {:?}", s.time);
                                    match string_table_index {
                                        Some(i) => {
                                            let s = string_table[i as usize].clone();
                                            println!("\tName: {:?}", s);
                                        }
                                        _ => {}
                                    }

                                    let result = sm.lookup_relative_address(nste.address as u32);
                                    println!("\tLookup result: {:?}", result);
                                }
                                Err(_) => {},
                            }
                            // string_table[nste.name as usize].clone()
                        });
                        // .map(|s| {
                        //     println!("\ts: {:?}", s);
                        //     s
                        // });
                }
                _ => {}
            }
        }
        // let time_stack_pairs = table.
    });
    // acc
}

fn symbolicate() {}
