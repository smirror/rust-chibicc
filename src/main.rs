use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut arg = args[1].to_string();
    arg.retain(|c| c != ' ');
    arg += " ";
    // The first token must be a number
    println!(".intel_syntax noprefix");
    println!(".global main");

    // ... followed by either `+ <number>` or `- <number>`.
    println!("main:");
    let mut n = "".to_string();
    let mut operator = '\0';
    for p in arg.as_str().chars() {
        if !p.is_ascii_digit() {
            match operator {
                '\0' => {
                    println!("   mov rax, {}", n);
                    n = "".to_string();
                }
                '+' => {
                    println!("   add rax, {}", n);
                    n = "".to_string();
                }
                '-' => {
                    println!("   sub rax, {}", n);
                    n = "".to_string();
                }
                _ => {}
            }
            operator = p;
        } else {
            n.push(p);
        }
    }
    println!("   ret");
}
