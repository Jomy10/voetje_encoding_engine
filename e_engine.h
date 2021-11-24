/*
This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at <http://mozilla.org/MPL/2.0/>.

# Voetje Encoding Engine<br/>
Open-source library.<br/>
Used in the app <a href="https://voetje.jonaseveraert.be">'t Voetje</a> (© KSA Noordzeegouw 2021) 
to encode Strings. <br/>
This library was created to share code between iOS (Swift) and Android (Java).

**Written by**: Jonas Everaeert<br/>
**Contributors**: <br/>
*/

#include <stdint.h>


/// A return type for C.
///
/// Contains a `return_code` which indicates any errors
/// and an `output` indicating the output of the method
struct C_Return {
  uint32_t return_code;
  char *output;
};

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
struct C_Return encode_jaar(const char *input, const char *jaar);

/// Reverses each word in a string.
///
/// ## Parameters
/// - input: the string to be encoded
///
/// ## Returns
/// - A string where all the words of the `input`have been reversed
///
/// ## Freeing memory
/// `omkeren_free`
char *encode_omkeren(const char *input);

/// Has to be called after `encode_jaar` to the free memory
///
/// Disregarding to do this will cause a **memory leak**.
///
/// Not applicable for Java.
void jaar_free(struct C_Return cret);

/// Has to be called after `encode_omkeren` to the free memory.<br/>
/// Disregarding to do this will cause a **memory leak**.
void omkeren_free(char *s);
