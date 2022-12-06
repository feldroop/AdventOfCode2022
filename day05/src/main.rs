use std::fs;

mod input;

fn main() {
    let input_string = fs::read_to_string("input/day5.txt").unwrap();
    let (stacks, moves) = input::parse(&input_string);

    let mut one_at_a_time_stacks = stacks.clone();
    for move_ in &moves {
        for _ in 0..move_.amount {
            let crate_ = one_at_a_time_stacks[move_.index_from]
                .pop()
                .expect("Trying to move a crate from an empty stack");
            one_at_a_time_stacks[move_.index_to].push(crate_);
        }
    }

    let final_top_crates: String = collect_final_top_crates(&one_at_a_time_stacks);
    println!("Final crates on the top of the stacks (moved one at a time): {final_top_crates}");

    let mut bulk_move_stacks = stacks;
    for move_ in &moves {
        let stack_from = &mut bulk_move_stacks[move_.index_from];
        let num_remaining = stack_from.len() - move_.amount;

        let crates = stack_from[num_remaining..].to_vec();
        stack_from.truncate(num_remaining);

        bulk_move_stacks[move_.index_to].extend_from_slice(&crates);
    }

    let final_top_crates: String = collect_final_top_crates(&bulk_move_stacks);
    println!("Final crates on the top of the stacks (bulk moved): {final_top_crates}");
}

fn collect_final_top_crates(stacks: &[input::Stack]) -> String {
    stacks
        .iter()
        .map(|stack| stack.last().expect("Empty stack in the end"))
        .collect()
}
