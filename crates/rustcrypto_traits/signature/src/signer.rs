//! Traits for generating digital signatures

use crate::{error::Error, Signature};

#[cfg(feature = "digest-preview")]
use crate::digest::Digest;

#[cfg(feature = "rand-preview")]
use crate::rand_core::{CryptoRng, RngCore};

/// Sign the provided message bytestring using `Self` (e.g. a cryptographic key
/// or connection to an HSM), returning a digital signature.
pub trait Signer<S: Signature> {
    /// Sign the given message and return a digital signature
    fn sign(&self, msg: &[u8]) -> S {
        self.try_sign(msg).expect("signature operation failed")
    }

    /// Attempt to sign the given message, returning a digital signature on
    /// success, or an error if something went wrong.
    ///
    /// The main intended use case for signing errors is when communicating
    /// with external signers, e.g. cloud KMS, HSMs, or other hardware tokens.
    fn try_sign(&self, msg: &[u8]) -> Result<S, Error>;
}

/// Sign the given prehashed message [`Digest`] using `Self`.
///
/// ## Notes
///
/// This trait is primarily intended for signature algorithms based on the
/// [Fiat-Shamir heuristic], a method for converting an interactive
/// challenge/response-based proof-of-knowledge protocol into an offline
/// digital signature through the use of a random oracle, i.e. a digest
/// function.
///
/// The security of such protocols critically rests upon the inability of
/// an attacker to solve for the output of the random oracle, as generally
/// otherwise such signature algorithms are a system of linear equations and
/// therefore doing so would allow the attacker to trivially forge signatures.
///
/// To prevent misuse which would potentially allow this to be possible, this
/// API accepts a [`Digest`] instance, rather than a raw digest value.
///
/// [Fiat-Shamir heuristic]: https://en.wikipedia.org/wiki/Fiat%E2%80%93Shamir_heuristic
#[cfg(feature = "digest-preview")]
#[cfg_attr(docsrs, doc(cfg(feature = "digest-preview")))]
pub trait DigestSigner<D, S>
where
    D: Digest,
    S: Signature,
{
    /// Sign the given prehashed message `Digest`, returning a signature.
    ///
    /// Panics in the event of a signing error.
    fn sign_digest(&self, digest: D) -> S {
        self.try_sign_digest(digest)
            .expect("signature operation failed")
    }

    /// Attempt to sign the given prehashed message `Digest`, returning a
    /// digital signature on success, or an error if something went wrong.
    fn try_sign_digest(&self, digest: D) -> Result<S, Error>;
}

/// Sign the given message using the provided external randomness source.
#[cfg(feature = "rand-preview")]
#[cfg_attr(docsrs, doc(cfg(feature = "rand-preview")))]
pub trait RandomizedSigner<R, S>
where
    R: CryptoRng + RngCore,
    S: Signature,
{
    /// Sign the given message and return a digital signature
    fn sign_with_rng(&self, rng: &mut R, msg: &[u8]) -> S {
        self.try_sign_with_rng(rng, msg)
            .expect("signature operation failed")
    }

    /// Attempt to sign the given message, returning a digital signature on
    /// success, or an error if something went wrong.
    ///
    /// The main intended use case for signing errors is when communicating
    /// with external signers, e.g. cloud KMS, HSMs, or other hardware tokens.
    fn try_sign_with_rng(&self, rng: &mut R, msg: &[u8]) -> Result<S, Error>;
}
