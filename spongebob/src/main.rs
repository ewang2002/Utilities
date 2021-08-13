use std::env;
use std::error::Error;
use fastrand;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    if args.is_empty() {
        println!("spongebob: Spongebob Text Generator");
        println!();
        println!("Usage:");
        println!("\tspongebob <text>");
        println!();
        println!("Examples:");
        println!("\tspongebob Why is waffle bad");
        return Ok(());
    }

    let args_str: Vec<_> = (args.join(" ") as String)
        .chars()
        .map(|x| x.to_string())
        .collect();

    let mut rand_new_str = String::new();
    args_str.iter().for_each(|x| rand_new_str.push_str(
        (if fastrand::u32(0..=100) > 50 {x.to_uppercase()} else {x.to_lowercase()})
            .as_str()
    ));

    let mut dist_new_str = String::new();
    let mut i = 0;
    args_str.iter().for_each(|x| {
        dist_new_str.push_str(
            (if i & 1 == 0 {x.to_uppercase()} else {x.to_lowercase()}).as_str()
        );
        i += 1;
    });

    println!("Randomized : {}", rand_new_str);
    println!("Distributed: {}", dist_new_str);
    return Ok(());
}