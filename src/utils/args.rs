use std::env;

pub fn get_var(i: usize) -> Option<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() > i {
        Some(args[i].clone())
    } else {
        None
    }
}