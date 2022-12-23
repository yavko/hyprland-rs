//! # Data module
//!
//! This module provides functions for getting information on the compositor
//!
//! ## Usage
//!
//! here is a example of every function in use! (blocking)
//! ```rust
//! use hyprland::data::{
//!     Monitors,
//!     Workspaces,
//!     Clients,
//!     ActiveWindow,
//!     Layers,
//!     Devices
//! };
//! use hyprland::prelude::*;
//! use hyprland::shared::HResult;
//!
//! fn main() -> HResult<()> {
//!     let monitors = Monitors::get()?.vec();
//!     println!("{monitors:#?}");
//!
//!     let workspaces = Workspaces::get()?.vec();
//!     println!("{workspaces:#?}");
//!
//!     let clients = Clients::get()?.vec();
//!     println!("{clients:#?}");
//!
//!     let active_window = ActiveWindow::get()?;
//!     println!("{active_window:#?}");
//!
//!     let layers = Layers::get()?;
//!     println!("{layers:#?}");
//!
//!     let devices = Devices::get()?;
//!     println!("{devices:#?}");
//!
//!     Ok(())
//! }
//! ```

#[macro_use]
mod macros;

use crate::shared::*;
use std::collections::HashMap;

mod regular;

/// Helpers data commands, these use other hyprctl commands to create new ones!
mod helpers;

pub use crate::data::helpers::*;

pub use crate::data::regular::*;

//// This module provides async function calls
//pub mod asynchronous;

//// This module provides blocking function calls
//pub mod blocking;