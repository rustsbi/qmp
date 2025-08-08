//! QMP monitor control.
use serde::{Deserialize, Serialize};

use crate::generic::Command;

/// Enumeration of capabilities to be advertised during initial client connection.
///
/// Used for agreeing on particular QMP extension behaviors.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum QmpCapability {
    /// The QMP server supports "out-of-band" (OOB) command execution.
    Oob,
}

/// A three-part version number.
///
/// QEMU C code does not define the actural integer width of `VersionTriple`
/// members. Here we use `u64` as a temporary representation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VersionTriple {
    /// The major version number.
    pub major: u64,
    /// The minor version number.
    pub minor: u64,
    /// The micro version number.
    pub micro: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VersionInfo {
    /// The version of QEMU.
    ///
    /// By current convention, a micro version of 50 signifies a
    /// development branch. A micro version greater than or equal
    /// to 90 signifies a release candidate for the next minor
    /// version. A micro version of less than 50 signifies a stable
    /// release.
    pub qemu: VersionTriple,
    /// QEMU will always set this field to an empty string.
    ///
    /// Downstream versions of QEMU should set this to a non-empty
    /// string. The exact format depends on the downstream however
    /// it highly recommended that a unique name is used.
    pub package: String,
}

// TODO other structures.

/// Return the current version of QEMU.
#[inline]
pub fn query_version() -> Command<(), ()> {
    Command {
        execute: "query-version".to_string(),
        arguments: None,
        id: None,
    }
}

// TODO other constructors.

#[cfg(test)]
mod tests {
    use super::{VersionInfo, VersionTriple, query_version};
    #[test]
    fn query_version_result() {
        let string = r#"{
            "qemu":{
                "major":0,
                "minor":11,
                "micro":5
            },
            "package":""
        }"#;
        let value = VersionInfo {
            qemu: VersionTriple {
                major: 0,
                minor: 11,
                micro: 5,
            },
            package: "".to_string(),
        };
        assert_eq!(value, serde_json::from_str(string).unwrap());
        let compact_string = r#"{"qemu":{"major":0,"minor":11,"micro":5},"package":""}"#;
        assert_eq!(compact_string, serde_json::to_string(&value).unwrap());
    }
    
    #[test]
    fn new_query_version() {
        let cmd = query_version();
        let compact_string = r#"{"execute":"query-version"}"#;
        assert_eq!(compact_string, serde_json::to_string(&cmd).unwrap());
    }
}
