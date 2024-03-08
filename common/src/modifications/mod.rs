//! Response containing modified data
//!
//! Only to an end-of-body the milter can respond with change requests.
//! These are modification actions.

pub mod body;
pub mod headers;
pub mod quarantine;
pub mod recipients;

use enum_dispatch::enum_dispatch;

use super::{
    actions::{Action, Continue},
    ServerMessage,
};

use crate::encoding::Writable;
use crate::{actions::Abort, optneg::Capability};
use bytes::BytesMut;

use body::ReplaceBody;
use headers::{AddHeader, ChangeHeader, InsertHeader};
use quarantine::Quarantine;
use recipients::{AddRecipient, DeleteRecipient};

/// A container for multiple modification requests towards the milter client.
///
/// ```
/// use miltr_common::modifications::{ModificationResponse, headers::AddHeader};
///
/// let mut builder = ModificationResponse::builder();
/// builder.push(AddHeader::new(
///      "Test Add Header".as_bytes(),
///      "Add Header Value".as_bytes(),
///   ));
/// let response = builder.contin();
/// ```
///
/// # Note on Capabilities
/// While all [`ModificationAction`] can be pushed into this response,
/// they might not all be sent.
/// During option negotiation, client and server agree on supported
/// [`Capability`].
#[derive(Debug)]
pub struct ModificationResponse {
    modifications: Vec<ModificationAction>,
    final_action: Action,
}

impl ModificationResponse {
    /// Create a builder to assemble a modification response.
    #[must_use]
    pub fn builder() -> ModificationResponseBuilder {
        ModificationResponseBuilder {
            modifications: Vec::default(),
        }
    }

    /// Create an empty `ModificationResponse` just to continue
    #[must_use]
    pub fn empty_continue() -> Self {
        Self {
            modifications: Vec::new(),
            final_action: Continue.into(),
        }
    }

    /// Filter modification actions in `self`, keep only those which have been
    /// allowed by the specified `capabilities`.
    pub fn filter_mods_by_caps(&mut self, capabilities: Capability) {
        self.modifications
            .retain(|m| Self::mod_matches_caps(m, capabilities));
    }

    /// Returns true, if a single modification action matches the set `capabilities`
    fn mod_matches_caps(modification: &ModificationAction, capabilities: Capability) -> bool {
        match modification {
            ModificationAction::AddHeader(_) => capabilities.contains(Capability::SMFIF_ADDHDRS),
            ModificationAction::ReplaceBody(_) => capabilities.contains(Capability::SMFIF_CHGBODY),
            ModificationAction::AddRecipient(_) => capabilities.contains(Capability::SMFIF_ADDRCPT),
            ModificationAction::DeleteRecipient(_) => {
                capabilities.contains(Capability::SMFIF_DELRCPT)
            }
            ModificationAction::ChangeHeader(_) | ModificationAction::InsertHeader(_) => {
                capabilities.contains(Capability::SMFIF_CHGHDRS)
            }
            ModificationAction::Quarantine(_) => {
                capabilities.contains(Capability::SMFIF_QUARANTINE)
            }
        }
    }

    /// Get the received modification actions
    #[must_use]
    pub fn modifications(&self) -> &[ModificationAction] {
        self.modifications.as_ref()
    }

    /// Get the final action to be done to the mail
    #[must_use]
    pub fn final_action(&self) -> &Action {
        &self.final_action
    }
}

impl From<ModificationResponse> for Vec<ServerMessage> {
    fn from(value: ModificationResponse) -> Self {
        let mut resp: Vec<ServerMessage> = Vec::with_capacity(value.modifications.len() + 1);
        resp.extend(
            value
                .modifications
                .into_iter()
                .map(ServerMessage::ModificationAction),
        );
        resp.push(ServerMessage::Action(value.final_action));
        resp
    }
}

/// Gather up Modification actions to send to the milter client
#[derive(Debug, Clone)]
pub struct ModificationResponseBuilder {
    modifications: Vec<ModificationAction>,
}

impl ModificationResponseBuilder {
    /// Push another modification action onto the builder
    pub fn push<M: Into<ModificationAction>>(&mut self, mod_action: M) {
        self.modifications.push(mod_action.into());
    }

    /// Send the `Abort` command to the milter client
    #[must_use]
    pub fn abort(self) -> ModificationResponse {
        self.build(Abort)
    }

    /// Send a `Continue` command to the milter client with all set
    /// modification responses.
    #[must_use]
    pub fn contin(self) -> ModificationResponse {
        self.build(Continue)
    }

    /// Finalize into a [`ModificationResponse`] with a final action
    #[must_use]
    pub fn build<A: Into<Action>>(self, final_action: A) -> ModificationResponse {
        ModificationResponse {
            modifications: self.modifications,
            final_action: final_action.into(),
        }
    }
}

/// The container of possible milter modification actions
#[enum_dispatch]
#[cfg_attr(feature = "tracing", derive(strum::Display))]
#[derive(Debug, Clone)]
pub enum ModificationAction {
    /// Add recipient
    AddRecipient,
    /// Delete recipient
    DeleteRecipient,
    // /* add recipient (incl. ESMTP args) */
    // currently not supported, feel free to implement
    // SmfirAddrcptPar,
    // /* 421: shutdown (internal to MTA) */
    // Not implemented in Milter
    // SmfirShutdown,
    /// Replace mail body
    ReplaceBody,
    // /* change envelope sender (from) */
    // currently not supported, feel free to implement
    // SmfirChgfrom,
    // /* cause a connection failure */
    // currently not supported, feel free to implement. But why would you
    // need the connection to fail? Please, at least try to reason why you
    // need this
    // SmfirConnFail,
    /// Add an arbitrary header
    AddHeader,
    /// Insert the header at a specific place
    InsertHeader,
    // /* set list of symbols (macros) */
    // SmfirSetsymlist,
    /// Change an existing header
    ChangeHeader,
    // /* progress */
    // currently not supported, feel free to implement. May be a bit complicated
    // and config needed to handle timeouts and when to send and stuff
    // SmfirProgress,
    /// Quarantine this mail
    Quarantine,
}
