use std::{
    env,
    process::{self, Command},
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let music_folder = match args.get(1) {
        Some(s) => s.to_owned(),
        None => {
            let lib = Command::new("xdg-user-dir")
                .arg("MUSIC")
                .output()
                .unwrap_or_else(|err| {
                    eprintln!("Problem parsing arguments: {}", err);
                    process::exit(1)
                });
            String::from_utf8(lib.stdout).expect("XD")
        }
    };

    let mut clear = Command::new("cmus-remote");
    clear.arg("-C").arg("clear").output().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    let mut add = Command::new("cmus-remote");
    add.arg("-C")
        .arg(format!("add {}", music_folder))
        .output()
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1)
        });

    let mut refresh = Command::new("cmus-remote");
    refresh
        .arg("-C")
        .arg("update-cache -f")
        .output()
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1)
        });
}
