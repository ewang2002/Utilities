use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    if args.is_empty() {
        println!("spoiler: Discord Spoiler Markdown On Each Character");
        println!();
        println!("Usage:");
        println!("\tspoiler <query>");
        println!();
        println!("Examples:");
        println!("\tspoiler There was once something going on");
        return Ok(());
    }

    let mut spoiler_str = String::new();

    (args.join(" ") as String)
        .chars()
        // Do we need to use "String" or will "char" suffice?
        .map(|x| x.to_string())
        .for_each(|x| {
            spoiler_str.push_str(format!("||{}||", x).as_str());
        });

    println!("{}", spoiler_str);
    println!();
    println!("{} Characters", spoiler_str.len());
    return Ok(());
}
