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

impl VersionTriple {
    /// Create a version triple from its major, minor and micro values.
    #[inline]
    pub const fn new(major: u64, minor: u64, micro: u64) -> Self {
        VersionTriple {
            major,
            minor,
            micro,
        }
    }
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

/// An enumeration of the actions taken when the watchdog device's timer is expired.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WatchdogAction {
    /// System resets.
    Reset,
    /// System shutdown.
    ///
    /// Note that it is similar to powerdown, which tries to set to
    /// system status and notify guest.
    Shutdown,
    /// System poweroff, the emulator program exits.
    Poweroff,
    /// System pauses, similar to stop.
    Pause,
    /// System enters debug state.
    Debug,
    /// Nothing is done.
    None,
    /// A non-maskable interrupt is injected into the first VCPU
    /// (all VCPUS on x86).
    InjectNmi,
}

/// Possible QEMU actions upon guest reboot.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RebootAction {
    /// Reset the VM.
    Reset,
    /// Shutdown the VM and exit, according to the shutdown action.
    Shutdown,
}

/// Possible QEMU actions upon guest shutdown.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ShutdownAction {
    /// Shutdown the VM and exit.
    Poweroff,
    /// Pause the VM.
    Pause,
}

/// Panic action.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PanicAction {
    /// Continue VM execution.
    None,
    /// Pause the VM.
    Pause,
    /// Shutdown the VM and exit, according to the shutdown action.
    Shutdown,
    /// Shutdown the VM and exit with non-zero status.
    ExitFailure,
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

/// Arguments of 'watchdog-set-action'.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WatchdogSetActionParams {
    /// Value 'action'.
    pub action: WatchdogAction,
}

/// Set watchdog action.
#[inline]
pub fn watchdog_set_action(value: WatchdogAction) -> Command<WatchdogSetActionParams, ()> {
    Command {
        execute: "watchdog-set-action".to_string(),
        arguments: Some(WatchdogSetActionParams { action: value }),
        id: None,
    }
}

/// Arguments of 'set-action'.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SetActionParams {
    /// `RebootAction` action taken on guest reboot.
    pub reboot: Option<RebootAction>,
    /// `ShutdownAction` action taken on guest shutdown.
    pub shutdown: Option<ShutdownAction>,
    /// `PanicAction` action taken on guest panic.
    pub panic: Option<PanicAction>,
    /// `WatchdogAction` action taken when watchdog timer expires.
    pub watchdog: Option<WatchdogAction>,
}

/// Set the actions that will be taken by the emulator in response to guest events.
#[inline]
pub fn set_action(params: SetActionParams) -> Command<SetActionParams, ()> {
    Command {
        execute: "watchdog-set-action".to_string(),
        arguments: Some(params),
        id: None,
    }
}

// TODO other constructors.

#[cfg(test)]
mod tests {
    use super::{VersionInfo, VersionTriple, query_version};

    #[test]
    fn struct_version_info() {
        let value_1 = VersionTriple {
            major: 5,
            minor: 6,
            micro: 7,
        };
        let value_2 = VersionTriple::new(5, 6, 7);
        assert_eq!(value_2, value_1)
    }

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
            qemu: VersionTriple::new(0, 11, 5),
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

    // TODO new_watchdog_set_action
    // TODO new_set_action
}
