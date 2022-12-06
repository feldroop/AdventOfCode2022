use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input/day6.txt").unwrap();

    assert!(input.is_ascii());
    let ascii_byte_input = input.into_bytes();

    let first_start_of_packet =
        get_index_after_first_n_distinct_consecutive_chars(&ascii_byte_input, 4);

    println!("Number of character processed before the first start-of-packet marker: {first_start_of_packet}");

    let first_start_of_message =
        get_index_after_first_n_distinct_consecutive_chars(&ascii_byte_input, 14);

    println!("Number of character processed before the first start-of-message marker: {first_start_of_message}");
}

fn get_index_after_first_n_distinct_consecutive_chars(
    ascii_chars: &[u8],
    num_consecutive: usize,
) -> usize {
    ascii_chars
        .windows(num_consecutive)
        .enumerate()
        .find_map(|(index, chars)| {
            let char_set: HashSet<_> = chars.iter().collect();

            if char_set.len() == num_consecutive {
                // offset of num_consecutive - 1 for the incomplete windows at the beginning
                // plus 1 for 0-based index
                Some(index + num_consecutive)
            } else {
                None
            }
        })
        .expect("The input string should contain a start-of-packet marker")
}
