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

    let spoiler_str: String = (args.join(" ") as String)
        .chars()
        .map(|x| format!("||{}||", x))
        .collect::<Vec<_>>()
        .join("");

    println!("{} Characters", spoiler_str.len());
    println!();
    println!("{}", spoiler_str);
    return Ok(());
}
