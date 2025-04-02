use std::io::prelude::*;
use std::path::Path;
use std::{fs::File, process::Command};

use chrono::Datelike;

use anyhow::{bail, Result};

mod flags {
    xflags::xflags! {
        cmd xtask {
            /// add new blog post
            cmd add {
                /// name of markdown file
                ///
                /// this will create file at ./content/blog/hoge.md
                ///     add --name hoge
                required --name name: String
                /// force file creation
                /// if the file exits, it will be replaced
                optional -f, --force
            }
            /// add new daily post if not exits
            cmd daily {
                /// force file creation
                /// if the file exits, it will be replaced
                optional -f, --force
            }
        }
    }
}

fn add(params: flags::Add) -> Result<()> {
    let path_from = format!("./content/blog/{}.md", params.name);
    let path = Path::new(&path_from);

    if path.try_exists()? && !params.force {
        bail!("The file {path_from} exits. To force creation, use --force flag.");
    }

    let date = {
        use chrono::{FixedOffset, Utc};
        let now_naive = {
            let now_utc = Utc::now();
            let now = now_utc.with_timezone(&FixedOffset::east_opt(9 * 3600).unwrap());
            now.date_naive()
        };

        format!(
            "{:04}-{:02}-{:02}",
            now_naive.year(),
            now_naive.month(),
            now_naive.day()
        )
    };

    let content = format!(
        "+++
title = \"Template\"
date = {date}
description = \"About blog post\"

[extra]
comment = true

[taxonomies]
tags = [\"tag1\", \"tag2\"]
+++
"
    );

    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;

    println!("The file was created at {path_from}");

    Ok(())
}

fn daily(params: flags::Daily) -> Result<()> {
    let date = {
        use chrono::{FixedOffset, Utc};
        let now_naive = {
            let now_utc = Utc::now();
            let now = now_utc.with_timezone(&FixedOffset::east_opt(9 * 3600).unwrap());
            now.date_naive()
        };

        format!(
            "{:04}-{:02}-{:02}",
            now_naive.year(),
            now_naive.month(),
            now_naive.day()
        )
    };

    let path_from = format!("./content/daily/{date}.md");
    let path = Path::new(&path_from);

    if !path.try_exists()? || params.force {
        let content = format!(
            "+++
title = \"{date}の日報\"
date = {date}
description = \"{date}の日報(?)です。実験中\"

[extra]
comment = true

[taxonomies]
tags = [\"daily\"]
+++
"
        );

        let mut file = File::create(path)?;
        file.write_all(content.as_bytes())?;

        println!("The file was created at {path_from}");
    } else {
        println!("The file {path_from} exits. To force creation, use --force flag.")
    }

    Command::new("jj")
        .arg("desc")
        .arg("-m")
        .arg(format!("daily: {date}"))
        .status()?;

    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());

    Command::new(editor).arg(path).status()?;

    Ok(())
}

fn main() -> Result<()> {
    let flags = flags::Xtask::from_env_or_exit();

    match flags.subcommand {
        flags::XtaskCmd::Add(params) => add(params)?,
        flags::XtaskCmd::Daily(params) => daily(params)?,
    }
    Ok(())
}
