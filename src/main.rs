#![allow(dead_code)]

mod frequency_analysis;
mod columnar_transposition;
mod util;
mod vigenere;
mod adfgvx;

use crate::columnar_transposition::{decode_columnar_transposition, brute_force_columnar_transposition};
use crate::vigenere::{guess_vigenere_key, decode_vigenere_with_key};
use crate::util::extract_duplicates;
use crate::adfgvx::convert_pair_to_char;
use crate::frequency_analysis::{scrape_languages, extract_alphanumeric_frequencies, best_unordered_language_fit, extract_alphabetic_frequencies, best_language_fit};


fn vigenere_and_column_decrypt()
{
    let str = std::fs::read_to_string("input/vigenere_column.txt").unwrap();

    // use this to guess the key, needs manual intervention
    brute_force_columnar_transposition(&str, |str| extract_duplicates(str.chars(), 3).len());

    // brute force gives a key of length 7
    let key: Vec<usize> = vec![1, 2, 3, 6, 0, 5, 4];
    println!("guessed columnar transposition key: {:?}", key);

    let partly_decoded = decode_columnar_transposition(&str, &key);
    println!("{}", partly_decoded);

    // use this to guess the vigenere key length, manual intervention is needed
    // let duplicates = vigenere_duplicates(&partly_decoded);
    // let divisor_count = get_divisors_from_duplicates(&duplicates);
    // println!("{:?}", divisor_count);

    // we could quite clearly see the guessed length was 9
    let key = guess_vigenere_key(&partly_decoded, 9);
    println!("guessed vigenere key: {:?}", key);

    let decoded = decode_vigenere_with_key(&partly_decoded, &key);
    println!("{}", decoded);
}

fn adfgvx_decrypt()
{
    let str = std::fs::read_to_string("input/adfgvx.txt").unwrap();

    // same as vigenere but we use length 6, because of the double characters
    // brute_force_columnar_transposition(&str, |str|
    //     {
    //         let iter = str.chars().step_by(2).zip(str.chars().skip(1).step_by(2)).map(convert_pair_to_char);
    //         extract_duplicates(iter, 3).len()
    //     });

    let key: Vec<usize> = vec![9, 6, 1, 7, 4, 8, 5, 0, 3, 2];
    let partly_decoded = decode_columnar_transposition(&str, &key);

    let by_pairs: String = partly_decoded.chars().step_by(2)
        .zip(partly_decoded.chars().skip(1).step_by(2))
        .map(convert_pair_to_char).collect();

    println!("{}", by_pairs);

    let mut frequencies = extract_alphanumeric_frequencies(by_pairs.chars());

    let table: Vec<_> = frequencies.iter().enumerate().map(|(index, f)| (std::char::from_u32('a' as u32 + index as u32).unwrap(), f)).collect();
    println!("{:?}", table);

    frequencies.sort_by(|a, b| b.partial_cmp(a).unwrap());
    frequencies.resize(26, f64::NAN);

    println!("{:?}", frequencies);

    let language = scrape_languages();
    let mut scores = best_unordered_language_fit(&frequencies, &language);
    scores.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());

    println!("{:?}", scores)
}

fn main()
{
    adfgvx_decrypt()
}
