use permutohedron::Heap;

pub fn decode_columnar_transposition(code: &str, inverse_key: &Vec<usize>) -> String
{
    let mut chars = code.chars();

    let num_columns = inverse_key.len();
    let num_rows = code.len() / inverse_key.len() + 1;

    let empty = num_columns * num_rows - code.len();
    let mut result = vec!['.'; code.len()];

    for key_column in inverse_key
    {
        let row_max = if *key_column < num_columns - empty {num_rows} else {num_rows - 1};

        for row_index in 0..row_max
        {
            let index = key_column + row_index*num_columns;
            result[index] = chars.next().unwrap();
        }
    }

    result.into_iter().collect()
}

pub fn brute_force_columnar_transposition(str: &str, filter: fn(String) -> usize)
{
    for length in 3..26
    {
        println!("checking for key length {}", length);

        let mut data: Vec<usize> = (0..length).collect();
        let heap = Heap::new(&mut data);

        let mut total_heuristic = 0;
        let mut max_heuristic = 0;
        let mut max_key = vec![];

        for key in heap
        {
            let decoded = decode_columnar_transposition(str, &key);
            let heuristic = filter(decoded);

            total_heuristic += heuristic;
            if heuristic > max_heuristic
            {
                max_heuristic = heuristic;
                max_key = key;
            }
        }

        println!("heuristic data for key length: {}", length);
        println!("  avg: {}", total_heuristic / (2..=length).fold(1, |acc, num| acc * num));
        println!("  max: {}", max_heuristic);
        println!("  key: {:?}", max_key);
    }
}