use std::env;

use lalrpop_util::lalrpop_mod;
mod calendar;
lalrpop_mod!(calendar_grammar);
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }
    let filename = &args[1];
    let contents =
        std::fs::read_to_string(filename).expect("Something went wrong reading the file");
    let _ = calendar_grammar::CalendarParser::new()
        .parse(&contents)
        .map_err(|x| println!("Error {}", x.to_string()))
        .map(|events| {
            for event in events {
                println!("{}", event);
            }
        });
}
