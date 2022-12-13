use chrono::Local;
use clap::{Arg, ArgMatches, Command};
use notify_rust::{Notification, Timeout};
use std::{thread, time};

fn parse_args() -> ArgMatches {
    Command::new("Romodoro")
        .version("1.0")
        .about(
            "Time management with love from Rust\n\n \
       (see https://en.wikipedia.org/wiki/Pomodoro_Technique for other info)",
        )
        .author("Alessio Giambrone")
        .arg(
            Arg::new("timer")
                .short('t')
                .long("timer")
                .default_value("25")
                .help("length of the work/study/... step, in minutes"),
        )
        .arg(
            Arg::new("relax")
                .short('r')
                .long("relax")
                .default_value("5")
                .help("length of the relax step, in minutes"),
        )
        .arg(
            Arg::new("repetitions")
                .short('x')
                .long("repetitions")
                .default_value("4")
                .help("number of pomodoros"),
        )
        .get_matches()
}

fn parse_number(matches: &ArgMatches, name: &str) -> u64 {
    matches
        .get_one::<String>(name)
        .unwrap()
        .parse::<u64>()
        .unwrap()
        * 60
}

fn notify(summary: &str, body: &str, timeout: u64) {
    Notification::new()
        .summary(summary)
        .body(body)
        .appname("romodoro")
        .timeout(Timeout::Milliseconds((timeout as u32) * 1000))
        .sound_name("bell")
        .show()
        .expect("error while showing notification");
    // let ends = local + Duration::seconds(*timer as i64);
    println!("{:?}: {}\n {}", Local::now(), summary, body);
}

fn pomodoro_loop(timer: u64, relax: u64, repetitions: u64) {
    for i in 0..repetitions {
        pomodoro_iteration(&timer, &relax, i == repetitions - 1);
    }
}

fn pomodoro_iteration(timer: &u64, relax: &u64, is_last: bool) {
    notify(
        "Pomodoro started!",
        &format!("Will end in {} minutes", &(timer / 60)),
        *timer,
    );

    thread::sleep(time::Duration::from_secs(*timer as u64));

    if is_last {
        notify(
            "You finished your pomodoro session!",
            "Congrats, now its time for some review",
            0,
        );
    } else {
        notify(
            "Pomodoro iteration ended!",
            &format!("Will start again in {} minutes", &(relax / 60)),
            *relax,
        );
        thread::sleep(time::Duration::from_secs(*relax as u64));
    }
}

fn main() {
    let matches = parse_args();

    let timer = parse_number(&matches, "timer");
    let relax = parse_number(&matches, "relax");
    let repetitions = parse_number(&matches, "repetitions");

    pomodoro_loop(timer, relax, repetitions);
}
