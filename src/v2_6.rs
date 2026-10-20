//! UCI 2.6
//!
#![doc = r#"DISTRIBUTION STATEMENTS

DISCLAIMER: The discussion of non-federal entities, methods, products, or services does not imply any endorsement by the United States Government, the Department of Defense, or the Department of the Air Force.

Distribution Statement A. Approved for public release: distribution is unlimited.
			DISCLAIMER:
			*** THIS IS NOT INTENDED FOR OPERATIONAL USE. ***
			*** USE OF THE INFORMATION IN THIS DOCUMENT IS AT YOUR OWN RISK. ***"#]

#![allow(dead_code)]
#[macro_use]
pub mod serde_utils;
pub mod choices;
pub mod common;
pub mod elements;
pub mod enums;
pub mod traits;
pub mod types;

pub const SCHEMA_VERSION: &'static str = r#"002.6.0"#;

