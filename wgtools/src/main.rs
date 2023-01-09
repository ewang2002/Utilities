use clap::Parser;
use std::fs;
use std::fs::OpenOptions;
use std::io::{stdin, BufWriter, Write};
use std::path::Path;
use tokio::time::Instant;
use webweg::types::MeetingDay;
use webweg::wrapper::{SearchRequestBuilder, SearchType, WebRegWrapper};

const MIN_YEAR: usize = 22;
const MAX_YEAR: usize = 24;
const TSV_HEADER: &str = "subj_course_id\tsec_code\tsec_id\tinstructor\ttotal_seats\tmeetings";

/// Puts all sections offered for a term into a TSV file.
///
/// # Parameters
/// - `w`: The `WebRegWrapper` reference.
async fn export_all_sections(w: &WebRegWrapper) {
    let file_name = format!("{}.tsv", w.get_term());
    let mut writer = BufWriter::new(
        OpenOptions::new()
            .write(true)
            .create(true)
            .open(&file_name)
            .expect("something went wrong when trying to create file."),
    );

    writeln!(writer, "{}", TSV_HEADER).unwrap();

    let results = match w
        .search_courses(SearchType::Advanced(&SearchRequestBuilder::new()))
        .await
    {
        Ok(o) => o,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let start_time = Instant::now();
    let mut total_added = 0;
    for res in results {
        println!(
            "Processing {} {}.",
            res.subj_code.trim(),
            res.course_code.trim()
        );
        let mut added = 0;
        w.get_course_info(res.subj_code.trim(), res.course_code.trim())
            .await
            .unwrap_or_default()
            .into_iter()
            .for_each(|c| {
                let meeting_str = c
                    .meetings
                    .into_iter()
                    .map(|m| {
                        let day_meet = match &m.meeting_days {
                            MeetingDay::Repeated(r) => r.join(""),
                            MeetingDay::OneTime(r) => r.to_string(),
                            MeetingDay::None => "N/A".to_string(),
                        };

                        format!(
                            "{},{},{}:{:02} - {}:{:02},{} {}",
                            m.meeting_type,
                            day_meet,
                            m.start_hr,
                            m.start_min,
                            m.end_hr,
                            m.end_min,
                            m.building,
                            m.room,
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("|");

                writeln!(
                    writer,
                    "{}\t{}\t{}\t{}\t{}\t{}",
                    c.subj_course_id,
                    c.section_code,
                    c.section_id,
                    c.all_instructors.join(" & "),
                    c.total_seats,
                    meeting_str
                )
                .unwrap();
                added += 1;
            });
        println!(
            "\tAdded {} sections of {} {} successfully.",
            added,
            res.subj_code.trim(),
            res.course_code.trim()
        );
        total_added += added;
    }

    println!(
        "Processed {} sections in {} SEC.",
        total_added,
        start_time.elapsed().as_secs_f32()
    );
    writer.flush().unwrap();
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Whether to provide the cookie string manually. If this is not provided, then it is
    /// assumed that there is a `cookie.txt` file in the directory that this executable is in.
    #[clap(short, long)]
    cookie: bool,

    /// The term that you want to request. Your cookies should correspond to this particular term.
    /// Some example inputs are 'SP22' or 'S122'.
    #[clap(short, long)]
    term: String,

    /// The type of data to request. For now, you can either request 'sections' or 'schedule'.
    #[clap(short, long)]
    data: String,
}

enum SelectedChoice {
    Sections,
    Schedule,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args: Args = Args::parse();

    let mut cookies = String::new();
    if args.cookie {
        println!("Please paste your cookies now.");
        if let Err(e) = stdin().read_line(&mut cookies) {
            eprintln!(
                "An error occurred when trying to read from standard input: {}",
                e
            );

            return;
        }

        cookies = cookies.trim().to_string();
    } else {
        let file = Path::new("cookie.txt");
        if !file.exists() {
            eprintln!("No 'cookie.txt' file found.");
            return;
        }

        cookies.push_str(&fs::read_to_string(file).unwrap_or_else(|_| "".to_string()));
    }

    // Parse choice
    let choice = match args.data.to_lowercase().as_str() {
        "sections" => SelectedChoice::Sections,
        "schedule" => SelectedChoice::Schedule,
        _ => {
            eprintln!("Only 'sections' or 'schedule' can be requested at this time.");
            return;
        }
    };

    let term = args.term.to_uppercase();
    let mut parsed_term = String::new();

    match &term[..2] {
        "FA" | "WI" | "SP" | "S1" | "S2" | "S3" | "SA" => parsed_term.push_str(&term[..2]),
        _ => {
            eprintln!(
                "Invalid term session; term must start with one of: 'FA', 'WI', 'SP', 'S1', 'S2', 'S3', 'SA'."
            );
            return;
        }
    }

    let yr = term[2..].parse::<usize>().unwrap_or_default();
    if !(MIN_YEAR..=MAX_YEAR).contains(&yr) {
        eprintln!(
            "Year {} not supported; year must be less than {} and greater than {}.",
            &term[2..],
            MAX_YEAR,
            MIN_YEAR
        );
        return;
    }

    parsed_term.push_str(&term[2..]);

    let wrapper = WebRegWrapper::new(webweg::reqwest::Client::new(), cookies, &parsed_term);
    if !wrapper.is_valid().await {
        eprintln!("An error occurred when trying to authenticate. Please try again with a new set of cookies.");
        return;
    }

    match choice {
        SelectedChoice::Sections => export_all_sections(&wrapper).await,
        SelectedChoice::Schedule => {
            eprintln!("'schedule' is not supported at this time.");
        }
    };
}
