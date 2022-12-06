use hashbag::HashBag;
use std::fs;

fn main() {
    let input = fs::read_to_string("input/day6.txt").unwrap();

    assert!(input.is_ascii());
    let ascii_byte_input = input.into_bytes();

    let first_start_of_packet =
        index_after_first_n_consecutive_distict_chars(&ascii_byte_input, 4).unwrap();

    println!("Number of character processed before the first start-of-packet marker: {first_start_of_packet}");

    let first_start_of_message =
        index_after_first_n_consecutive_distict_chars(&ascii_byte_input, 14).unwrap();

    println!("Number of character processed before the first start-of-message marker: {first_start_of_message}");
}

// ultra unnecessarily efficient implementation of this function
fn index_after_first_n_consecutive_distict_chars(
    ascii_chars: &[u8],
    num_consecutive: usize,
) -> Result<usize, String> {
    let (first_window, after_first_window) = ascii_chars.split_at(num_consecutive);
    let mut current_chars: HashBag<_> = first_window.iter().collect();

    let window_border_iter = ascii_chars.iter().zip(after_first_window.iter());

    for (window_idx, (oldest_current_char, next_char)) in window_border_iter.enumerate() {
        if current_chars.set_len() == num_consecutive {
            // offset of num_consecutive - 1 for the incomplete windows at the beginning
            // plus 1 for 0-based index
            return Ok(window_idx + num_consecutive);
        }

        current_chars.remove(oldest_current_char);
        current_chars.insert(next_char);
    }

    Err(format!(
        "slice does not contain {num_consecutive} distinct chars"
    ))
}
