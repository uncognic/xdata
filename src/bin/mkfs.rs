use xdata::mkfs;
fn main() {
    let name: Vec<String> = std::env::args().collect();
    if name.len() != 2 {
        println!("using default disk.img");
        let name = "disk.img";
        mkfs(name);
        std::process::exit(0);
    } else {
        mkfs(&name[1]);
        std::process::exit(0);
    }
}
