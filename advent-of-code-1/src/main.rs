fn main() {
    let input = std::fs::read_to_string("src/input-2.txt").unwrap();
    let elves = counting_calories(input); 
    day1_part1(&elves);
    day1_part2(&elves);
}

fn day1_part1(elves: &[u32]) {
    let max = elves.iter().max().unwrap();
    println!("{max}");
}

fn day1_part2(elves: &[u32]) {
    let mut elves_copy = elves.to_vec();
    elves_copy.sort_unstable();
    elves_copy.reverse();
    println!("{}", elves_copy.iter().take(3).sum::<u32>());
}


fn counting_calories(input: String) -> Vec<u32> {
    let elves: Vec<u32> = input
    .split("\n\n")
    .map(|elf| elf.lines().map(|calories| calories.parse::<u32>().unwrap()).sum::<u32>())
    .collect::<Vec<u32>>();
    elves
}