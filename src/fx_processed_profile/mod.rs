pub mod table_address;

use serde::de;
use serde::de::{Deserializer, Visitor};
use serde::Serializer;
use serde::{ser, Deserialize, Serialize};
use std::fmt;

use crate::profile_table_iterator::{self, TableIterator, TableLookup};
use table_address::{Address, TableAddress};
// Type definitions for a "processed" firefox profile.
// This definition closely follow the Flow types found in [profile.js](https://github.com/firefox-devtools/profiler/blob/main/src/types/profile.js), trying to use the same names where possible.
// For this reason, this file is rather under-documented to avoid replicating or diverging documentation.

// Some utility types to reduce typing:
pub type Array<T> = Vec<T>;
pub type ArrayQ<T> = Array<Option<T>>;

pub type IndexIntoStackTable = i64;
pub type IndexIntoSamplesTable = i64;
pub type IndexIntoRawMarkerTable = i64;
pub type IndexIntoFrameTable = i64;
pub type IndexIntoStringTable = i64;
pub type IndexIntoFuncTable = i64;
pub type IndexIntoResourceTable = TableAddress;
pub type IndexIntoLibs = i64;
pub type IndexIntoNativeSymbolTable = i64;
pub type IndexIntoCategoryList = i64;
pub type IndexIntoSubcategoryListForCategory = i64;
pub type resourceTypeEnum = i64;
pub type ThreadIndex = i64;

// The Tid is most often a i64. However in some cases such as merged profiles
// we could generate a string.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Tid {
    String(String),
    Integer(u32),
}
pub type IndexIntoJsTracerEvents = i64;
pub type CounterIndex = i64;
pub type TabID = i64;
pub type InnerWindowID = i64;
pub type Pid = String;

// Some generic types from units.js, translated into Rust
pub type Nanoseconds = f64;
pub type Microseconds = f64;
pub type Milliseconds = f64;
pub type Seconds = f64;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StartEndRange {
    pub start: Milliseconds,
    pub end: Milliseconds,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StackTable {
    pub frame: Array<IndexIntoFrameTable>,
    pub category: Array<IndexIntoCategoryList>,
    pub prefix: ArrayQ<IndexIntoStackTable>,
    pub length: u64,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct StackTableEntry {
    pub frame: IndexIntoFrameTable,
    pub category: IndexIntoCategoryList,
    pub prefix: Option<IndexIntoStackTable>,
}

impl TableLookup<StackTableEntry> for StackTable {
    fn length(&self) -> usize {
        self.length as usize
    }
    fn lookup(&self, ix: usize) -> StackTableEntry {
        // TODO: At some point, we should validate the lengths...
        StackTableEntry {
            frame: self.frame[ix],
            category: self.category[ix],
            prefix: self.prefix[ix],
        }
    }
    fn iter(&self) -> profile_table_iterator::TableIterator<Self, StackTableEntry>
    where
        Self: Sized,
    {
        TableIterator::from(self)
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WeightType {
    Samples,
    TracingMs,
    Bytes,
}

type Weight = i64;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SamplesLikeTableShape {
    pub stack: ArrayQ<IndexIntoFrameTable>,
    pub time: Array<Milliseconds>,
    pub weight: Option<Weight>,
    pub weightType: WeightType,
    pub length: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SamplesTable {
    pub responsiveness: Option<ArrayQ<Milliseconds>>,
    pub eventDelay: Option<ArrayQ<Milliseconds>>,
    pub stack: ArrayQ<IndexIntoStackTable>,
    pub time: Array<Milliseconds>,
    pub weight: Option<Array<Weight>>,
    pub weightType: WeightType,
    pub threadCPUDelta: Option<ArrayQ<i32>>,
    pub threadId: Option<Array<Tid>>,
    pub length: u32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SampleTableEntry {
    pub responsiveness: Option<Milliseconds>,
    pub eventDelay: Option<Milliseconds>,
    pub stack: Option<IndexIntoStackTable>,
    pub time: Milliseconds,
    pub weight: Option<Weight>,
    pub weightType: WeightType,
    pub threadCPUDelta: Option<i32>,
    pub threadId: Option<Tid>,
}

impl TableLookup<SampleTableEntry> for SamplesTable {
    fn length(&self) -> usize {
        self.length as usize
    }
    fn lookup(&self, ix: usize) -> SampleTableEntry {
        SampleTableEntry {
            responsiveness: self.responsiveness.as_ref().and_then(|a| a[ix]),
            eventDelay: self.eventDelay.as_ref().and_then(|a| a[ix]),
            stack: self.stack[ix],
            time: self.time[ix],
            weight: self.weight.as_ref().map(|a| a[ix]),
            weightType: self.weightType,
            threadCPUDelta: self.threadCPUDelta.as_ref().and_then(|a| a[ix]),
            threadId: self.threadId.as_ref().map(|a| a[ix].clone()),
        }
    }
    fn iter(&self) -> TableIterator<Self, SampleTableEntry>
    where
        Self: Sized,
    {
        TableIterator::from(self)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct JsAllocationsTable {
    pub time: Array<Milliseconds>,
    pub className: Array<String>,
    pub coarseType: Array<String>,
    pub weight: Array<u8>,
    pub weightType: WeightType,
    pub inNursery: Array<bool>,
    pub stack: ArrayQ<IndexIntoStackTable>,
    pub length: u32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UnbalancedNativeAllocationsTable {
    pub time: Array<Milliseconds>,
    pub weight: Array<u8>,
    pub weightType: WeightType,
    pub stack: ArrayQ<IndexIntoStackTable>,
    pub length: u32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BalancedNativeAllocationsTable {
    pub time: Array<Milliseconds>,
    pub weight: Array<u8>,
    pub weightType: WeightType,
    pub stack: ArrayQ<IndexIntoStackTable>,
    pub length: u32,
    pub memoryAddress: Array<u32>,
    pub threadId: Array<u32>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NativeAllocationsTable {
    UnbalancedNativeAllocationsTable(UnbalancedNativeAllocationsTable),
    BalancedNativeAllocationsTable(BalancedNativeAllocationsTable),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProcessType {
    Default,
    Plugin,
    Tab,
    IpdlUnitTest,
    GeckoMediaPlugin,
    GPU,
    Pdfium,
    VR,
    Invalid,
    #[serde(other)]
    Other,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FrameTable {
    pub address: Array<TableAddress>,
    pub inlineDepth: Array<i32>,
    pub category: ArrayQ<IndexIntoCategoryList>,
    pub subcategory: ArrayQ<IndexIntoSubcategoryListForCategory>,
    pub func: ArrayQ<IndexIntoFuncTable>,
    pub nativeSymbol: ArrayQ<IndexIntoNativeSymbolTable>,
    pub innerWindowID: ArrayQ<InnerWindowID>,
    pub implementation: ArrayQ<IndexIntoStringTable>,
    pub line: ArrayQ<u32>,
    pub column: ArrayQ<u32>,
    pub length: u32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct FrameTableEntry {
    pub address: TableAddress,
    pub inlineDepth: i32,
    pub category: Option<IndexIntoCategoryList>,
    pub subcategory: Option<IndexIntoSubcategoryListForCategory>,
    pub func: Option<IndexIntoFuncTable>,
    pub nativeSymbol: Option<IndexIntoNativeSymbolTable>,
    pub innerWindowID: Option<InnerWindowID>,
    pub implementation: Option<IndexIntoStringTable>,
    pub line: Option<u32>,
    pub column: Option<u32>,
}

impl TableLookup<FrameTableEntry> for FrameTable {
    fn length(&self) -> usize {
        self.length as usize
    }
    fn lookup(&self, ix: usize) -> FrameTableEntry {
        FrameTableEntry {
            address: self.address[ix],
            inlineDepth: self.inlineDepth[ix],
            category: self.category[ix],
            subcategory: self.subcategory[ix],
            func: self.func[ix],
            nativeSymbol: self.nativeSymbol[ix],
            innerWindowID: self.innerWindowID[ix],
            implementation: self.implementation[ix],
            line: self.line[ix],
            column: self.column[ix],
        }
    }
    fn iter(&self) -> TableIterator<Self, FrameTableEntry>
    where
        Self: Sized,
    {
        TableIterator::from(self)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FuncTable {
    pub name: Array<IndexIntoStringTable>,
    pub isJS: Array<bool>,
    pub relevantForJS: Array<bool>,
    pub resource: Array<IndexIntoResourceTable>,
    pub fileName: ArrayQ<IndexIntoStringTable>,
    pub lineNumber: ArrayQ<u32>,
    pub columnNumber: ArrayQ<u32>,
    pub length: u32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NativeSymbolTable {
    pub libIndex: Array<IndexIntoLibs>,
    pub address: Array<Address>,
    pub name: Array<IndexIntoStringTable>,
    pub functionSize: ArrayQ<u8>,
    pub length: u32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NativeSymbolTableEntry {
    pub libIndex: IndexIntoLibs,
    pub address: Address,
    pub name: IndexIntoStringTable,
    pub functionSize: Option<u8>,
}

impl TableLookup<NativeSymbolTableEntry> for NativeSymbolTable {
    fn length(&self) -> usize {
        self.length as usize
    }
    fn lookup(&self, ix: usize) -> NativeSymbolTableEntry {
        NativeSymbolTableEntry {
            libIndex: self.libIndex[ix],
            address: self.address[ix],
            name: self.name[ix],
            functionSize: self.functionSize[ix],
        }
    }
    fn iter(&self) -> TableIterator<Self, NativeSymbolTableEntry>
    where
        Self: Sized,
    {
        TableIterator::from(self)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceTable {
    pub length: u32,
    pub lib: ArrayQ<IndexIntoLibs>,
    pub name: Array<IndexIntoStringTable>,
    pub host: ArrayQ<IndexIntoStringTable>,
    #[serde(rename = "type")]
    pub ty: Array<resourceTypeEnum>,
}

// This is a lot simpler than the JS implementation, but lacks the "reverse lookup" optimisation
pub type UniqueStringArray = Array<String>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Thread {
    pub processType: ProcessType,
    pub processStartupTime: Milliseconds,
    pub processShutdownTime: Option<Milliseconds>,
    pub registerTime: Option<Milliseconds>,
    pub unregisterTime: Option<Milliseconds>,
    // TODO: Implement:
    // pub pausedRanges: Array<PausedRange>,
    pub name: String,
    pub isMainThread: bool,
    // TODO: Implement
    #[serde(rename = "eTLD+1")]
    pub eTLDone: Option<String>,
    pub processName: Option<String>,
    pub isJsTracer: Option<bool>,
    pub pid: Pid,
    pub tid: Tid,

    pub samples: SamplesTable,
    // TODO: Implement parsing for these structures
    // pub jsAllocations: Option<JsAllocationsTable>,
    // pub nativeAllocations: Option<NativeAllocationsTable>,
    // pub markers: RawMarkerTable,
    pub stackTable: StackTable,
    pub frameTable: FrameTable,
    // For some reason, this is sometimes generated as "stringArray"
    #[serde(alias = "stringArray")]
    #[serde(alias = "stringTable")]
    pub stringTable: UniqueStringArray,
    pub funcTable: FuncTable,
    // pub resourceTable: ResourceTable,
    pub nativeSymbols: NativeSymbolTable,
    // pub jsTracer: Option<JsTracerTable>,
    pub isPrivateBrowsing: Option<bool>,
    pub userContextId: Option<u32>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Lib {
    pub arch: Option<String>,
    pub name: String,
    pub path: String,
    pub debugName: String,
    pub debugPath: String,
    pub breakpadId: String,
    pub codeId: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    // TODO: Implement
    // pub meta: serde_json::Value,
    pub libs: Array<Lib>,
    pub pages: serde_json::Value,
    // pub counters: serde_json::Value,
    // pub profilerOverhead: serde_json::Value,
    pub threads: Array<Thread>,
    // pub profilingLog: Option<serde_json::Value>,
    // pub profilerGatheringLog: Option<serde_json::Value>,
}
