//! Protocol generic specification.
use crate::qmp_monitor::VersionInfo;
use serde::{Deserialize, Serialize};

/// Server greeting structure.
///
/// Right when connected the Server will issue a greeting message,
/// which signals that the connection has been successfully
/// established and that the Server is ready for capabilities
/// negotiation.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ServerGreeting {
    /// Server's version information.
    ///
    /// The format is the same as for the 'query-version' command.
    pub version: VersionInfo,
    /// Specifies the availability of features beyond the baseline specification.
    ///
    /// The order of elements in this array has no particular significance.
    pub capabilities: Vec<String>,
}

/// An in-band command.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Command<T, U> {
    /// Identifies the command to be executed by the server.
    pub execute: String,
    /// The `arguments` member is used to pass any arguments
    /// required for the execution of the command.
    ///
    /// It is optional when no arguments are required.
    ///
    /// Each command documents what contents will be considered valid
    /// when handling the json-argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<T>,
    /// The `id` member is a transaction identification associated with the command execution.
    ///
    /// It is optional and will be part of the response if provided.
    ///
    /// The id member can be any json-value. A json-number incremented for each successive command works fine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<U>,
}

/// An out-of-band command.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OobCommand<T, U> {
    /// Identifies the out-of-band execution command to be executed by the server.
    #[serde(rename = "exec-oob")]
    pub exec_oob: String,
    /// The `arguments` member is used to pass any arguments
    /// required for the execution of the command.
    ///
    /// It is optional when no arguments are required.
    ///
    /// Each command documents what contents will be considered valid
    /// when handling the json-argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<T>,
    /// The `id` member is a transaction identification associated with the command execution.
    ///
    /// It is optional and will be part of the response if provided.
    ///
    /// The id member can be any json-value. A json-number incremented for each successive command works fine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<U>,
}

/// A response to a command.
///
/// There are two possible responses which the Server will issue as
/// the result of a command execution: success or error.
///
/// As long as the commands were issued with a proper id field,
/// then the same id field will be attached in the corresponding
/// response message so that requests and responses can match.
/// Clients should drop all the responses that have an unknown id field.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Response<T, U> {
    /// The response content, which contains either a success or an error value.
    #[serde(flatten)]
    pub response: ReturnOrError<T>,
    /// The id member contains the transaction identification associated
    /// with the command execution if issued by the Client.
    pub id: U,
}

/// Either this response succeeds or fails with an error.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReturnOrError<T> {
    /// The response succeeded.
    Return {
        /// The return member contains the data returned by the command.
        ///
        /// It is defined on a per-command basis
        /// (usually a json-object or json-array of json-objects,
        /// but sometimes a json-number, json-string, or json-array of json-strings);
        /// it is an empty json-object if the command does not return data.
        #[serde(rename = "return")]
        value: T,
    },
    /// The response failed with an error.
    Error {
        /// The `class` member contains the error class name (eg. "GenericError").
        #[serde(rename = "error")]
        class: String,
        /// The `desc` member is a human-readable error message. Clients should not attempt to parse this message.
        #[serde(rename = "desc")]
        description: String,
    },
}

/// Asynchronous events.
///
/// As a result of state changes, the Server may send messages unilaterally
/// to the Client at any time, when not in the middle of any other response.
/// They are called "asynchronous events".
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Event<T> {
    /// The `event` member contains the event's name.
    pub event: String,
    /// The `data` member contains event specific data, which is defined in a per-event basis.
    /// It is optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    /// The timestamp member contains the exact time of when the event occurred in the Server.
    pub timestamp: Timestamp,
}

/// The exact time of when the event occurred in the Server.
///
/// It is a fixed json-object with time in seconds and microseconds relative to the Unix Epoch (1 Jan 1970);
/// if there is a failure to retrieve host time, both members of the timestamp will be set to -1.
///
/// Some events are rate-limited to at most one per second. If additional "similar" events arrive within one second,
/// all but the last one are dropped, and the last one is delayed.
/// "Similar" normally means same event type.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Timestamp {
    /// Number of seconds from the Unix Epoch.
    pub seconds: u64,
    /// Number of microseconds.
    pub microseconds: u64,
}

// TODO tests for generic structures. Refer to 'QMP Examples' section.
