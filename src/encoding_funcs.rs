///////////////////////////////////////////////////////////////////////////////////
/// This file contains the rust functions for the library. These are use inside ///
/// of the C and Java functions, so they share a common functionality. The      ///
/// Functions for Java and C then convert the return values of these functions  ///
/// to values they can read.                                                    ///
/// `uni` in the function names stand for universal, indicating that these      ///
/// are to be used by the different sub functions for the different platforms   ///
///                                                                             ///
/// Author: Jonas Everaert                                                      ///
///////////////////////////////////////////////////////////////////////////////////

use regex::*;

/// # Encode Jaar (Universal)
/// 
/// Encodes a string `input` using the code `jaar`.
/// 
/// ## Returns
/// - Return code `u8`: 0 if no errors occured
/// - Encoded `String`
/// 
/// ## Error codes
/// 1: Jaar is not 4 chars long.
pub fn encode_jaar_uni(input: &str, jaar: &str) -> (u8, String) {
    let mut jaar_int: [u8; 4] = [0; 4];

    if jaar.chars().count() == 4 {
        let jaar_char: [char; 4] = [
            jaar.chars().take(1).last().unwrap(), 
            jaar.chars().take(2).last().unwrap(), 
            jaar.chars().take(3).last().unwrap(), 
            jaar.chars().take(4).last().unwrap()
        ];

        for i in 0..4 {
            jaar_int[i] = jaar_char[i] as u8 - 0x30;
        }

        // Split input into sentences
        let regex = Regex::new("[!.?]").unwrap();
        let sentences: Vec<&str> = regex.split(input).collect();

        // dbg!(&sentences);

        let mut f = 0;

        // Remove empty elements of the sentences vector
        let mut adjusted_sentences: Vec<&str> = sentences.clone();
        for s in sentences {
            if s.chars().count() == 0 {
                adjusted_sentences.remove(f);
            }
            f = f + 1;
        }

        let mut output: String = String::new();

        for sentence in adjusted_sentences {            
            let mut jaar1: Vec<String> = vec![];
            let mut jaar2: Vec<String> = vec![];
            let mut jaar3: Vec<String> = vec![];
            let mut jaar4: Vec<String> = vec![];

            // Convert sentence
            let non_chars_regex = Regex::new(r"[^A-Za-z0-9\p{latin}]").unwrap();
            // Replace all non-chars with "" (all characters that are not A-Z, a-z or 0-9)
            let sentence = non_chars_regex.replace_all(sentence, "");
            // Trim sentence and to uppercase
            let sentence = sentence.trim().to_uppercase();

            // Parse the sentence
            let mut i = 0;

            loop {
                if sentence[i..].chars().count() == 0 {
                    break;
                }
                // Eerst jaar1
                if sentence[i..].chars().count() > jaar_int[0].into() {
                    let _j: usize = jaar_int[0].into();
                    jaar1.push( sentence[i..(i+_j)].to_owned() );
                    i = i + _j;
                } else {
                    // Less characters left than fit the jaartal jaar1 (= jaar_int[0])
                    let lenght_left = sentence[i..].chars().count();
                    let x_to_add = Into::<usize>::into(jaar_int[0]) - lenght_left;

                    let mut s = String::from(&sentence[i..]);
                    s.push_str(&"X".repeat(x_to_add));

                    jaar1.push(
                        s
                    );
                    // No more words in this sentence.
                    break; 
                }
                // Dan jaar2
                if sentence[i..].chars().count() > jaar_int[1].into() {
                    jaar2.push( sentence[i..(i+Into::<usize>::into(jaar_int[1]))].to_owned() );
                    i = i + Into::<usize>::into(jaar_int[1]);
                } else {
                    let lenght_left = sentence[i..].chars().count();
                    let x_to_add = Into::<usize>::into(jaar_int[1]) - lenght_left;

                    let mut s = String::from(&sentence[i..]);
                    s.push_str(&"X".repeat(x_to_add));

                    jaar2.push(
                        s
                    );
                    // No more words in this sentence.
                    break; 
                }
                if sentence[i..].chars().count() > jaar_int[2].into() {
                    jaar3.push( sentence[i..(i+Into::<usize>::into(jaar_int[2]))].to_owned() );
                    i = i + Into::<usize>::into(jaar_int[2]);
                } else {
                    let lenght_left = sentence[i..].chars().count();
                    let x_to_add = Into::<usize>::into(jaar_int[2]) - lenght_left;

                    let mut s = String::from(&sentence[i..]);
                    s.push_str(&"X".repeat(x_to_add));

                    jaar3.push(
                        s
                    );
                    // No more words in this sentence.
                    break; 
                }
                if sentence[i..].chars().count() > jaar_int[3].into() {
                    jaar4.push( sentence[i..(i+Into::<usize>::into(jaar_int[3]))].to_owned() );
                    i = i + Into::<usize>::into(jaar_int[3]);
                } else {
                    let lenght_left = sentence[i..].chars().count();
                    let x_to_add = Into::<usize>::into(jaar_int[3]) - lenght_left;

                    // String to add
                    let mut s = String::from(&sentence[i..]);
                    s.push_str(&"X".repeat(x_to_add));

                    jaar4.push(
                        s.to_owned()
                    );
                    // No more words in this sentence.
                    break; 
                }
            } // Einde loop
            
            for j in 0..std::cmp::max(std::cmp::max(jaar_int[0], jaar_int[1]), std::cmp::max(jaar_int[2], jaar_int[3])) {
                for k in 0..std::cmp::max (
                    std::cmp::max(jaar1.len(), jaar2.len()),
                    std::cmp::max(jaar3.len(), jaar4.len())
                ) {
                    if k < jaar1.len() {
                        if Into::<usize>::into(j) < jaar1[k].chars().count() {
                            output = format!("{}{}", output, &jaar1[k].chars().nth(Into::<usize>::into(j)).unwrap());
                        }
                    }
                    if k < jaar2.len() {
                        if Into::<usize>::into(j) < jaar2[k].chars().count() {
                            output = format!("{}{}", output, &jaar2[k].chars().nth(Into::<usize>::into(j)).unwrap());
                        }
                    }
                    if k < jaar3.len() {
                        if Into::<usize>::into(j) < jaar3[k].chars().count() {
                            output = format!("{}{}", output, &jaar3[k].chars().nth(Into::<usize>::into(j)).unwrap());
                        }
                    }
                    if k < jaar4.len() {
                        if Into::<usize>::into(j) < jaar4[k].chars().count() {
                            output = format!("{}{}", output, &jaar4[k].chars().nth(Into::<usize>::into(j)).unwrap());
                        }
                    }
                } // Einde for k
                // Add space to output
                output = format!("{} ", output);
            } // Einde for j
            output = format!("{}/ ", output);
        } // Einde zin

        return (0, String::from(output));
    } else {
        println!("Jaar is not 4 chars long!");
        return (1, String::from("Jaar is not 4 chars long!"));
    }
}

/// # Encode Woord Omkeren (Universal)
/// 
/// Reverses every word in a `str`, while preserving punctuation.
pub fn encode_omkeren_uni(input: &str) -> String {
    let words: Vec<&str> = input.split(' ').collect();

    let mut output = String::new();

    for word in words {
        dbg!(&word);
        let mut reversed_word = String::new();

        let word_length = word.chars().count();
        let mut leesteken = String::new();

        for i in 1..=word_length {
            // c = char
            let c = word.chars().nth(word_length - i).unwrap();
            let allowed_chars = Regex::new(r"[0-9\p{latin}]").unwrap();
            let leestekens_regex = Regex::new(r"[.,!?]").unwrap();
            // Reverse if character is part of the latin alfabet, or a number
            if allowed_chars.is_match(&String::from(c)) {
                reversed_word = reversed_word + &String::from(c);
            } else if leestekens_regex.is_match(&String::from(c)) {
                // Leesteken na het woord toevoegen
                leesteken = String::from(c);
            } else {
                // TODO: andere niet-tekst characters 
            }
        }

        // Add reversed word to output
        if output.is_empty() {
            output = format!("{}{}", reversed_word, leesteken);
        } else {
            output = format!("{} {}{}", output, reversed_word, leesteken);
        }
    }

    return String::from(output);
}

