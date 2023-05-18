use futures::future::join;
use futures::future::join_all;
use fx_processed_profile::NativeSymbolTable;
use fx_processed_profile::NativeSymbolTableEntry;
use fx_processed_profile::{
    table_address::TableAddress, IndexIntoCategoryList, IndexIntoStackTable, Milliseconds,
    SamplesTable, FrameTable,
};
use std::path::Path;

use profile_table_iterator::TableLookup;
use serde_json::value::Index;
use wholesym::{LibraryInfo, SymbolManager, SymbolManagerConfig};

use crate::fx_processed_profile::Lib;
use crate::fx_processed_profile::{table_address::Address, IndexIntoFrameTable, StackTable};

pub mod fx_processed_profile;
pub mod profile_table_iterator;
pub mod transposed;

const MOZILLA_SYMBOL_SERVER: &'static str = "https://symbols.mozilla.org/";


pub async fn query_symbol_file_server(filename: &String, uuid: &String) -> Option<()> {
    // We can call unwrap here "safely" as the URL is static (i.e. we've pre-verified it).
    let base_url : url::Url = url::Url::parse(MOZILLA_SYMBOL_SERVER).unwrap();

    // We want to make a request like:
    // https://symbols.mozilla.org/XUL/5B2AE053F0313841AF23AA605E66A6470/XUL.dSYM.tar.bz2
    let sym_file = filename.clone() + ".dSYM.tar.bz2"; // MacOS Specific!
    // Construct a url for the request
    let mut url = base_url.clone();
    url = url.join(filename.as_str()).ok()?;
    url = url.join(uuid.as_str()).ok()?;
    url = url.join(sym_file.as_str()).ok()?;

    let response = reqwest::get(url.clone()).await.ok()?;
    let status = response.status();

    if !(status.is_redirection() || status.is_success()) {
        return None;
    }

    // We have some success, so continue to create a tempfile and download the data.
    println!("Successfully found symbols at {:?}", url.as_str());


    None
}

// Note, this is currently MacOS specific.
const MOZILLA_LIBS: [&'static str; 2] = ["XUL", "firefox"];

pub async fn find_symbol_map(
    lib: &Lib,
    symbol_manager: &SymbolManager,
) -> Option<wholesym::SymbolMap> {
    // Carry out a multi-staged attaempt to get a symbolmap for this library.
    // Step 1. We only care about libraries that we can *optimise*, i.e.
    // libraries that contain Mozilla source code, so reject all libraries
    // where the name is not in the list of libraries that we care about.
    let lib_name = lib.name.clone();
    if ! MOZILLA_LIBS.contains(&lib_name.as_str() ) {
        return None;
    }
    // Step 2. See if we can get the info from a simple lookup of the file.
    let symbol_map = symbol_manager
        .load_symbol_map_for_binary_at_path(
            Path::new(&lib.path),
            Some(wholesym::MultiArchDisambiguator::Arch("arm64".into())),
        )
        .await;

    // If it works out, return the map we found
    if let Ok(map) = symbol_map {
        return Some(map);
    }

    // Step 3. If step two fails, then go to the internet to look for the symbols.
    // Build a query.
    let symbol_file = query_symbol_file_server(&lib.name, &lib.breakpadId);
    None
}

pub async fn get_lib_info(
    libs: Vec<Lib>,
) -> Vec<(
    Result<wholesym::SymbolMap, wholesym::Error>,
    Result<LibraryInfo, wholesym::Error>,
)> {
    let sm = SymbolManager::with_config(SymbolManagerConfig::new());

    join_all(libs.iter().map(|lib| {
        // Try and get a symbol map for the binary.
        let symbol_map = sm.load_symbol_map_for_binary_at_path(Path::new(&lib.path), None);
        // TODO: We should try and disambiguate!
        let library_info =
            SymbolManager::library_info_for_binary_at_path(Path::new(&lib.path), None);
        join(symbol_map, library_info)
    }))
    .await
}


pub async fn gather_samples(profile: fx_processed_profile::Profile) -> () {
    println!("Gathering samples.");
    let libs = &profile.libs;

    let sm = SymbolManager::with_config(SymbolManagerConfig::new());
    // Start off by getting the symbols with samply.
    let clibs = &profile.libs.clone();
    for lib in clibs {
        if let Some(sym_map) = find_symbol_map(lib,&sm ).await {
            println!("Found symbol map for: {:?}", sym_map.symbol_file_origin());
            println!("\tSymbol count: {:?}", sym_map.symbol_count());
            if sym_map.symbol_count() < 100 {
                for (id, name) in sym_map.iter_symbols() {
                    println!("\t\tSymbol: {:?} -- {:?}", id, name);
                }
            }
        }
    }
    // let symbol_managers = join_all(clibs.iter().map(|lib| {
    //     println!(
    //         "Loading: {:?} / {:?} @ {:?}",
    //         lib.name, lib.breakpadId, lib.path
    //     );
    //     sm.load_symbol_map_for_binary_at_path(
    //         Path::new(&lib.path),
    //         Some(wholesym::MultiArchDisambiguator::Arch("arm64".into())),
    //     )
    // }))
    // .await;

    // let dylib_info = join_all(clibs.iter().map(|lib| {
    //     println!("Computing debug info for: {:?}", lib.name);
    //     SymbolManager::library_info_for_binary_at_path(
    //         Path::new(&lib.path),
    //         Some(wholesym::MultiArchDisambiguator::Arch("arm64".into())),
    //     )
    // }))
    // .await;

    // dylib_info.iter().for_each(|m| match m {
    //     Ok(r) => {
    //         println!(
    //             "Library info: {:?}, {:?}, {:?}",
    //             r.name, r.debug_id, r.code_id
    //         );
    //         match r.debug_id {
    //             Some(d) => println!(
    //                 "Breakpad: {:?}, simple: {}",
    //                 d.breakpad().to_string(),
    //                 d.uuid().as_simple()
    //             ),
    //             _ => {}
    //         };
    //     }
    //     _ => {
    //         println!("Failed to load libraryinfo for library");
    //     }
    // });
}

fn symbolicate() {}
