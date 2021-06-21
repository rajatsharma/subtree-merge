use std::convert::TryInto;
use std::env;
use std::process::Command;

use common_rs::Result;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    for arg in args {
        let [repo, branch] = if arg.contains('@') {
            arg.split('@')
                .collect::<Vec<&str>>()
                .try_into()
                .expect("Cannot convert to array")
        } else {
            ["master", &*arg]
        };

        let output = Command::new("git")
            .args(&[
                "subtree",
                "add",
                &*format!("--prefix={}", repo),
                &*format!("https://github.com/rajatsharma/{}", repo),
                branch,
            ])
            .output()?;

        println!("{}", String::from_utf8(output.stdout)?);
        println!("{}", String::from_utf8(output.stderr)?);
    }

    Ok(())
}
