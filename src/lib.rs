/*
    Copyright © 2023, ParallelChain Lab 
    Licensed under the Apache License, Version 2.0: http://www.apache.org/licenses/LICENSE-2.0
*/

//! Implementation of the encode and decode operations (as defined in IETF RFC 4648) on a type [Base64URL].
//! Base64URL are Strings restricted to containing the 2^6 UTF-8 code points without padding.
//! It is the *only* binary-to-text encoding scheme used in ParallelChain F. 
//! 
//! In general, we use `-` and `_` instead of `+` and `/`, without paddings.

use base64::{Engine as _, engine::{self, general_purpose}, alphabet};
const CUSTOM_ENGINE: engine::GeneralPurpose = engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

/// encode takes in a slice of bytes and returns the bytes encoded as a Base64URL String. 
pub fn encode<T: AsRef<[u8]>>(bytes: T) -> String { 
    CUSTOM_ENGINE.encode(bytes)
}

/// decode takes in a string and tries to decode it into a Vector of bytes. It returns a base64::DecodeError if `string`
/// is not valid Base64URL.
pub fn decode<T: ?Sized + AsRef<[u8]>>(base64_url: &T) -> Result<Vec<u8>, base64::DecodeError> {
    CUSTOM_ENGINE.decode(base64_url)
} 

