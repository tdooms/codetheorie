use std::collections::HashMap;
use scraper::{Html, Selector};

pub type Frequencies = Vec<f64>;
pub type Languages = HashMap<String, Frequencies>;

pub fn scrape_languages() -> Languages
{
    // This code was written on 15/10/2020 maybe the wikipedia page has changed since then and this won't work anymore
    let html = reqwest::blocking::get("https://en.wikipedia.org/wiki/Letter_frequency#Relative_frequencies_of_letters_in_other_languages").unwrap().text().unwrap();

    let mut frequencies = HashMap::new();
    let languages = ["English", "French", "German", "Spanish", "Portuguese", "Esperanto",
        "Italian", "Turkish", "Swedish", "Polish", "Dutch", "Danish", "Icelandic", "Finnish", "Czech"];

    let document = Html::parse_document(&html);
    let table_selector = Selector::parse("table.wikitable").unwrap();

    let table = document.select(&table_selector).skip(2).next().unwrap();

    for row in table.select(&Selector::parse("tr").unwrap()).skip(1)
    {
        let cell_selector = Selector::parse("td").unwrap();
        let mut iter = row.select(&cell_selector);

        let unicode_char = iter.next().unwrap().inner_html().chars().nth(3).unwrap();
        let ascii_char = unidecode::unidecode_char(unicode_char).chars().nth(0).unwrap();
        let char_index = ascii_char as usize - 'a' as usize;

        for (lang_index, cell) in iter.enumerate()
        {
            let percentage: f64 = cell.inner_html().trim_matches(|c| !char::is_numeric(c)).parse().unwrap();
            frequencies.entry(languages[lang_index].to_string()).or_insert(vec![0f64; 26])[char_index] += percentage
        }
    }

    frequencies
}

pub fn extract_alphabetic_frequencies(iter: impl Iterator<Item=char>) -> Frequencies
{
    let mut counts = vec![0;26];
    let mut sum = 0;

    for char in iter
    {
        assert!(char.is_alphabetic() && char.is_ascii_lowercase());
        let index = char as usize - 'a' as usize;
        counts[index] += 1;
        sum += 1;
    }

    counts.into_iter().map(|count| 100f64 * count as f64 / sum as f64).collect()
}

pub fn extract_alphanumeric_frequencies(iter: impl Iterator<Item=char>) -> Frequencies
{
    let mut counts = vec![0;36];
    let mut sum = 0;

    for char in iter
    {
        let index = if char.is_alphabetic()
        {
            char as usize - 'a' as usize
        }
        else if char.is_numeric()
        {
            (char as usize - '0' as usize) + 26
        }
        else
        {
            unreachable!()
        };

        counts[index] += 1;
        sum += 1;
    }

    counts.into_iter().map(|count| 100f64 * count as f64 / sum as f64).collect()
}

fn least_squares(frequencies: &Frequencies, target: &Frequencies) -> f64
{
    target.iter().zip(frequencies.iter()).fold(0f64, |acc, (a, b)| acc + (a-b)*(a-b))
}

pub fn best_language_fit(frequencies: &Frequencies, languages: &Languages) -> Vec<(String, f64)>
{
    languages.iter().map(|(name, target)| (name.clone(), least_squares(frequencies, target))).collect()
}

// I assume frequencies is sorted
pub fn best_unordered_language_fit(frequencies: &Frequencies, languages: &Languages) -> Vec<(String, f64)>
{
    let mut res = vec![];
    for (name, language) in languages
    {
        let mut temp = language.clone();
        temp.sort_by(|a, b| b.partial_cmp(a).unwrap());

        let score = least_squares(frequencies, &temp);
        res.push((name.clone(), score));
    }
    res
}

pub fn best_offset_and_language_fit(frequencies: &Frequencies, languages: &Languages) -> Vec<((String, char), f64)>
{
    let mut result = vec![];
    let mut rotated = frequencies.clone();

    for char in 'a'..='z'
    {
        let mut scores = best_language_fit(&rotated, languages);
        scores.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());

        let (name, score) = scores[0].clone();
        result.push(((name, char), score));

        rotated.rotate_right(1);
    }

    result
}