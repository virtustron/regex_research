use regex::Regex;

const SINGLE_OPTION_REGEX: &str = r"-{1,2}([[:alpha:]]+)";
const KEY_VALUE_OPTION_REGEX: &str = r"-([[:alpha:]]+=.+)";
const BOTH_OPTION_REGEX: &str = concat!(
    r"-{1,2}([[:alpha:]]+)", 
    r"|", 
    r"-([[:alpha:]]+=.+)");


const SINGLE_OPTIONS_EXAMPLE: &str = "-a -bb -ccc -ddd -  -- --- --v --verbose";
const KEY_VALUE_OPTIONS_EXAMPLE: &str ="-a=va -bb=vbb -ccc=vccc -ddd=vddd -e=  -eee= --e= ---- -=- ====";
const BOTH_OPTIONS_EXAMPLE: &str = concat!(
    "-a -bb -ccc -ddd -  -- --- --v --verbose", 
    "-a=va -bb=vbb -ccc=vccc -ddd=vddd -e=  -eee= --e= ---- -=- ====");


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
            println!("Cap[{}] length = {}. Cap[{}][0] = {}", &i, &cap.len(),  &i, &cap[0]);         
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



/*
    println!("Single options!");
    let key_value_options_string = "-a=va -bb=vbb -ccc=vccc -ddd=vddd -e=  -eee= --e= ---- -=- ====";
    println!("Example string: {}", key_value_options_string);

    let kay_value_options = key_value_options_string.split(" ");

    let mut i: i32 = 0;
    for option in kay_value_options.clone() {
        println!("Option {}: {}", i,  option);
        i = i + 1;
    }
        
    println!("Regex result:");
    let single_regex = Regex::new(r"-([[:alpha:]]+=.+)").unwrap();

    let mut i: i32 = 0;
    for option in single_options.clone() {
        for cap in single_regex.captures_iter(option) {
            println!("Cap[{}] length = {}. Cap[{}][0] = {}", &i, &cap.len(),  &i, &cap[0]);         
            i = i + 1;
        }
    }
    println!("Regex finished.\n");
*/
/*
    println!("Both options!");
    let both_options_string = " -a -bb -ccc -ddd -  -- --- --v --verbose-a=va -bb=vbb -ccc=vccc -ddd=vddd -e=  -eee= --e= ---- -=- ====";
    println!("Example string: {}", both_options_string);

    let single_options = both_options_string.split(" ");

    let mut i: i32 = 0;
    for option in single_options.clone() {
        println!("Option {}: {}", i,  option);
        i = i + 1;
    }
        
    println!("Regex result:");
    let single_regex = Regex::new(r"-([[:alpha:]]+=.+)").unwrap();

    let mut i: i32 = 0;
    for option in single_options.clone() {
        for cap in single_regex.captures_iter(option) {
            println!("Cap[{}] length = {}. Cap[{}][0] = {}", &i, &cap.len(),  &i, &cap[0]);         
            i = i + 1;
        }
    }
    println!("Regex finished.\n");
   */
}

