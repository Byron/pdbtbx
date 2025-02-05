//! # pdbtbx (PDB Toolbox)
//!
//! A library to work with crystallographic Protein DataBank files. It can parse the main part
//! of the PDB format (it is actively in development so more will follow). After parsing the
//! structure is accessible with an API loosely based on CCTBX. The resulting structures can
//! be saved in a valid PDB file for use in other software.
//!
//! ## Goals
//! This library is designed to be a dependable, safe, stable and fast way of handling PDB files
//! in idiomatic Rust. It is the goal to be very community driven, to make it into a project that
//! is as useful to everyone as possible, while keeping true to its core principles.
//!
//! ## Why
//! As Rust is a very recent language there is not a lot of support for scientific work in Rust
//! in comparison to languages that are used much longer (see Python). I think that using Rust
//! would have huge benefits over other languages (especially Python) in bigger scientific
//! projects. Writing a library that makes more scientific work with Rust possible makes it
//! easier for scientists to start using Rust, which I want to support.
//!
//! ## How to use it
//! The following example opens a pdb file (`1ubq.pdb`). Removes all `H` atoms. Calculates the
//! average B factor (or temperature factor) and prints that. It also saves the resulting PDB
//! to a file.
//!
//! ```
//! use pdbtbx;
//! let (mut pdb, _errors) = pdbtbx::parse("example-pdbs/1ubq.pdb").unwrap();
//! pdb.remove_atoms_by(|atom| atom.element() == "H"); // Remove all H atoms
//!
//! let mut avg_b_factor = 0.0;
//! for atom in pdb.atoms() { // Iterate over all atoms in the structure (not the HETATMs)
//!     avg_b_factor += atom.b_factor();
//! }
//! avg_b_factor /= pdb.atom_count() as f64;
//!
//! println!("The average B factor of the protein is: {}", avg_b_factor);
//! pdbtbx::save(&pdb, "dump/1ubq.pdb");
//! ```

mod error;
mod read;
mod reference_tables;
mod save;
mod structs;
mod transformation;
mod validate;

pub use error::*;
pub use read::parse;
pub use save::save;
pub use structs::*;
pub use transformation::*;
pub use validate::validate;
