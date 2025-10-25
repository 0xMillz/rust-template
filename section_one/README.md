## Getting started
`cargo build`  
`cargo run`

## Must-Know Resources
https://doc.rust-lang.org/book/

This is an excellent official resource and should be used alongside this course if you need help cementing any concepts.


https://play.rust-lang.org/?version=stable&mode=debug&edition=2021

This is a great resource if you want to play around with Rust, but don't want to start a whole rust project in your IDE. You can just go online and get started.

## Helpful commands
Clean  
`cargo clean`  
Show version  
`rustup --version`    
Update rust  
`rustup update`  
List toolchains  
`rustup toolchain list`  
Install specific version  
`rustup toolchain install <version>`  
Set default version  
`rustup default <version>`  
Uninstall a toolchain  
`rustup toolchain uninstall <version>`  
Add target for cross-compilation    
`rustup target add <target>`  
List installed components  
`rustup component list`  
Add a component  
`rustup component add <component>`  
Remove a component  
`rustup component remove <component>`  
Check which rustc binary is currently in use  
`rustup which rustc`