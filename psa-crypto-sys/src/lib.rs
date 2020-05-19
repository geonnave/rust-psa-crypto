// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! # PSA Cryptography API Wrapper
//!
//! This crate provides abstraction over an implementation of the PSA Cryptography API.
//! You can find the API
//! [here](https://developer.arm.com/architectures/security-architectures/platform-security-architecture/documentation).

// This one is hard to avoid.
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::missing_safety_doc)]
// Respect the C API case
#![allow(non_snake_case)]

#[allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    trivial_casts
)]
#[allow(clippy::all)]
mod psa_crypto_binding {
    include!(concat!(env!("OUT_DIR"), "/shim_bindings.rs"));
}

#[allow(dead_code)]
mod constants;
pub use constants::*;
pub use psa_crypto_binding::*;

pub unsafe fn psa_get_key_bits(attributes: *const psa_key_attributes_t) -> usize {
    shim_get_key_bits(attributes)
}

pub unsafe fn psa_get_key_type(attributes: *const psa_key_attributes_t) -> psa_key_type_t {
    shim_get_key_type(attributes)
}

pub unsafe fn psa_key_attributes_init() -> psa_key_attributes_t {
    shim_key_attributes_init()
}

pub unsafe fn psa_set_key_algorithm(attributes: *mut psa_key_attributes_t, alg: psa_algorithm_t) {
    shim_set_key_algorithm(attributes, alg);
}

pub unsafe fn psa_set_key_bits(attributes: *mut psa_key_attributes_t, bits: usize) {
    shim_set_key_bits(attributes, bits);
}

pub unsafe fn psa_set_key_id(attributes: *mut psa_key_attributes_t, id: psa_key_id_t) {
    shim_set_key_id(attributes, id);
}

pub unsafe fn psa_set_key_lifetime(
    attributes: *mut psa_key_attributes_t,
    lifetime: psa_key_lifetime_t,
) {
    shim_set_key_lifetime(attributes, lifetime);
}

pub unsafe fn psa_set_key_type(attributes: *mut psa_key_attributes_t, type_: psa_key_type_t) {
    shim_set_key_type(attributes, type_);
}

pub unsafe fn psa_set_key_usage_flags(
    attributes: *mut psa_key_attributes_t,
    usage_flags: psa_key_usage_t,
) {
    shim_set_key_usage_flags(attributes, usage_flags);
}

pub fn PSA_ALG_IS_HASH(alg: psa_algorithm_t) -> bool {
    unsafe { shim_PSA_ALG_IS_HASH(alg) == 1 }
}

pub fn PSA_ALG_IS_MAC(alg: psa_algorithm_t) -> bool {
    unsafe { shim_PSA_ALG_IS_MAC(alg) == 1 }
}

pub fn PSA_ALG_IS_CIPHER(alg: psa_algorithm_t) -> bool {
    unsafe { shim_PSA_ALG_IS_CIPHER(alg) == 1 }
}

pub fn PSA_ALG_IS_AEAD(alg: psa_algorithm_t) -> bool {
    unsafe { shim_PSA_ALG_IS_AEAD(alg) == 1 }
}

pub fn PSA_ALG_IS_SIGN(alg: psa_algorithm_t) -> bool {
    unsafe { shim_PSA_ALG_IS_SIGN(alg) == 1 }
}

pub fn PSA_ALG_IS_ASYMMETRIC_ENCRYPTION(alg: psa_algorithm_t) -> bool {
    unsafe { shim_PSA_ALG_IS_ASYMMETRIC_ENCRYPTION(alg) == 1 }
}

pub fn PSA_ALG_IS_KEY_AGREEMENT(alg: psa_algorithm_t) -> bool {
    unsafe { shim_PSA_ALG_IS_KEY_AGREEMENT(alg) == 1 }
}

pub fn PSA_ALG_IS_KEY_DERIVATION(alg: psa_algorithm_t) -> bool {
    unsafe { shim_PSA_ALG_IS_KEY_DERIVATION(alg) == 1 }
}

pub fn PSA_ALG_IS_RSA_PKCS1V15_SIGN(alg: psa_algorithm_t) -> bool {
    unsafe { shim_PSA_ALG_IS_RSA_PKCS1V15_SIGN(alg) == 1 }
}

pub fn PSA_ALG_IS_RSA_PSS(alg: psa_algorithm_t) -> bool {
    unsafe { shim_PSA_ALG_IS_RSA_PSS(alg) == 1 }
}

pub fn PSA_ALG_IS_ECDSA(alg: psa_algorithm_t) -> bool {
    unsafe { shim_PSA_ALG_IS_ECDSA(alg) == 1 }
}

pub fn PSA_ALG_IS_DETERMINISTIC_ECDSA(alg: psa_algorithm_t) -> bool {
    unsafe { shim_PSA_ALG_IS_DETERMINISTIC_ECDSA(alg) == 1 }
}

pub fn PSA_ALG_SIGN_GET_HASH(alg: psa_algorithm_t) -> psa_algorithm_t {
    unsafe { shim_PSA_ALG_SIGN_GET_HASH(alg) }
}

pub fn PSA_ALG_RSA_PKCS1V15_SIGN(hash_alg: psa_algorithm_t) -> psa_algorithm_t {
    unsafe { shim_PSA_ALG_RSA_PKCS1V15_SIGN(hash_alg) }
}

pub fn PSA_ALG_RSA_PSS(hash_alg: psa_algorithm_t) -> psa_algorithm_t {
    unsafe { shim_PSA_ALG_RSA_PSS(hash_alg) }
}

pub fn PSA_ALG_ECDSA(hash_alg: psa_algorithm_t) -> psa_algorithm_t {
    unsafe { shim_PSA_ALG_ECDSA(hash_alg) }
}

pub fn PSA_ALG_DETERMINISTIC_ECDSA(hash_alg: psa_algorithm_t) -> psa_algorithm_t {
    unsafe { shim_PSA_ALG_DETERMINISTIC_ECDSA(hash_alg) }
}
