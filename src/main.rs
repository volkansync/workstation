fn main() {
    let ad = "workstation";
    let surum = "0.1.0";
    banner(ad, surum);
    println!("preparing workstation");
}

fn banner(ad: &str, surum: &str) {
    println!("{} {}", ad, surum);
}
