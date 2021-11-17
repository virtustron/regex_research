#[macro_use]
extern crate lazy_static;


// https://regex101.com/
//^(((-{1})([a-zA-Z_-]+)=[[:alpha:]]+)|(-{1,2})([[:alpha:]]+))$

use regex::Regex;

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

    // 1. Get options as string
    // 2. Get whitelist of options
    // 3. Split options by "space"
    // 4. Parse each option ad extract key-s from "key and value" pairs or only "key"
    // 5. Check each option key using given whitelist
    // 3. Return
    //    - vector of accepted options 
    //    - vector of discarded options



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
}


fn filter_compiler_options(
    options: &Vec<String>, 
    options_whitelist: &Vec<String>, 
    ) -> Result<Vec<String>, Vec<String>> {


    }



fn extract_option_key(input: &str) -> Option<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(-{1})(?P<key>([[:alpha:]]+))(=)(?P<value>(.+))").unwrap();
    }
    RE.captures(input).and_then(|cap| {cap.name("key").map(|key| key.as_str())})
}

