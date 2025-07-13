use std::io::Read;
use std::process::exit;

use selfdeletion;

fn main() {
    println!("[*] Deleting self...");
    if let Err(e) = selfdeletion::delete_self() {
        eprintln!("{}", e);
        exit(1);
    }

    println!("[+] {} Should Be Deleted", std::env::args().next().unwrap());

    println!("[#] Press <Enter> To Quit ... ");
    let _ = std::io::stdin().read(&mut [0u8]);

    exit(0);
}
