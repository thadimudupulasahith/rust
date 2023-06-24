
//wsl --install
//In the WSL terminal, run the following command to download and install the Rust installation script:
//curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
//After the installation is complete, you'll be prompted to run 
//the following command to configure the current shell session
//source $HOME/.cargo/env
//cargo new filename
//cargo build for compile
//cargo run for ti run compiled file

fn main() {
    print();
    add();
    mutable();
    scope();
}
fn print(){
    println!("hello, world");
}
fn add(){
    //let is used to declare the variables
    let x: i8 = 2;
    let y: i8 = 4;
    println!("{}",x+y);
}
fn mutable(){
    let mut x = 4;
    x += 5;
    println!("{}",x);
}
fn scope(){
    let x = 3;
    let y = 4;
    {
        println!("{}",x+y);
    }
    println!("{}",x-y);
}
