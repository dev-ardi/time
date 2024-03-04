use std::{process::{exit, Command}, time::Instant};

fn main() {
    let now = Instant::now();
    let mut args =  std::env::args_os().skip(1);
    let Some(prog) = args.next() else { 
        println!(
"Usage: time <command>. 
time itself took {:?}", now.elapsed());
        exit(1);
    };
    let code = Command::new(&prog).args(args).spawn().unwrap().wait().unwrap();
    println!("\n{prog:?} took {:?}", now.elapsed());
    exit(code.code().unwrap_or(1));
}
