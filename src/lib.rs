#![warn(missing_docs)]
//! This Source Code Form is subject to the terms of the Mozilla Public
//! License, v. 2.0. If a copy of the MPL was not distributed with this
//! file, You can obtain one at <http://mozilla.org/MPL/2.0/>.
//! 
//! # Voetje Encoding Engine<br/>
//! Open-source library.<br/>
//! Used in the app <a href="https://voetje.jonaseveraert.be">'t Voetje</a> (© KSA Noordzeegouw 2021) 
//! to encode Strings. <br/>
//! This library was created to share code between iOS (Swift) and Android (Java).
//! 
//! Written by: Jonas Everaeert<br/>
//! Contributors: <br/>

use std::os::raw::{c_char};
use std::ffi::*;
// use libc::{c_int};
mod encoding_funcs; // Import the encoding functions

#[no_mangle]
/// Encodes a string to jaartal
/// 
/// ## Parameters
/// - input: the string to be encoded
/// - jaar: the jaar it has to be encoded with
/// 
/// ## Returns
/// - A C_Return struct containing:
///     - c_int: the return code (0 if no errors occur, 
/// other variables if there was an error)
///     - c_char: the encoded string
/// 
/// ## Errors
/// Error codes:<br/>
/// 1: Couldn't convert CString to Rust str
/// 
/// ## Freeing memory
/// For Swift, it is important to call `jaar_free` after this function is called to free memory.
/// Disregarding to do this will cause a memory leak.
pub extern "C" fn encode_jaar(input: *const c_char, jaar: *const c_char) -> C_Return {
    // Convert CStrings to Rust str
    let c_str = unsafe { CStr::from_ptr(input) };
    let input = match c_str.to_str() {
        Ok(str) => str,
        Err(err) => {
            println!("An error occured: {}", err);
            return C_Return { return_code: 1, output: CString::new("Couldn't convert the CString to Rust str.").unwrap().into_raw() };
        }
    };

    let c_jaar = unsafe { CStr::from_ptr(jaar) };
    let jaar = match c_jaar.to_str() {
        Ok(str) => str,
        Err(err) => {
            println!("An error occured: {}", err);
            return C_Return { return_code: 1, output: CString::new("Couldn't convert the CString to Rust str for jaar.").unwrap().into_raw() };
        }
    };

    let output = encoding_funcs::encode_jaar_uni(input, jaar);

    // Return result
    return C_Return {
        return_code: output.0.into(),
        output: CString::new(output.1).unwrap().into_raw()
    } 
}

#[cfg(target_os="ios")]
#[no_mangle]
/// Has to be called after `encode_jaar` to the free memory
/// 
/// Disregarding to do this will cause a memory leak.C_Return
/// 
/// Not applicable for Java.
pub extern "C" fn jaar_free(cret: C_Return) {
    unsafe {
        if cret.output.is_null() { return }
        CString::from_raw(cret.output)
    };
}

#[repr(C)]
/// A return type for C
/// Contains a `return_code` which indicates any errors
/// and an `output` indicating the output of the method
pub struct C_Return {
    return_code: u32,
    output: *mut c_char
}

#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;
    
    use super::*;
    use self::jni::JNIEnv;
    use self::jni::objects::{JClass, JString};
    use self::jni::sys::{jstring};

    #[no_mangle]
    /// Encodeer jaar for Java (Android)
    /// 
    /// # Arguments
    /// - java_in `JString`: the input in Java
    /// - java_jaar `JString`: the inputted jaartal in Java
    /// - JNIEnv: object we will use to read values associated with the pointers that we are taking as argument
    /// - JClass: Class reference
    pub unsafe extern fn Java_be_ksa_voetje_methods_encoderen_EncodingEngine_java_1encodeer_1jaar(env: JNIEnv, _: JClass, java_in: JString, java_jaar: JString) -> jstring {
        let out = encode_jaar(env.get_string(java_in).expect("Invalid pattern string.").as_ptr(), env.get_string(java_jaar).expect("Invalid pattern string.").as_ptr());
        let out_ptr: CString = CString::from_raw(out.output);
        if out.return_code != 0 {
            println!("Error!");
            env.throw_new("java/lang/Exception", "Unkown Exception for now...");
        } 
        let output: JString = env.new_string(out_ptr.to_str().unwrap()).expect("Couldn't create java String.");

        output.into_inner()
    }
}