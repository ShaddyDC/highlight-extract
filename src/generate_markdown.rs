use crate::model::{Metadata, Section};

pub fn print_markdown(m: Metadata, notes: Vec<Section>) {
    println!("# {}", &m.title);
    println!("");
    println!("**Author:** {}", &m.author);
    println!("\n---\n");

    println!("## Highlights\n");

    for section in notes {
        match section {
            Section::Chapter(c) => println!("## {c}\n"),
            Section::N(highlight) => {
                println!(
                    "### Highlight (Page {}, {})\n",
                    &highlight.page, &highlight.timestamp
                );
                for line in highlight.highlight.lines() {
                    println!("> {line}");
                }
                println!("");

                if let Some(n) = &highlight.note {
                    println!("{n}\n");
                }
            }
        }
    }
}
