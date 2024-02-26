//! Control flow (re-)actions to `Commands`.
//!
//! These actions indicate to the communications partner how to react regarding
//! the last command.

mod bidirectional;
mod quit;
mod to_mta_only;

use enum_dispatch::enum_dispatch;

pub use self::bidirectional::{Abort, Continue};
pub use self::quit::{Quit, QuitNc};
pub use self::to_mta_only::{Discard, Reject, Replycode, Skip, Tempfail};

/// All control-flow actions combined
///
/// See the contained variants for more.
#[allow(missing_docs)]
#[enum_dispatch]
#[derive(Debug, Clone)]
pub enum Action {
    Continue,
    Abort,

    Discard,
    Reject,
    Tempfail,
    Skip,
    Replycode,

    Quit,
    QuitNc,
}
