// use serde::Deserialize;

// #[derive(Deserialize, Debug)]
// struct Request {
//     // Use the result of a function as the default if "resource" is
//     // not included in the input.
//     #[serde(default = "default_resource")]
//     resource: String,

//     // Use the type's implementation of std::default::Default if
//     // "timeout" is not included in the input.
//     #[serde(default)]
//     timeout: Timeout,

//     // Use a method from the type as the default if "priority" is not
//     // included in the input. This may also be a trait method.
//     #[serde(default = "Priority::lowest")]
//     priority: Priority,
// }

// fn default_resource() -> String {
//     "/".to_string()
// }

// /// Timeout in seconds.
// #[derive(Deserialize, Debug)]
// struct Timeout(u32);
// impl Default for Timeout {
//     fn default() -> Self {
//         Timeout(30)
//     }
// }

// #[derive(Deserialize, Debug)]
// enum Priority { ExtraHigh, High, Normal, Low, ExtraLow }
// impl Priority {
//     fn lowest() -> Self { Priority::ExtraLow }
// }

// fn main() {
//     let json = r#"
//         [
//           {
//             "resource": "/users"
//           },
//           {
//             "timeout": 5,
//             "priority": "High"
//           }
//         ]
//     "#;

//     let requests: Vec<Request> = serde_json::from_str(json).unwrap();

//     // The first request has resource="/users", timeout=30, priority=ExtraLow
//     println!("{:?}", requests[0]);

//     // The second request has resource="/", timeout=5, priority=High
//     println!("{:?}", requests[1]);
// }

use serde::{Deserialize, Serialize};

use std::collections::BTreeMap as Map;

#[derive(Serialize, Debug, Deserialize)]
struct Resource {
    // Always serialized.
    name: String,

    // Never serialized.
    #[serde(skip_serializing, skip_deserializing)]
    #[allow(dead_code)]
    hash: String,

    // Use a method to decide whether the field should be skipped.
    #[serde(skip_serializing_if = "Map::is_empty", skip_deserializing)]
    metadata: Map<String, String>,
}

fn main() {
    let resources = vec![
        Resource {
            name: "Stack Overflow".to_string(),
            hash: "b6469c3f31653d281bbbfa6f94d60fea130abe38".to_string(),
            metadata: Map::new(),
        },
        Resource {
            name: "GitHub".to_string(),
            hash: "5cb7a0c47e53854cd00e1a968de5abce1c124601".to_string(),
            metadata: {
                let mut metadata = Map::new();
                metadata.insert("headquarters".to_string(),
                                "San Francisco".to_string());
                metadata
            },
        },
    ];

    let json = serde_json::to_string_pretty(&resources).unwrap();
    println!("{}", json);
    
    let result: Vec<Resource>  = serde_json::from_str(json.as_str()).unwrap();

    println!("{:#?}",result);

    // Prints:
    //
    //    [
    //      {
    //        "name": "Stack Overflow"
    //      },
    //      {
    //        "name": "GitHub",
    //        "metadata": {
    //          "headquarters": "San Francisco"
    //        }
    //      }
    //    ]
    
}