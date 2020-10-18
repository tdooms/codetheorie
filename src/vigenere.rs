use crate::frequency_analysis::{scrape_languages, best_offset_and_language_fit, extract_alphabetic_frequencies};

pub fn analyze_divisors_from_duplicates(duplicates: &Vec<(usize, u32)>) -> Vec<u32>
{
    const SIZE: usize = 20;
    let mut div_count = vec![0u32; SIZE];

    for (offset, _) in duplicates
    {
        let sqrt = (*offset as f64).sqrt() as usize;
        for div in 3..sqrt.min(SIZE)
        {
            if *offset % div == 0
            {
                div_count[div] += 1;
            }
        }
    }

    div_count
}

pub fn invert_key(key: Vec<usize>) -> Vec<usize>
{
    let mut inverse: Vec<_> = key.into_iter().enumerate().collect();
    inverse.sort_by_key(|(_, k)| *k);
    inverse.into_iter().map(|(a, _)| a as usize).collect()
}

pub fn print_duplicates(duplicates: &Vec<(usize, u32)>)
{
    let printable: Vec<_> = duplicates.iter().map(|(offset, buffer)|
        {
            let a = (buffer & 0xFF000000) >> 24;
            let b = (buffer & 0x00FF0000) >> 16;
            let c = (buffer & 0x0000FF00) >> 8;
            let d = (buffer & 0x000000FF) >> 0;
            (offset, String::from_utf8(vec![a as u8, b as u8, c as u8, d as u8]).unwrap())
        }).collect();
    println!("{:?}", printable);
}


pub fn guess_vigenere_key(str: &str, key_length: usize) -> Vec<char>
{
    let languages = scrape_languages();
    let mut key = vec![];

    for start in 0..key_length
    {
        let iter = str.chars().skip(start).step_by(key_length);
        let frequencies = extract_alphabetic_frequencies(iter);

        let mut scores = best_offset_and_language_fit(&frequencies, &languages);
        scores.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());

        // uncomment the next line to see the least squares scores for the key
        println!("{:?}", scores);

        let (_, char) = scores[0].0;
        key.push(char);
    }

    key
}

pub fn decode_vigenere_with_key(str: &str, key: &Vec<char>) -> String
{
    str.chars().enumerate().map(|(index, char)|
        {
            let ascii_number = ((char as u32 - 'a' as u32) + (key[index % key.len()] as u32 - 'a' as u32)) % 26 + 'a' as u32;
            std::char::from_u32(ascii_number).unwrap()
        }).collect()
}

