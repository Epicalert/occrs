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
use std::io::BufReader;
use std::io::BufRead;

use std::fs::File;

use std::env;

//a unique pattern in the input
struct UniqueLine {
    pattern: String,
    count: u32  //TODO: change to usize
}

//summary of the unique patterns
struct Summary {
    unique: Vec::<UniqueLine>,
    total_count: usize
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

fn line_iteration(unique_lines: &mut Vec::<UniqueLine>, line: String, input_count: &mut usize ) {
    if line == "" { return; }

    let index_result = get_line_index(unique_lines, line.clone());

    if index_result.is_ok() {
        let index = index_result.ok().unwrap();
        unique_lines[index].count += 1;
    } else {
        unique_lines.push(
            UniqueLine {
                pattern: line,
                count: 1 });
    }

    *input_count += 1;
}

//generates a Summary struct of the information
fn count_occurances() -> Summary {
    let args: Vec<String> = env::args().collect();
    let mut input_count = 0;
    let mut unique_lines = Vec::<UniqueLine>::new();

    if args.len() > 1 {
        //load from a file
        let input_file = File::open(args[1].clone())
            .expect(
                format!("Could open file {}", args[1].clone())
                    .as_str());
        let input_bufreader = BufReader::new(input_file);
        //TODO: add option for alternate line separator
        let input_lines = input_bufreader.split(b'\n');

        for line in input_lines {
            let line = String::from_utf8(
                line.expect("Something went wrong while reading the file.")
                ).expect("This file is not valid UTF-8.");
            line_iteration(&mut unique_lines, line, &mut input_count);
        }
    } else {
        //load from stdin
        let input = BufReader::new(io::stdin());

        //Tried to follow DRY but now I just hate types
        let input_lines = input.split(b'\n');
        for line in input_lines {
            let line = String::from_utf8(
                line.expect("Something went wrong while reading from stdin.")
                ).expect("Stdin contains invalid UTF-8.");
            line_iteration(&mut unique_lines, line, &mut input_count);
        }
    }

    return Summary {
        unique: unique_lines,
        total_count: input_count};
}

//format and print a Summary struct to stdout
fn print_results(summary: Summary) {
    for line in summary.unique {
        println!("{} ({:.2}%) {}",
            line.count,
            (line.count as f32 / summary.total_count as f32 * 100.0),
            line.pattern);
    }
    println!("Total items: {}", summary.total_count);
}

fn main() {
    print_results(count_occurances());

    //Nobody expects the spanish inquisition!
}

