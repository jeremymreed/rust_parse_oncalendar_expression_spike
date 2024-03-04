use std::{thread, time::{self, Instant}};
use chrono::Local;

mod systemd_analyze;

fn main() {
    println!("Getting next event datetime!");

    let sleep_duration = time::Duration::from_secs(1);

    let input: String = "*-*-* *:0/2".to_string();
    println!("input: {:?}", input);

    let target  = systemd_analyze::get_next_event(&input);

    loop {
        let now = Instant::now();
        let dt_now = Local::now();
        println!("target: {:?}", target);
        println!("dt_now: {:?}", dt_now);

        let delta = target - dt_now;

        println!("delta:        {:?}", delta);

        match delta.num_seconds() <= 0 {
            true => {
                println!("dt_now <= target");
                return;
            },
            false => println!("dt_now != target"),
        }

        let elapsed = now.elapsed();
        println!("Elapsed time: {:#?}", elapsed);
        println!("\n");

        thread::sleep(sleep_duration);
    }
}
