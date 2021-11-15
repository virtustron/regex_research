use regex::Regex;

fn main() {
    println!("Hello Regex!\n");

    // (?P<email>[a-z]+@(?P<email_domain>[a-z]+[.](com|org|net))) | (?P<domain>[a-z]+[.](com|org|net))
    


    println!("Single options!");
    let single_options_string = "-a -bb -ccc -ddd -  -- --- --v --verbose";
    println!("Example string: {}", single_options_string);

    let single_options = single_options_string.split(" ");

    let mut i: i32 = 0;
    for option in single_options.clone() {
        println!("Option {}: {}", i,  option);
        i = i + 1;
    }
        
    println!("Regex result:");
    let single_regex = Regex::new(r"-{1,2}([[:alpha:]]+)").unwrap();

    let mut i: i32 = 0;
    for option in single_options.clone() {
        for cap in single_regex.captures_iter(option) {
            println!("Cap[{}] length = {}. Cap[{}][0] = {}", &i, &cap.len(),  &i, &cap[0]);         
            i = i + 1;
        }
    }



    println!("Single options!");
    let key_value_options_string = "-a=va -bb=vbb -ccc=vccc -ddd=vddd -e=  -eee= --e= ---- -=- ====";
    println!("Example string: {}", key_value_options_string);

    let single_options = key_value_options_string.split(" ");

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


    



    /*
    let text = "-a -bb -ccc -ddd -e=e -ff=ff -ggg=gggg";

    for cap in re.captures_iter(text) {
        println!("{}", &cap[0]);
    }
    */








}
