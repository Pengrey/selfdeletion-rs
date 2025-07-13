# Self deletion (Rust Implementation)

Rust implementation of selfdeletion with patch for Windows 11.

## Overview

This library allows the binary to self delete even if it's still running.

## Usage

### Basic Example

Add this to your `Cargo.toml`:

```toml
[dependencies]
selfdeletion = { git = "https://github.com/Pengrey/selfdeletion-rs.git", branch = "main" }
```

```rust
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
```

### Library Functions

- `delete_self()`: Invokes the deletion of the binary

## Requirements

- Windows operating system
- Rust toolchain
- `windows` crate with appropriate features

## License

This project is provided as-is for educational purposes. Please refer to the original blog post for attribution and additional context.

## Credits

- **Original Research**: @jonasLyk and TKYNSEC
- **Original Blog**: [The Not So Self Deleting Executable on 24h2](https://tkyn.dev/2025-6-8-The-Not-So-Self-Deleting-Executable-on-24h2/)
- **Rust Implementation**: Based on the original C++ proof-of-concept 
