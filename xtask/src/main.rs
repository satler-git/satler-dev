use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use chrono::Datelike;

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
        }
    }
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn add(add: flags::Add) -> Result<()> {
    let path_from = format!("./content/blog/{}.md", add.name);
    let path = Path::new(&path_from);

    if path.try_exists()? && !add.force {
        panic!("The file {path_from} exits. To force creation, use -f flag"); // TODO:
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
            now_naive.month0() + 1,
            now_naive.day0() + 1
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

fn main() -> Result<()> {
    let flags = flags::Xtask::from_env_or_exit();

    match flags.subcommand {
        flags::XtaskCmd::Add(add_params) => add(add_params)?,
    }
    Ok(())
}
