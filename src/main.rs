use std::io;
use std::io::Read;

struct UniqueLine {
    pattern: String,
    count: u32
}

fn get_line_index(list: &Vec::<UniqueLine>, pattern: String) -> Result<usize, String> {
    let mut i: usize = 0;
    let mut result: Result<usize, String> = Err(String::from("This doesnt exist mate"));

    for line in list {
        if line.pattern == pattern {
            result = Ok(i);
            break;
        } else {
            i += 1;
        }
    }

    result
}


fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("oopsies poopsies");

    let mut unique_lines = Vec::<UniqueLine>::new();

    //TODO: separate this into a function
    //TODO: add option for alternate line separator
    for line in input.split("\n") {
        let index_result = get_line_index(&unique_lines, String::from(line));

        //TODO: ignore empty
        if index_result.is_ok() {
            let index = index_result.ok().unwrap();
            unique_lines[index].count += 1;
        } else {
            unique_lines.push(
                UniqueLine {
                    pattern: String::from(line),
                    count: 1 });
        }
    }
    
    for line in unique_lines {
        println!("{} {}", line.count, line.pattern);
    }
}
