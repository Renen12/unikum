use std::{
    env::args,
    fs,
    process::{Command, exit},
};

use serde_json::Value;
static HELP_MESSAGE: &'static str = "
--help — display this message\n 
\t--shibsession_name=[your shibsession cookie's name]\n
\t--shibsession_value=[your shibsession cookie's value]\n
\t--jsessionid=[your jsessionid]\n
 \t--unihzsessid=[your unihzsessid]\n
  \t--pid=[the pid of the blog page, e.g 11349120275]\n
  \t --html — output these into respective HTML files\n
  \t --dry — show raw data
  Instructions on how to obtain these is in the README.md file. This must be run in a directory with the proper return_json.ps1 file.
  \n Such a file is provided in the repository.";
#[allow(unused)]
fn main() {
    let args: Vec<_> = args().collect();
    let mut jsessionid = String::new();
    let mut dry = false;
    let mut html = false;
    let mut unihzsessid = String::new();
    let mut shibsession_name = String::new();
    let mut shibsession_value = String::new();
    let mut pid = String::new();
    let pwsh_contents = fs::read_to_string("return_json.ps1").unwrap();
    for arg in args {
        let split_equals: Vec<&str> = arg.split("=").collect();
        if arg == "--dry" {
            dry = true;
        }
        if arg == "--html" {
            html = true;
        }
        if arg.contains("--jsessionid") {
            jsessionid = split_equals[1].to_owned();
        }
        if arg.contains("--shibsession_name") {
            shibsession_name = split_equals[1].to_owned();
        }
        if arg.contains("--shibsession_value") {
            shibsession_value = split_equals[1].to_owned();
        }
        if arg.contains("--unihzsessid") {
            unihzsessid = split_equals[1].to_owned();
        }
        if arg.contains("--pid") {
            pid = split_equals[1].to_owned();
        }
        if arg == "--help" {
            print!("{}", HELP_MESSAGE);
            exit(0);
        }
    }
    if !jsessionid.is_empty()
        || !unihzsessid.is_empty()
        || !pid.is_empty()
        || !shibsession_name.is_empty()
        || !shibsession_value.is_empty()
    {
        let out = Command::new("pwsh")
            .args([
                "return_json.ps1",
                &jsessionid,
                &unihzsessid,
                &shibsession_name,
                &shibsession_value,
                &pid,
            ])
            .output()
            .unwrap();
        let out_readable = String::from_utf8(out.stdout).unwrap();
        if dry {
            println!("{}", out_readable);
        } else {
            let json: Value = match serde_json::from_str(&out_readable) {
                Ok(v) => v,
                Err(_) => {
                    eprintln!("Cannot read JSON from Unikum.");
                    exit(1);
                }
            };
            let main_list = match json["list"].as_array() {
                Some(v) => v,
                None => {
                    eprintln!("Cannot turn the main JSON list into an array");
                    exit(1);
                }
            };
            let mut item = 1;
            for blog in main_list {
                let mut content = String::from(blog["contentHTML"].as_str().unwrap());
                content.push_str(
                    "<script>// Injected by the rust wrapper script — to fix the links
                for (let x of document.querySelectorAll(\"a\")) {
                    if (x.href.includes(\"/unikum/content\")) {
                        x.href = \"https://start.unikum.net\" + x.getAttribute(\"data-file-url\");
                }
                }</script>",
                );
                fs::write(format!("{item}.html"), content).unwrap();
                item += 1;
            }
        }
    } else {
        eprintln!("Not enough arguments");
        exit(1);
    }
}
