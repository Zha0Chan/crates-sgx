//! parsers recognizing numbers

#[macro_use]
mod macros;

pub mod complete;
pub mod streaming;

/// Configurable endianness
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Endianness {
  /// big endian
  Big,
  /// little endian
  Little,
}
