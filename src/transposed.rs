use crate::{
    fx_processed_profile::{
        self, FrameTable, IndexIntoFrameTable, NativeSymbolTable, NativeSymbolTableEntry,
        SamplesTable, StackTable,
    },
    profile_table_iterator::TableLookup,
};

/// A ThreadTables struct is a collection of references to thread-specific tables.
/// This is important for when we collect samples from a set of threads, as indices into
/// tables are thread-specific, and we would otherwise not know which table to look into.
#[derive(Copy, Clone)]
pub struct ThreadTables<'a> {
    pub stack_table: &'a StackTable,
    pub frame_table: &'a FrameTable,
    pub string_table: &'a Vec<String>,
    pub symbol_table: &'a NativeSymbolTable,
}

/// A Transposed sample is a flattened form of the processed firefox profile samples.
/// We want it in this form so that we can iterate in a "flat" manner.
pub struct TransposedSample<'a> {
    pub stack_frame: i64,
    pub symbol_table_entry: NativeSymbolTableEntry,
    pub sample_time: f64,
    pub string_table_index: Option<i64>,
    // lookup references, as these are thread specific, so we need to
    pub thread_tables: ThreadTables<'a>,
}

pub fn transpose_samples<'a>(
    profile: &'a fx_processed_profile::Profile,
) -> Vec<TransposedSample<'a>> {
    let mut acc = vec![];
    // Start going through the profile, threads first:
    profile.threads.iter().for_each(|thread| {
        let stack_table: &StackTable = &thread.stackTable;
        let frame_table: &FrameTable = &thread.frameTable;
        let symbol_table: &NativeSymbolTable = &thread.nativeSymbols;

        let thread_tables = ThreadTables {
            stack_table: &thread.stackTable,
            frame_table: &thread.frameTable,
            string_table: &thread.stringTable,
            symbol_table: &thread.nativeSymbols,
        };

        // Within a thread, walk samples
        let sample_table: &SamplesTable = &thread.samples;

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
                            let transposed_sample = TransposedSample {
                                stack_frame: i,
                                symbol_table_entry: nste,
                                sample_time: s.time,
                                string_table_index: string_table_index,
                                thread_tables: thread_tables,
                            };
                            acc.push(transposed_sample);
                            // Try to look up the name:
                            // match &symbol_managers[nste.libIndex as usize] {
                            //     Ok(sm) => {
                            //         println!("Sample @ {:?}", s.time);
                            //         let result = sm.lookup_relative_address(nste.address as u32);
                            //         println!("\tLookup result: {:?}", result);
                            //     }
                            //     Err(_) => {}
                            // }
                        });
                }
                _ => {}
            }
        }
        // let time_stack_pairs = table.
    });
    acc
}
