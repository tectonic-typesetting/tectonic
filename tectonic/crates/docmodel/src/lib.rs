// Copyright 2020-2021 the Tectonic Project
// Licensed under the MIT License.

#![deny(missing_docs)]

//! The Tectonic document model and its serialization into `Tectonic.toml`.
//!
//! This crate provides data structures and serialization support for the
//! Tectonic document model and its expression in the `Tectonic.toml` file. It
//! only provides data access: you can use this crate without needing to link
//! with the full Tectonic engines and all of the dependencies they drag in. The
//! main `tectonic` crate provides extension traits that attach actual
//! document-processing capabilities to these data structures.
//!
//! Your primary entrypoint to this crate will likely be
//! [`workspace::Workspace::open_from_environment`], which will attempt to load
//! up a workspace by searching the process’ current directory and parents for a
//! `Tectonic.toml` file. There is also [`workspace::WorkspaceCreator`] for
//! creating new workspaces from scratch.

pub mod document;
mod syntax;
pub mod workspace;
