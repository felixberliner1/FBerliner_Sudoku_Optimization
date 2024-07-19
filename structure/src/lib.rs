#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::{HashMap, HashSet};
use toml::{Value, Table};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Structure {
    pub description: String,
    pub size: usize,
    pub constants: HashMap<String, usize>,
    pub relations: HashMap<String, Vec<Vec<usize>>>
}

#[cfg(test)]
mod structure_tests {
    /*use super::*;

    fn seg1() -> String {
r#"description = "segment {0,1,2}"
size = 3

[constants]
MIN = 0
MAX = 2

[relations]
S = [[0, 1], [1, 2]]
"<" = [[0, 1], [0, 2], [1, 2]]
"#.to_string()
    }

    fn structure1() -> Structure {
	Structure {
	    description: "segment {0,1,2}".to_string(),
	    size: 3,
	    constants: HashMap::from([
		("MIN".to_string(), 0),
		("MAX".to_string(), 2)
	    ]),
	    relations: HashMap::from([
		("S".to_string(), vec![vec![0, 1], vec![1, 2]]),
		("<".to_string(), vec![vec![0, 1], vec![0, 2], vec![1, 2]])
	    ])
	}
    }
    
    #[test]
    fn seg1_to_table() {
	let seg1 = seg1();

	let table1: Table = toml::from_str(&seg1).unwrap();

	println!("Table: {:?}", table1);
	
	assert_eq!(table1["size"].as_integer().unwrap(), 3);
	assert_eq!(table1["constants"]["MIN"].as_integer().unwrap(), 0);
	assert_eq!(table1["relations"]["<"].as_array().unwrap()[0],
		   Value::Array(vec![Value::Integer(0), Value::Integer(1)]));
    }

    #[test]
    fn deserialize_seg1() {
	let seg1 = seg1();

	let structure1: Structure = toml::from_str(&seg1).unwrap();

	println!("{:?}", structure1);

	assert_eq!(structure1.description, "segment {0,1,2}");
	assert_eq!(structure1.size, 3);
	assert_eq!(structure1.constants["MIN"], 0);
	assert_eq!(structure1.constants["MAX"], 2);
	assert_eq!(structure1.relations["<"],
		   vec![vec![0, 1], vec![0, 2], vec![1, 2]]);
	assert_eq!(structure1.relations["S"],
		   vec![vec![0, 1], vec![1, 2]]);
	}

    #[test]
    fn serde_structure1() {
	let structure1 = structure1();
	let seg2 = toml::to_string(&structure1).unwrap();

	println!("String:\n{}", seg2);

	let structure2 = toml::from_str(&seg2).unwrap();

	assert_eq!(structure1, structure2)
    }*/
}
