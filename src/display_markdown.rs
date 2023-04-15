use std::fmt::Display;

use crate::model::{BooxFile, Section};

pub struct DisplayMarkdown<'a, T: AsMarkdown>(pub &'a T);

impl<T: AsMarkdown> Display for DisplayMarkdown<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt_markdown(f)
    }
}

pub trait AsMarkdown {
    fn fmt_markdown(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result;
}

impl AsMarkdown for BooxFile {
    fn fmt_markdown(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "# {}", &self.metadata.title)?;
        writeln!(f)?;
        writeln!(f, "**Author:** {}", &self.metadata.author)?;
        writeln!(f, "\n---\n")?;

        writeln!(f, "## Highlights\n")?;

        for section in &self.sections {
            match section {
                Section::Chapter(c) => writeln!(f, "### {c}\n")?,
                Section::HL(highlight) => {
                    writeln!(
                        f,
                        "#### Highlight (Page {}, {})\n",
                        &highlight.page, &highlight.timestamp
                    )?;
                    for line in highlight.highlight.lines() {
                        writeln!(f, "> {line}")?;
                    }
                    writeln!(f)?;

                    if let Some(n) = &highlight.note {
                        writeln!(f, "{n}\n")?;
                    }
                }
            }
        }

        Ok(())
    }
}
