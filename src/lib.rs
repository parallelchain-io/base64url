/*
    Copyright © 2023, ParallelChain Lab 
    Licensed under the Apache License, Version 2.0: http://www.apache.org/licenses/LICENSE-2.0
*/

//! Implementation of the encode and decode operations on base64url (as defined in IETF RFC 4648) String.
//! The encoded string restricted to containing the 2^6 UTF-8 code points without padding.
//! 
//! In general, we use `-` and `_` instead of `+` and `/`, without paddings.

use base64::{Engine as _, engine::{self, general_purpose}, alphabet};
const ENGINE: engine::GeneralPurpose = engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

/// encode takes in a slice of bytes and returns the bytes encoded as a base64url String. 
pub fn encode<T: AsRef<[u8]>>(bytes: T) -> String { 
    ENGINE.encode(bytes)
}

/// decode takes in a string and tries to decode it into a Vector of bytes. It returns a base64::DecodeError if `string`
/// is not valid Base64URL.
pub fn decode<T: ?Sized + AsRef<[u8]>>(base64_url: &T) -> Result<Vec<u8>, base64::DecodeError> {
    ENGINE.decode(base64_url)
} 

