/*
   Copyright 2020 Amado Wilkins

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

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

    //TODO: add option for alternate line separator
    let input_lines = input.split("\n");
    let mut input_count = 0;

    let mut unique_lines = Vec::<UniqueLine>::new();
    //TODO: separate this into a function
    for line in input_lines {
        if line == "" { continue; }

        let index_result = get_line_index(&unique_lines, String::from(line));

        if index_result.is_ok() {
            let index = index_result.ok().unwrap();
            unique_lines[index].count += 1;
        } else {
            unique_lines.push(
                UniqueLine {
                    pattern: String::from(line),
                    count: 1 });
        }

        input_count += 1;
    }
    
    for line in unique_lines {
        println!("{} ({:.2}%) {}",
            line.count,
            (line.count as f32 / input_count as f32 * 100.0),
            line.pattern);
    }

    println!("Total items: {}", input_count);
}
