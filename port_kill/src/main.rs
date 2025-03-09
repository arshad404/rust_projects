use std::env;
use std::process::Command;

fn list_all_process() {
    let output = Command::new("lsof")
        .arg("-i")
        .arg("-P")
        .arg("-n")
        .arg("-sTCP:LISTEN")
        .output()
        .expect("Failed to execute lsof");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{}", stdout);
}

fn get_pid_on_port(port: &str) -> Option<String> {
    let output = Command::new("lsof")
        .arg("-i")
        .arg(format!(":{}", port))
        .arg("-t")
        .output()
        .expect("Failed to execute lsof");

    let stdout = String::from_utf8(output.stdout).unwrap().trim().to_string();
    if stdout.is_empty() {
        println!("No process found running on port {}", port);
        None
    } else {
        Some(stdout)
    }
}

fn kill_process(pid: &str) {
    println!("Killing process {}", pid);
    Command::new("kill")
        .arg("-9")
        .arg(pid)
        .output()
        .expect("Failed to execute kill");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Usage:");
        println!("  {} list       - List all running ports", args[0]);
        println!("  {} kill <port> - Kill process on given port", args[0]);
        return;
    }

    match args[1].as_str() {
        "list" => list_all_process(),
        "kill" if args.len() == 3 => {
            let pid = get_pid_on_port(args[2].as_str());
            if pid.is_some() {
                kill_process(pid.unwrap().as_str())
            } else {
                println!("No process found running on port {}", args[2])
            }
        },
        _ => println!("Invalid command"),
    }

}