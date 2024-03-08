# Miltr Common

Single source of truth for `miltr-server` and `miltr-client` implementations.

The milter protocol basically works as follows:

1. A client establishes a connection to a server, negotiating Options
   via both sending [`optneg::OptNeg`] packages.
2. The client send a [`commands::Command`] for each SMTP command it receives
3. The server responds to each of those commands with an [`actions::Action`]
4. After [`commands::EndOfBody`] the server responds with a list of
   [`modifications::ModificationAction`] to instruct the client what to
   change in the processed mail.

This is what's contained within the [`actions`], [`commands`], [`modifications`]
and [`optneg`] module.

As all packages share some logic on how to be (de-)serialized, modules
[`encoding`] and [`decoding`] contain the implementation of that.

All parsing is based on splitting [`bytes::BytesMut`] into smaller parts.
