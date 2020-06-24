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

extern crate clap;
use clap::{Arg, App};

pub fn args_prepare() -> clap::ArgMatches<'static> {
    return App::new("occrs")
                .version(clap::crate_version!())
                .about("Count occurrances of unique items in a list")
                .arg(Arg::with_name("FILE")
                     .help("The input file to process")
                     .index(1))
                .get_matches().clone();
}
