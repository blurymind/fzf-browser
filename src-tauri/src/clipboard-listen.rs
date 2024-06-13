extern crate copypasta;
use std::env;
use copypasta::{ ClipboardContext, ClipboardProvider };
use std::process::{ Command, Stdio };

fn main() {
    let _args: Vec<String> = env::args().collect();
    let query = _args.get(1);

    let mut ctx = ClipboardContext::new().unwrap();
    let mut prev_clip = String::from("");
    loop {
        let content = ctx.get_contents().unwrap().to_string();
        if prev_clip != content {
            println!("Searching for {:?}", query);
            println!("{}", content);

            let output = Command::new("cat")
                .arg("Cargo.toml")
                // Tell the OS to record the command's output
                .stdout(Stdio::piped())
                // execute the command, wait for it to complete, then capture the output
                .output()
                // Blow up if the OS was unable to start the program
                .unwrap();

            // extract the raw bytes that we captured and interpret them as a string
            let stdout = String::from_utf8(output.stdout).unwrap();
            println!("{}", stdout);
        }
        prev_clip = content;
    }
}
