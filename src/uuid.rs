use core::str;
use std::{
    fmt::{Display, Formatter, Result},
    marker::PhantomData,
};

use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha20Rng,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Raw;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Prefixed;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Uuid<Type = Raw> {
    inner: String,
    prefix: Option<String>,
    variant: PhantomData<Type>,
}

const CHAR_SET: &[char] = &[
    // Numbers
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', //
    // Losercase
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', //
    'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', //
    'u', 'v', 'w', 'x', 'y', 'z', //
    // Uppercase
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', //
    'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', //
    'U', 'V', 'W', 'X', 'Y', 'Z', //
];

impl Uuid {
    /// Creates a new prefixed UUID identifier.
    pub fn prefixed(prefix: impl Into<String>) -> Uuid<Prefixed> {
        let identifier = Self::generate();

        // prfx:12345-abcdf-GHIJK-lMn0p

        Uuid {
            inner: identifier,
            prefix: Some(prefix.into()),
            variant: PhantomData,
        }
    }

    /// Creates a new UUID identifier.
    pub fn raw() -> Uuid<Raw> {
        Uuid {
            inner: Self::generate(),
            prefix: None,
            variant: PhantomData,
        }
    }
}

impl Uuid<Prefixed> {
    /// Gets the prefix of the UUID.
    pub fn get_prefix(&self) -> &str {
        // Should never be None.
        self.prefix.as_deref().unwrap()
    }

    pub fn without_prefix(&self) -> Uuid<Raw> {
        Uuid {
            inner: self.inner.clone(),
            prefix: None,
            variant: PhantomData,
        }
    }
}

impl Uuid<Raw> {
    pub fn with_prefix(self, prefix: impl Into<String>) -> Uuid<Prefixed> {
        Uuid {
            inner: self.inner,
            prefix: Some(prefix.into()),
            variant: PhantomData,
        }
    }
}

impl<Type> Uuid<Type> {
    /// Generates a new UUID identifier.
    /// This is a character array of the length 23.
    ///
    /// All characters are randomly chosen from the following character set:
    /// 0-9, a-z, A-Z
    ///
    /// The following structure is generated:
    /// xxxxx-xxxxx-xxxxx-xxxxx
    pub fn generate() -> String {
        let mut rng = ChaCha20Rng::from_entropy();

        // Creates a buffer on the stack for the random bytes
        let mut buffer = [0u8; 23];

        // Fill the buffer with random bytes
        rng.fill_bytes(&mut buffer);

        // Create the identifier buffer on the stack
        let mut identifier: [char; 23] = ['0'; 23];

        // Iterate through all the random bytes
        for (i, b) in buffer.iter().enumerate() {
            // Map the random bytes to a character in the CHAR_SET
            identifier[i] = CHAR_SET[*b as usize % CHAR_SET.len()];
        }

        // Forcefully insert '-' in the correct positions
        identifier[5] = '-';
        identifier[11] = '-';
        identifier[17] = '-';

        identifier.iter().collect::<String>()
    }

    /// Gets the identifier of the UUID.
    pub fn get_identifier(&self) -> &str {
        &self.inner
    }
}

impl Default for Uuid<Raw> {
    fn default() -> Self {
        Self::raw()
    }
}

impl<Type> From<Uuid<Type>> for String {
    fn from(val: Uuid<Type>) -> Self {
        val.to_string()
    }
}

impl<Type> Display for Uuid<Type> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.prefix {
            Some(prefix) => write!(f, "{}:{}", prefix, self.inner),
            None => write!(f, "{}", self.inner),
        }
    }
}
