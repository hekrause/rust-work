use std::time::Instant;
use std::time::Duration;
use std::thread::sleep;
use std::process::Command;

fn main() {
    let mut input: Vec<String> = std::env::args().collect();
    let _unit: String = input.swap_remove(1); ;
    let ivec = convert(input).ok().unwrap();
    let _start = ivec[2];
    let end = ivec[0];
    let step = ivec[1];
    let mut loops = 0;
    let mut increment = step;

    config_thread();
    sleep(Duration::new(1, 0));

    while increment < end {
        let mut max_delay: std::time::Duration = Duration::new(0, 0);
        while loops < 100 {
            let start_time = Instant::now();
            sleep(Duration::new(0, increment as u32));
            let stop_time = Instant::now();

            let elapsed_time = stop_time.duration_since(start_time);

            if elapsed_time > max_delay {
                max_delay = elapsed_time;
            }

            loops += 1;
        }
        println!("{} {}", increment, ((max_delay.subsec_nanos() as f64) - increment as f64)/1000 as f64);
        increment += step;
        loops = 0;
    }
}

pub fn config_thread() {
	let _rt_simple_thread = Command::new("sh")
    .arg("real_time.sh")
    .spawn()
    .expect("command failed to start");
}

/*
pub fn config_parent_thread() {
    let simple_thread = Command::new("sudo")
    .arg("pgrep")
    .arg("-f")
    .arg("simple_thread")
    .output()
    .expect("command failed to start");

    let pid_simple_thread = String::from_utf8_lossy(&simple_thread.stdout);

    let _rt_simple_thread = Command::new("chrt")
    .arg("-f")
    .arg("-p")
    .arg("99")
    .arg(pid_simple_thread.into_owned())
    .output()
    .expect("command failed to start");
}

pub fn config_thread() {
    let simple_thread = Command::new("pgrep")
    .arg("-f")
    .arg("simple_thread")
    .output()
    .expect("command failed to start");

    let pid_simple_thread = String::from_utf8_lossy(&simple_thread.stdout);

    println!("Give RT rights to:{} simple_thread", pid_simple_thread);
    let _rt_simple_thread = Command::new("chrt")
    .arg("-f")
    .arg("-p")
    .arg("99")
    .arg(pid_simple_thread.into_owned())
    .output()
    .expect("command failed to start");

    let cargo = Command::new("pgrep")
    .arg("-f")
    .arg("cargo")
    .output()
    .expect("command failed to start");

    let pid_cargo = String::from_utf8_lossy(&cargo.stdout);

    println!("Give RT rights to:{} cargo", pid_cargo);
    let _rt_simple_thread = Command::new("chrt")
    .arg("-f")
    .arg("-p")
    .arg("99")
    .arg(pid_cargo.into_owned())
    .output()
    .expect("command failed to start");
}
*/

pub fn convert(mut args: Vec<String>) -> Result<Vec<u64>, &'static str> {
    args.swap_remove(0);
    let tmp: Result<Vec<u64>, _> = args.into_iter().map(|s| s.parse::<u64>()).collect();

    if tmp.is_ok() {
        Ok(tmp.ok().unwrap())
    } else {
        Err(&"parameter is no integer")
    }
}
