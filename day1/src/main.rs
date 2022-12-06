fn main() {
    let calories = std::fs::read_to_string("input/day1.txt").unwrap();

    let mut current_elf_calories = 0;
    // let mut max_elf_calories: u32 = 0;
    let mut max_elf_calories: [u32; 3] = [0, 0, 0];

    for line in calories.lines() {
        if line.is_empty() {
            if current_elf_calories > max_elf_calories[0] {
                max_elf_calories[0] = current_elf_calories;
                max_elf_calories.sort();
            }

            current_elf_calories = 0;
            continue;
        }

        let calorie_value: u32 = line.parse().unwrap();
        current_elf_calories += calorie_value;
    }

    let sum: u32 = max_elf_calories.into_iter().sum();
    println!("Max calories carried by an elf: {sum}");
}
