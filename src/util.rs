use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub fn extract_duplicates(iter: impl Iterator<Item=char>, length: usize) -> Vec<(usize, u64)>
{
    assert!(length <= 8);
    // yay bit magic
    let mask: u64 = (1 << (8 * length)) - 1;

    // this hashmap is a terrible choice for high performance ...
    let mut occurrences = HashMap::new();
    let mut duplicates = vec![];

    let mut buffer = 0u64;
    let mut iter = iter.enumerate();

    let push_buffer = |char, buffer: &mut u64|
    {
        *buffer <<= 8;
        *buffer |= char as u64
    };

    // initialize the buffer
    for _ in 0..(length - 1)
    {
        push_buffer(iter.next().unwrap().1, &mut buffer);
    }

    for (current, char) in iter
    {
        push_buffer(char, &mut buffer);

        let index = current - 2;
        match occurrences.entry(buffer & mask)
        {
            Entry::Occupied(mut entry) =>
                {
                    let old = entry.insert(index);
                    duplicates.push((index - old, *entry.key()));
                },
            Entry::Vacant(entry) =>
                {
                    entry.insert(index);
                },
        };
    }
    duplicates
}