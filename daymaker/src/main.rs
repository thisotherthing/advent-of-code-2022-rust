use chrono::Datelike;
use std::env;
use std::fs;

fn main() {
    dotenv::dotenv().ok();

    let current_date = chrono::Utc::now();

    // get day from args
    let args: Vec<String> = env::args().collect();
    let year: String = if args.len() >= 2 && args[1].len() == 4 {
        String::from(&args[1])
    } else {
        current_date.year().to_string()
    };
    let day = if args.len() >= 3 && !args[2].is_empty() && args[2].len() < 3 {
        String::from(&args[2])
    } else {
        current_date.day().to_string()
    };

    let day_path = format!("./days/day{day}");

    // fetch input
    {
        let url = format!("https://adventofcode.com/{year}/day/{day}/input");

        let client = reqwest::blocking::Client::new();

        let resp = client
            .get(url)
            .header(
                reqwest::header::COOKIE,
                format!("session={}", env::var("SESSION_ID").unwrap()),
            )
            .send()
            .expect("Unable to download")
            .text()
            .expect("can't parse response body");

        // ensure file path exists
        let path = format!("{day_path}/src");
        assert!(fs::create_dir_all(&path).is_ok());

        // write input
        fs::write(format!("{path}/input.txt"), resp).expect("Unable to write file");
    }

    // write other day files
    {
        let template_path = "./days/template";

        let cargo_toml_contents = fs::read_to_string(format!("{template_path}/Cargo.toml"))
            .expect("Should have been able to read the file");

        // cargo.toml
        fs::write(
            format!("{day_path}/Cargo.toml"),
            cargo_toml_contents.replace("template", format!("day{day}").as_str()),
        )
        .expect("unable to write Cargo.toml");

        // main.rs
        fs::copy(
            format!("{template_path}/src/main.rs"),
            format!("{day_path}/src/main.rs"),
        )
        .expect("Unable to write main.rs");

        // example.txt
        fs::write(format!("{day_path}/src/example.txt"), "123")
            .expect("unable to write example.txt");
    }

    // open url
    {
        let day_url = format!("https://adventofcode.com/{year}/day/{day}");
        open::that(day_url).unwrap();
    }

    let run_command = format!("run with 'cargo run --release --package day{day}'");
    dbg!(run_command);
}
