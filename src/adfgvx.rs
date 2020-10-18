pub fn convert_pair_to_char((first, second) : (char, char)) -> char
{
    let indexer = |char|match char
    {
        'A' => 0u32,
        'D' => 1,
        'F' => 2,
        'G' => 3,
        'V' => 4,
        'X' => 5,
        _ => unreachable!("problem with input in ADFGVX")
    };

    let num = indexer(first) + 6 * indexer(second);
    if num < 26 { std::char::from_u32('a' as u32 + num) } else { std::char::from_u32('0' as u32 + (num - 26))}.unwrap()
}