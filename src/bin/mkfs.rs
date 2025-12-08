use xdata::mkfs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("using default disk.img");
        mkfs("disk.img");
        std::process::exit(0);
    }
    mkfs(&args[1]);
}
