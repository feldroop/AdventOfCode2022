mod discovery;
mod filesystem_model;
mod input;

use std::fs;

use discovery::FilesystemDiscoverer;

fn main() {
    let input_string = fs::read_to_string("input/day7.txt").unwrap();
    let commands = input::parse(&input_string);

    // this assumes that the beginning of the user history is at root
    let mut filesystem_discoverer = FilesystemDiscoverer::start_at_root();

    for command in commands {
        filesystem_discoverer.apply_command(&command);
    }

    let tree = filesystem_discoverer.finish();

    // println!("{}", tree.format_to_string());

    let directory_sizes = tree.recursive_directory_sizes();

    const SMALL_DIRECTORY_SIZE: usize = 100_000;
    let sum_of_small_directory_sizes: usize = directory_sizes
        .values()
        .filter(|&&size| size <= SMALL_DIRECTORY_SIZE)
        .sum();

    println!("Sum of small directory sizes: {sum_of_small_directory_sizes}");

    const MAX_OCCUPIED_MEMORY: usize = 40_000_000;
    let root_size = directory_sizes[&0];
    let needed_memory_size = root_size.saturating_sub(MAX_OCCUPIED_MEMORY);

    let smallest_sufficient_directory_size = directory_sizes
        .values()
        .filter(|&&size| size >= needed_memory_size)
        .min()
        .unwrap();

    println!(
        "Smallest directory size that frees enough space: {smallest_sufficient_directory_size}"
    );
}

// count recursive directory sizes
