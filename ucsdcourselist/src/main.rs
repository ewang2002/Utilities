use html_escape::decode_html_entities;
use regex::Regex;
use reqwest::Client;
use std::borrow::Cow;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

macro_rules! def_str_const {
    ($identifier: ident, $expression: expr) => {
        const $identifier: &str = $expression;
    };
}

def_str_const!(
    GENERAL_CATALOG_PAGE,
    "https://catalog.ucsd.edu/front/courses.html"
);

// Basically, for this regular expression, we want to extract any string that is in
//              courses/____.html
//                      ^^^^
// To do this, we just match any letter or number in this area to capture.
def_str_const!(DEPT_CODE_REGEX_STR, r"courses/([a-zA-Z\d]+)\.html");

// The way UCSD renders each course and its description is very conveninent; in particular,
// each course is laid out in the form (the lines with ^^^ are not part of the website HTML):
//
//      <p class="anchor-parent"><a class="anchor" id="course code" name="course code"></a></p>
//      <p class="course-name">Course Code/Name/Units</p>
//                             ^^^^^^^^^^^^^^^^^^^^^^ (Group 1)
//      <p class="course-descriptions">description and prerequisites</p>
//                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ (Group 2)
//
// For this regular expression, we want to extract two things (two groups to capture); they are
// labeled above. Note that, because there is a newline separating the course name from
// description, we use \n in the regex to denote that.
def_str_const!(
    CRSE_DESC_REGEX_STR,
    r#"<p class="course-name">(.+)</p>\n<p class="course-descriptions">(.+)</p>"#
);

// All course names are (presumably) in the form
//
//      <course code>. <course name> (unit count)
//
// Thus, our regular expression is designed to capture each component.
// - ([a-zA-Z0-9-/]*\s+[a-zA-Z0-9-/]*\s+[a-zA-Z0-9-]*|[a-zA-Z0-9-/]*\s+[a-zA-Z0-9-/]*)
//               captures any string of the form XXX XXX, where X can be one of a letter,
//               number, hyphen, slash, or dash, **OR** captures any string of the form
//               XXX XXX XXX (X is defined above).
// - [.:\s]      matches either a dot, colon, or space
// - \s*?        lazily matches some number of spaces
// - (.*)        matches any symbols
// - \s+         matches > 0 spaces
// - \((.*)\)$   literally captures anything between the () at the end
def_str_const!(
    CRSC_CODE_NAME_UNIT_REGEX_STR,
    r#"([a-zA-Z0-9-–/\s]*)?[.:\s]\s*(.*)\s+\((.*)\)$"#
);

// This regular expression will attempt to capture everything, as described above, except the unit
// count (since some classes, e.g. COMM 101A, doesn't have a unit count).
def_str_const!(
    CRSC_CODE_NAME_REGEX_STR,
    r#"([a-zA-Z0-9-–/\s]*)?[.:\s]\s*(.*)"#
);

// A special regular expression for the case of courses with names like
//      Linguistics/Heritage Languages (XXXX) XXXX. YYYYY (ZZZZ)
def_str_const!(
    CRSC_LINGUISTICS_REGEX_STR,
    r#"\((.*)\)\s+([a-zA-Z0-9\s,]*)?[.:\s]\s*(.*)\s+\((.*)\)$"#
);

def_str_const!(
    CSRC_COMM_REGEX_STR,
    r#"([a-zA-Z0-9-–/]*\s+[a-zA-Z0-9-–/]*)?[.:\s]\s*(.*)\s+\((.*)\)$"#
);

/// Removes any `<...>` from the string.
///
/// # Parameters
/// - `str_to_check`: The string to check.
///
/// # Returns
/// The string, without any tags.
fn remove_tags(str_to_check: &str) -> String {
    let mut in_tag = false;
    let mut s = String::new();

    for c in str_to_check.chars() {
        if c == '<' {
            in_tag = true;
            continue;
        }

        if c == '>' {
            in_tag = false;
            continue;
        }

        if in_tag {
            continue;
        }

        s.push(c);
    }

    s
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
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

    let client = Client::new();
    let gen_catalog_html = client
        .get(GENERAL_CATALOG_PAGE)
        .send()
        .await?
        .text()
        .await?;

    let department_code_regex = Regex::new(DEPT_CODE_REGEX_STR)?;
    let course_desc_regex = Regex::new(CRSE_DESC_REGEX_STR)?;
    let gen_all_extract_regex = Regex::new(CRSC_CODE_NAME_UNIT_REGEX_STR)?;
    let gen_no_unit_extract_regex = Regex::new(CRSC_CODE_NAME_REGEX_STR)?;
    let gen_heritage_extract_regex = Regex::new(CRSC_LINGUISTICS_REGEX_STR)?;
    let gen_comm_extract_regex = Regex::new(CSRC_COMM_REGEX_STR)?;

    let mut writer = BufWriter::new(file);
    writeln!(
        writer,
        "department\tcourse_number\tcourse_name\tunits\tdescription"
    )?;

    let mut num_found = 0;
    for cap_code in department_code_regex.captures_iter(&gen_catalog_html) {
        let dept_code = cap_code[1].to_uppercase();
        println!("Processing Department: {}", dept_code);
        let course_url = format!("https://catalog.ucsd.edu/courses/{}.html", dept_code);
        let course_listing_html = client.get(&course_url).send().await?.text().await?;

        for cap_crsc in course_desc_regex.captures_iter(&course_listing_html) {
            let course_name_temp = remove_tags(decode_html_entities(&cap_crsc[1]).replace(" ", " ").trim());
            let course_name = course_name_temp.trim();
            let course_desc_temp = decode_html_entities(&cap_crsc[2]).replace(" ", " ");
            let course_desc = course_desc_temp.trim();

            let (code, name, units) = if course_name.starts_with("Linguistics") {
                // Case of linguistics/heritage
                match gen_heritage_extract_regex.captures(&course_name) {
                    Some(captures) => (
                        Cow::from(format!("{} {}", &captures[1], &captures[2])),
                        Cow::from(captures[3].to_string()),
                        Cow::from(captures[4].to_string()),
                    ),
                    None => panic!("poorly formatted heritage string: {}", &course_name),
                }
            } else if course_name.starts_with("COMM") && course_name.contains("CSI") {
                // COMM + CSI (because CSI is scuffed)
                match gen_comm_extract_regex.captures(&course_name) {
                    Some(captures) => (
                        Cow::from(captures[1].to_string()),
                        Cow::from(captures[2].to_string()),
                        Cow::from(captures[3].to_string()),
                    ),
                    None => panic!("poorly formatted comm string: {}", &course_name),
                }
            } else {
                match gen_all_extract_regex.captures(&course_name) {
                    Some(captures) => (
                        Cow::from(captures[1].to_string()),
                        Cow::from(captures[2].to_string()),
                        Cow::from(captures[3].to_string()),
                    ),
                    None => match gen_no_unit_extract_regex.captures(&course_name) {
                        Some(captures_alt) => (
                            Cow::from(captures_alt[1].to_string()),
                            Cow::from(captures_alt[2].to_string()),
                            Cow::from(""),
                        ),
                        None => panic!("poorly formatted general string: {}", &course_name),
                    },
                }
            };

            num_found += 1;
            writeln!(
                writer,
                "{}\t{}\t{}\t{}\t{}",
                dept_code,
                code,
                name,
                units,
                remove_tags(&*course_desc)
            )?;
        }
    }

    println!("Added {} courses to courses.tsv.", num_found);
    Ok(())
}
