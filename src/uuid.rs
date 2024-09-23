use core::str;
use std::fmt::Display;

use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha20Rng,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Uuid(String);

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
    pub fn prefixed(prefix: impl Into<String>) -> Self {
        let identifier = Self::generate();

        // prfx:12345-abcdf-GHIJK-lMn0p

        let uuid = prefix.into() + ":" + &identifier;
        Self(uuid)
    }

    /// Creates a new UUID identifier.
    pub fn new() -> Self {
        Self(Self::generate())
    }

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

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn take(self) -> String {
        self.0
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl From<Uuid> for String {
    fn from(val: Uuid) -> Self {
        val.0
    }
}

impl Display for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
