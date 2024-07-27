use std::io::Write;
use std::process;
use syscalls::syscall;

fn exec_cmd() -> i32 {
    0
}

fn parse_builtins(cmd: &str) {
    match cmd {
        "exit" => process::exit(0x0),
        _ => return
    }
}

fn shell_loop() {
    let mut cmd = String::new();

    loop {
        //print the prompt
        print!("$ ");

        //flush stdio to ensure output happens
        std::io::stdout().flush().unwrap();

        cmd.clear();
        std::io::stdin().read_line(&mut cmd).unwrap();

        //check for builtins
        parse_builtins(&cmd);
    }
}

fn main() {
    shell_loop();
}
