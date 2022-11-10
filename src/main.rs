mod wol;

fn main() {
    println!("Waking the wizzard, here be dragons");

    match wol::wol("18:c0:4d:c0:72:7f", "255.255.255.255") {
        Ok(_) => println!("The computer has been woken! maybe.."),
        Err(e) => println!("Somethign went wrong: {}", e),
    }
}
