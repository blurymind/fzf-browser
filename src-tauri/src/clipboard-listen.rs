extern crate copypasta;
use std::env;
use copypasta::{ ClipboardContext, ClipboardProvider };
use std::process::{ Command, Stdio };

fn main() {
    let _args: Vec<String> = env::args().collect();
    let query = _args.get(1);

    let mut ctx = ClipboardContext::new().unwrap();
    let mut prev_clip = String::from("");

    let mut content = ctx.get_contents().unwrap().to_string();
    println!("search for:: {}", content);

    // todo here we need to kill the command when clipboard changes
    // let mut fzf = Command::new("fzf")
    // // .stdin(Stdio::piped()) // pipe into
    // .stdout(Stdio::piped())
    // .args([
    //     "-m", "--bind", "ctrl-a:select-all,ctrl-d:deselect-all,ctrl-t:toggle-all",
    //     "--query", &content
    //     ])
    // .spawn().expect("--- fails to spawn!! --");
    // let child_id = fzf.id();

    // let processOutput = fzf.wait_with_output().expect("fail2");//wait with output prevents future killing
    
    // let result = String::from_utf8(processOutput.stdout);
    // println!("RESULT:: {:?}", result.unwrap());// todo here fzf only outputs the selected item, we need it to output filtered items

    loop {
        content = ctx.get_contents().unwrap().to_string();
        if prev_clip != content {
            println!("Clipboard changed:: {:?}", content);

           // TODO kill and rerun fzf
           // https://crates.io/crates/shared_child
            // &fzf.kill();
            let mut fzf = Command::new("fzf")
            // .stdin(Stdio::piped()) // pipe into
            .stdout(Stdio::piped())
            .args([
                "-m", "--bind", "ctrl-a:select-all,ctrl-d:deselect-all,ctrl-t:toggle-all",
                "--query", &content
                ])
            .spawn().expect("--- fails to spawn!! --");
            let child_id = fzf.id();
        
            let processOutput = fzf.wait_with_output().expect("fail2");//wait with output prevents future killing
            
            let result = String::from_utf8(processOutput.stdout);
            println!("RESULT:: {:?}", result.unwrap());// todo here fzf only outputs the selected item, we need it to output filtered items

            
            // println!("{}", content);
            // let output = Command::new("cat")
            //     .arg("Cargo.toml")
            //     // Tell the OS to record the command's output
            //     .stdout(Stdio::piped())
            //     // execute the command, wait for it to complete, then capture the output
            //     .output()
            //     // Blow up if the OS was unable to start the program
            //     .unwrap();
            // // extract the raw bytes that we captured and interpret them as a string
            // let textFileContent = String::from_utf8(output.stdout).unwrap();

        }
        prev_clip = content;
    }
}
