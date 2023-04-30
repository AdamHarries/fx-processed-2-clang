use assert_json_diff::{assert_json_eq, assert_json_include};
use debugid::DebugId;
use serde_json::json;

extern crate fx_processed_to_clang as fptc;
// Tests "borrowed" from fxprof-processed-profile in samply

fn serialise_deserialise_and_compare(original: serde_json::Value) {
    let simple_dump = serde_json::to_string(&original).expect("Could not dump to string");
    let parsed_profile: fptc::fx_processed_profile::Profile =
        serde_json::from_str(simple_dump.as_str()).expect("Error parsing json");
    let serialized_profile = serde_json::to_string(&parsed_profile).expect("Error dumping json");
    let re_parsed_profile: serde_json::Value = serde_json::from_str(serialized_profile.as_str())
        .expect("Failed to re-parse generated json");
    assert_json_eq!(original, re_parsed_profile);
}

#[test]
fn profile_without_js() {
    // eprintln!("{}", serde_json::to_string_pretty(&profile).unwrap());
    serialise_deserialise_and_compare(json!(
      {
        "meta": {
          "categories": [
            {
              "name": "Other",
              "color": "grey",
              "subcategories": [
                "Other"
              ]
            },
            {
              "name": "Regular",
              "color": "blue",
              "subcategories": [
                "Other"
              ]
            }
          ],
          "debug": false,
          "extensions": {
            "baseURL": [],
            "id": [],
            "length": 0,
            "name": []
          },
          "interval": 1.0,
          "preprocessedProfileVersion": 46,
          "processType": 0,
          "product": "test",
          "sampleUnits": {
            "eventDelay": "ms",
            "threadCPUDelta": "µs",
            "time": "ms"
          },
          "startTime": 1636162232627.0,
          "symbolicated": false,
          "pausedRanges": [],
          "version": 24,
          "usesOnlyOneStackType": true,
          "doesNotUseFrameImplementation": true,
          "sourceCodeIsNotOnSearchfox": true,
          "markerSchema": [
            {
              "name": "Text",
              "display": [
                "marker-chart",
                "marker-table"
              ],
              "chartLabel": "{marker.data.name}",
              "tableLabel": "{marker.name} - {marker.data.name}",
              "data": [
                {
                  "key": "name",
                  "label": "Details",
                  "format": "string",
                  "searchable": true
                }
              ]
            },
            {
              "name": "custom",
              "display": [
                "marker-chart",
                "marker-table"
              ],
              "tooltipLabel": "Custom tooltip label",
              "data": [
                {
                  "key": "eventName",
                  "label": "Event name",
                  "format": "string",
                  "searchable": true
                },
                {
                  "key": "allocationSize",
                  "label": "Allocation size",
                  "format": "bytes",
                  "searchable": true
                },
                {
                  "key": "url",
                  "label": "URL",
                  "format": "url",
                  "searchable": true
                },
                {
                  "key": "latency",
                  "label": "Latency",
                  "format": "duration",
                  "searchable": true
                },
                {
                  "label": "Description",
                  "value": "This is a test marker with a custom schema."
                }
              ]
            }
          ]
        },
        "libs": [
          {
            "name": "dump_syms",
            "path": "/home/mstange/code/dump_syms/target/release/dump_syms",
            "debugName": "dump_syms",
            "debugPath": "/home/mstange/code/dump_syms/target/release/dump_syms",
            "breakpadId": "5C0A0D51EA1980DF43F203B4525BE9BE0",
            "codeId": "510d0a5c19eadf8043f203b4525be9be3dcb9554",
            "arch": null
          },
          {
            "name": "libc.so.6",
            "path": "/usr/lib/x86_64-linux-gnu/libc.so.6",
            "debugName": "libc.so.6",
            "debugPath": "/usr/lib/x86_64-linux-gnu/libc.so.6",
            "breakpadId": "1629FCF0BE5C8860C0E1ADF03B0048FB0",
            "codeId": "f0fc29165cbe6088c0e1adf03b0048fbecbc003a",
            "arch": null
          }
        ],
        "threads": [
          {
            "frameTable": {
              "length": 16,
              "address": [
                -1,
                796420,
                911223,
                1332248,
                2354017,
                2452862,
                1700071,
                172156,
                1075602,
                905942,
                979918,
                2437518,
                1405368,
                737506,
                2586868,
                674246
              ],
              "inlineDepth": [
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0
              ],
              "category": [
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1
              ],
              "subcategory": [
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0
              ],
              "func": [
                0,
                1,
                2,
                3,
                4,
                5,
                6,
                7,
                8,
                9,
                10,
                11,
                12,
                13,
                14,
                15
              ],
              "nativeSymbol": [
                null,
                null,
                null,
                null,
                null,
                null,
                0,
                1,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                2
              ],
              "innerWindowID": [
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null
              ],
              "implementation": [
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null
              ],
              "line": [
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null
              ],
              "column": [
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null
              ],
              // "optimizations": [
              //   null,
              //   null,
              //   null,
              //   null,
              //   null,
              //   null,
              //   null,
              //   null,
              //   null,
              //   null,
              //   null,
              //   null,
              //   null,
              //   null,
              //   null,
              //   null
              // ]
            },
            "funcTable": {
              "length": 16,
              "name": [
                0,
                2,
                3,
                4,
                5,
                6,
                8,
                9,
                10,
                11,
                12,
                13,
                14,
                15,
                16,
                17
              ],
              "isJS": [
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false
              ],
              "relevantForJS": [
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false
              ],
              "resource": [
                -1,
                0,
                0,
                0,
                0,
                0,
                1,
                1,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                1
              ],
              "fileName": [
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null
              ],
              "lineNumber": [
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null
              ],
              "columnNumber": [
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null
              ]
            },
            // "markers": {
            //   "length": 2,
            //   "category": [
            //     0,
            //     0
            //   ],
            //   "data": [
            //     {
            //       "name": "Hello world!",
            //       "type": "Text"
            //     },
            //     {
            //       "allocationSize": 512000,
            //       "eventName": "My event",
            //       "latency": 123.0,
            //       "type": "custom",
            //       "url": "https://mozilla.org/"
            //     }
            //   ],
            //   "endTime": [
            //     0.0,
            //     2.0
            //   ],
            //   "name": [
            //     18,
            //     19
            //   ],
            //   "phase": [
            //     0,
            //     1
            //   ],
            //   "startTime": [
            //     0.0,
            //     0.0
            //   ]
            // },
            "name": "test",
            "isMainThread": true,
            "nativeSymbols": {
              "length": 3,
              "address": [
                1700001,
                172156,
                674226
              ],
              "functionSize": [
                180,
                20,
                44
              ],
              "libIndex": [
                1,
                1,
                1
              ],
              "name": [
                8,
                9,
                17
              ]
            },
            "pausedRanges": [],
            "pid": "123",
            "processName": "test",
            "processShutdownTime": null,
            "processStartupTime": 0.0,
            "processType": "default",
            "registerTime": 0.0,
            "resourceTable": {
              "length": 2,
              "lib": [
                0,
                1
              ],
              "name": [
                1,
                7
              ],
              "host": [
                null,
                null
              ],
              "type": [
                1,
                1
              ]
            },
            "samples": {
              "length": 4,
              "stack": [
                null,
                6,
                11,
                15
              ],
              "time": [
                0.0,
                1.0,
                2.0,
                3.0
              ],
              "weight": [
                1,
                1,
                1,
                1
              ],
              "weightType": "samples",
              "threadCPUDelta": [
                0,
                0,
                0,
                0
              ]
            },
            "stackTable": {
              "length": 16,
              "prefix": [
                null,
                0,
                1,
                2,
                3,
                4,
                5,
                1,
                7,
                8,
                9,
                10,
                7,
                12,
                13,
                14
              ],
              "frame": [
                0,
                1,
                2,
                3,
                4,
                5,
                6,
                7,
                8,
                9,
                10,
                11,
                12,
                13,
                14,
                15
              ],
              "category": [
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1
              ],
              "subcategory": [
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0
              ]
            },
            "stringTable": [
              "0x7ffdb4824837",
              "dump_syms",
              "0xc2704",
              "0xde777",
              "0x145418",
              "0x23eb61",
              "0x256d7e",
              "libc.so.6",
              "libc_symbol_1",
              "libc_symbol_2",
              "0x106992",
              "0xdd2d6",
              "0xef3ce",
              "0x25318e",
              "0x1571b8",
              "0xb40e2",
              "0x2778f4",
              "libc_symbol_3",
              "Experimental",
              "CustomName"
            ],
            "tid": "12345",
            "unregisterTime": null
          }
        ],
        "pages": [],
        "profilerOverhead": [],
        // "counters": [
          // {
          //   "category": "Memory",
          //   "name": "malloc",
          //   "description": "Amount of allocated memory",
          //   "mainThreadIndex": 0,
          //   "pid": "123",
          //   "sampleGroups": [
          //     {
          //       "id": 0,
          //       "samples": {
          //         "length": 3,
          //         "count": [
          //           0.0,
          //           1000.0,
          //           800.0
          //         ],
          //         "number": [
          //           0,
          //           2,
          //           1
          //         ],
          //         "time": [
          //           0.0,
          //           1.0,
          //           2.0
          //         ]
          //       }
          //     }
          //   ]
        //   }
        // ]
      }
    ));
}

#[test]
fn profile_with_js() {
    // eprintln!("{}", serde_json::to_string_pretty(&profile).unwrap());
    serialise_deserialise_and_compare(json!(
      {
        "meta": {
          "categories": [
            {
              "name": "Other",
              "color": "grey",
              "subcategories": [
                "Other"
              ]
            },
            {
              "name": "Regular",
              "color": "green",
              "subcategories": [
                "Other"
              ]
            }
          ],
          "debug": false,
          "extensions": {
            "baseURL": [],
            "id": [],
            "length": 0,
            "name": []
          },
          "interval": 1.0,
          "preprocessedProfileVersion": 46,
          "processType": 0,
          "product": "test with js",
          "sampleUnits": {
            "eventDelay": "ms",
            "threadCPUDelta": "µs",
            "time": "ms"
          },
          "startTime": 1636162232627.0,
          "symbolicated": false,
          "pausedRanges": [],
          "version": 24,
          "usesOnlyOneStackType": false,
          "doesNotUseFrameImplementation": true,
          "sourceCodeIsNotOnSearchfox": true,
          "markerSchema": []
        },
        "libs": [],
        "threads": [
          {
            "frameTable": {
              "length": 2,
              "address": [
                -1,
                -1
              ],
              "inlineDepth": [
                0,
                0
              ],
              "category": [
                1,
                1
              ],
              "subcategory": [
                0,
                0
              ],
              "func": [
                0,
                1
              ],
              "nativeSymbol": [
                null,
                null
              ],
              "innerWindowID": [
                null,
                null
              ],
              "implementation": [
                null,
                null
              ],
              "line": [
                null,
                null
              ],
              "column": [
                null,
                null
              ],
              // "optimizations": [
              //   null,
              //   null
              // ]
            },
            "funcTable": {
              "length": 2,
              "name": [
                0,
                1
              ],
              "isJS": [
                true,
                false
              ],
              "relevantForJS": [
                false,
                false
              ],
              "resource": [
                -1,
                -1
              ],
              "fileName": [
                null,
                null
              ],
              "lineNumber": [
                null,
                null
              ],
              "columnNumber": [
                null,
                null
              ]
            },
            "markers": {
              "length": 0,
              "category": [],
              "data": [],
              "endTime": [],
              "name": [],
              "phase": [],
              "startTime": []
            },
            "name": "test2",
            "isMainThread": true,
            "nativeSymbols": {
              "length": 0,
              "address": [],
              "functionSize": [],
              "libIndex": [],
              "name": []
            },
            "pausedRanges": [],
            "pid": "123",
            "processName": "test2",
            "processShutdownTime": null,
            "processStartupTime": 0.0,
            "processType": "default",
            "registerTime": 0.0,
            "resourceTable": {
              "length": 0,
              "lib": [],
              "name": [],
              "host": [],
              "type": []
            },
            "samples": {
              "length": 1,
              "stack": [
                1
              ],
              "time": [
                1.0
              ],
              "weight": [
                1
              ],
              "weightType": "samples",
              "threadCPUDelta": [
                0
              ]
            },
            "stackTable": {
              "length": 2,
              "prefix": [
                null,
                0
              ],
              "frame": [
                0,
                1
              ],
              "category": [
                1,
                1
              ],
              "subcategory": [
                0,
                0
              ]
            },
            "stringTable": [
              "Some label string",
              "0x7f76b7ffc0e6"
            ],
            "tid": "12346",
            "unregisterTime": null
          }
        ],
        "pages": [],
        "profilerOverhead": [],
        "counters": []
      }
    ));
}
