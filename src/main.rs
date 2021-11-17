#[macro_use]
extern crate lazy_static;


// https://regex101.com/
//^(((-{1})([a-zA-Z_-]+)=[[:alpha:]]+)|(-{1,2})([[:alpha:]]+))$

use regex::Regex;

use std::collections::HashSet;
use std::iter::FromIterator;

const OPTIONS_SEPARATOR: &str = r" ";

const SINGLE_OPTION_REGEX: &str = r"-{1,2}([[:alpha:]]+)";
const KEY_VALUE_OPTION_REGEX: &str = r"-([[:alpha:]]+=.+)";

const BOTH_OPTION_REGEX: &str = r"(?P<key_value>(-{1})([[:alpha:]]+=.+)( |$))|(?P<single>(-{1,2})([[:alpha:]]+)( |$))";
    


const SINGLE_OPTIONS_EXAMPLE: &str = "-a -bb -ccc -ddd -  -- --- --v --verbose";
const KEY_VALUE_OPTIONS_EXAMPLE: &str ="-a=va -bb=vbb -ccc=vccc -ddd=vddd -e=  -eee= --e= ---- -=- ====";
const BOTH_OPTIONS_EXAMPLE: &str = concat!(
    "-a -bb -ccc -ddd -  -- --- --v --verbose", 
    " -a=va -bb=vbb -ccc=vccc -ddd=vddd -e=  -eee= --e= ---- -=- ====");


fn main() {
    println!("Hello Regex!\n");





    // ---------------------------------
    println!("Single options!");
    println!("Example string: {}", SINGLE_OPTIONS_EXAMPLE);

    let options = SINGLE_OPTIONS_EXAMPLE.split(" ");
    let mut i: i32 = 0;

    for option in options.clone() {
        println!("Option {}: {}", i,  option);
        i = i + 1;
    }
        
    println!("Regex result:");
    let regex = Regex::new(SINGLE_OPTION_REGEX).unwrap();

    let mut i: i32 = 0;
    for option in options.clone() {
        for cap in regex.captures_iter(option) {
            println!("Cap[{}] length = {}. Cap[{}][0] = {}", &i, &cap.len(),  &i, &cap[0]);         
            i = i + 1;
        }
    }
    println!("Regex finished.\n");


    // ---------------------------------
    println!("Key=Value options!");
    println!("Example string: {}", KEY_VALUE_OPTIONS_EXAMPLE);

    let options = KEY_VALUE_OPTIONS_EXAMPLE.split(" ");
    let mut i: i32 = 0;

    for option in options.clone() {
        println!("Option {}: {}", i,  option);
        i = i + 1;
    }
        
    println!("Regex result:");
    let regex = Regex::new(KEY_VALUE_OPTION_REGEX).unwrap();

    let mut i: i32 = 0;
    for option in options.clone() {
        for cap in regex.captures_iter(option) {
            let key_value = extract_option_key(&cap[0]);

            println!("Cap[{}] length = {}. Cap[{}][0] = {}. Option key = {}", &i, &cap.len(),  &i, &cap[0], &key_value.unwrap());         
            i = i + 1;
        }
    }
    println!("Regex finished.\n");


    // ---------------------------------
    println!("Both options!");
    println!("Example string: {}", BOTH_OPTIONS_EXAMPLE);

    let options = BOTH_OPTIONS_EXAMPLE.split(" ");
    let mut i: i32 = 0;

    for option in options.clone() {
        println!("Option {}: {}", i,  option);
        i = i + 1;
    }
        
    println!("Regex result:");
    let regex = Regex::new(BOTH_OPTION_REGEX).unwrap();

    let mut i: i32 = 0;
    for option in options.clone() {
        for cap in regex.captures_iter(option) {
            println!("Cap[{}] length = {}. Cap[{}][0] = {}", &i, &cap.len(),  &i, &cap[0]);         
            i = i + 1;
        }
    }
    println!("Regex finished.\n");


    // 1. Get options as string
    // 2. Get whitelist of options
    // 3. Split options by "space"
    // 4. Parse each option ad extract key-s from "key and value" pairs or only "key"
    // 5. Check each option key using given whitelist
    // 3. Return
    //    - vector of accepted options 
    //    - vector of discarded options

    // 1. Get options as string
    let raw_options = String::from(BOTH_OPTIONS_EXAMPLE);

    // 2. Get whitelist of options
    // it's temporary - JSON will be used
    let mut options_whitelist: Vec<String> = Vec::new();
    options_whitelist.push("-v".to_string());

    // 3. Split options by "space"
    let options: Vec<String> = raw_options.split(OPTIONS_SEPARATOR).map(|s| s.to_string()).collect();

    
    let options_whitelist: Vec<String> = Vec::new();
    
    let filtering_result =  filter_compiler_options(&options, &options_whitelist);
    
    match filtering_result {
        Ok(filtered_options) => {
            println!("Accepted options list:");
            
            for option in filtered_options {
                println!("Option: {}", option);
            }

        }
        
        Err(filtered_options) => {
            println!("Declined options list:");
            
            for option in filtered_options {
                println!("Option: {}", option);
            }
        }
    }
}


fn to_hashset(vector: &Vec<String>) -> HashSet<String> {
    HashSet::from_iter(vector.iter().cloned())
}

// if there are one or more option in "options" from "whitelist"
//     - return Ok(accepted_options)
// else (zero "accepted option")
//     - return Err(declined_options)
fn filter_compiler_options(options: &Vec<String>, options_whitelist: &Vec<String>) -> Result<Vec<String>, Vec<String>> {
    let options_set: HashSet<String>  = to_hashset(options);
    let options_whitelist_set: HashSet<String> = to_hashset(options_whitelist);

    let accepted_options_set: HashSet<&String>  = options_set.intersection(&options_whitelist_set).collect();
    let declined_options_set: HashSet<&String> =  options_set.difference(&options_whitelist_set).collect();

    let accepted_options: Vec<String> = accepted_options_set.into_iter().map(|s| s.to_string()).collect();    
    let declined_options: Vec<String> = declined_options_set.into_iter().map(|s| s.to_string()).collect();

    if accepted_options.len() == 0 {
        Err(declined_options)
    } 
    else {
        Ok(accepted_options)
    }
}




fn extract_option_key(input: &str) -> Option<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(-{1})(?P<key>([[:alpha:]]+))(=)(?P<value>(.+))").unwrap();
    }
    RE.captures(input).and_then(|cap| {cap.name("key").map(|key| key.as_str())})
}

