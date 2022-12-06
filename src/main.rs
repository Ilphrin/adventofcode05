use std::env;
use std::fs;

fn separator(lines: &Vec<&str>) -> Vec<usize> {
    let mut start = 0;
    let mut columns = 0;
    for (index, line) in lines.iter().enumerate() {
        if line.starts_with(" 1") {
            start = index;
            columns = line.split("   ").collect::<Vec<&str>>().len();
            break;
        }
    }
    let mut output = Vec::new();
    output.push(start);
    output.push(columns);
    return output;
}

fn construct_stacks(lines: &Vec<&str>, start: usize, columns: usize) -> Vec<Vec<char>> {
    println!("Number of columns: {columns}");
    let mut stacks: Vec<Vec<char>> = Vec::new();

    for _i in (0..columns) {
        stacks.push(Vec::new());
    }

    for i in (0..start).rev() {
        for j in (0..columns) {
            let letter = lines[i]
                .chars()
                .nth(j * 4 + 1)
                .expect("To ba able to get a char");

            if letter != ' ' {
                stacks[j].push(letter);
            }
        }
    }
    return stacks;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");
    let splitted_content = contents.split("\n").collect::<Vec<&str>>();

    let start = separator(&splitted_content)[0];
    let columns = separator(&splitted_content)[1];
    let mut stacks = construct_stacks(&splitted_content, start, columns);

    for line in (start + 2)..splitted_content.len() {
        let order = splitted_content[line].split(" ").collect::<Vec<&str>>();
        if order.len() == 6 {
            let destination: usize = order[5].parse().unwrap();
            let from: usize = order[3].parse().unwrap();
            if args.len() == 2 && args[1] == "2" {
                let mut intermediary_vec: Vec<char> = Vec::new();
                for _remaining_to_remove in 0..(order[1].parse().unwrap()) {
                    let removed = stacks[from - 1].pop().unwrap();
                    intermediary_vec.push(removed);
                }
                for _i in 0..(intermediary_vec.len()) {
                    let removed = intermediary_vec.pop().unwrap();
                    stacks[destination - 1].push(removed);
                }
            } else {
                for _remaining_to_remove in 0..(order[1].parse().unwrap()) {
                    let removed = stacks[from - 1].pop().unwrap();
                    stacks[destination - 1].push(removed);
                }
            }
        }
    }

    for i in stacks {
        print!("{:?}", i[i.len() - 1]);
    }
}
