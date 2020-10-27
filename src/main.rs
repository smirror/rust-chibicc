use lib::*

fn main() {
    let arg = std::env::args().nth(1).unwrap();
    
    let p = {
        usize::from_str_radix(arg.as_str(), 10).unwrap()
    };
    // The first token must be a number
    println!(".intel_syntax noprefix");
    println!(".global main");

    // ... followed by either `+ <number>` or `- <number>`.
    println!("main:");
    println!("   mov rax, {}", p);
    while (p){
        if (p == "+"){
            println!("  add rax, {}", p)
        } else if(p == "-") {
            println!("  sub rax, {}", p)
        }else{
            println!("  add rax, {}", p)
        }
    }
    println!("   ret");
}
