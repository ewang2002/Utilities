use html_escape::decode_html_entities;
use regex::Regex;
use reqwest::Client;
use std::borrow::Cow;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

const GENERAL_CATALOG_PAGE: &str = "https://catalog.ucsd.edu/front/courses.html";
const DEPT_CODE_REGEX_STR: &str = r"courses/([a-zA-Z\d]+)\.html";
const CRSE_DESC_REGEX_STR: &str =
    r#"<p class="course-name">(.+)</p>\n<p class="course-descriptions">(.+)</p>"#;
const PREREQ_REGEX_STR: &str = r#"<strong class="italic">.*Prerequisites:.*</strong>(.*)"#;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let gen_catalog_html = client
        .get(GENERAL_CATALOG_PAGE)
        .send()
        .await?
        .text()
        .await?;

    let department_code_regex = Regex::new(DEPT_CODE_REGEX_STR)?;
    let course_desc_regex = Regex::new(CRSE_DESC_REGEX_STR)?;
    let prereq_regex = Regex::new(PREREQ_REGEX_STR)?;

    let curr_dir = std::env::current_dir()?.join("courses.tsv");
    if curr_dir.exists() {
        println!("File '{:?}' already exists.", curr_dir);
        return Ok(());
    }

    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&curr_dir)
        .unwrap_or_else(|_| panic!("Could not open or create file '{:?}'", curr_dir));
    let mut writer = BufWriter::new(file);
    writeln!(writer, "course_name\tdescription\tprerequisites")?;

    let mut num_found = 0;
    for cap_code in department_code_regex.captures_iter(&gen_catalog_html) {
        let dept_code = &cap_code[1];
        println!("Processing Department Code: {}", dept_code);
        let course_url = format!("https://catalog.ucsd.edu/courses/{}.html", dept_code);
        let course_listing_html = client.get(&course_url).send().await?.text().await?;

        for cap_crsc in course_desc_regex.captures_iter(&course_listing_html) {
            let course_name = decode_html_entities(&cap_crsc[1]);
            let mut course_desc = decode_html_entities(&cap_crsc[2]);

            let prereqs = match prereq_regex.captures(&*course_desc) {
                Some(capture) => Cow::from(capture[1].trim().to_string()),
                None => Cow::from("N/A"),
            };

            if prereqs != "" {
                course_desc = Cow::from(prereq_regex.replace(&*course_desc, "").trim().to_string());
            }

            num_found += 1;
            writeln!(writer, "{}\t{}\t{}", course_name, course_desc, prereqs)?;
        }
    }

    println!("Added {} courses to courses.tsv.", num_found);
    Ok(())
}
