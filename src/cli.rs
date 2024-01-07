use clap::{arg, command, crate_authors, crate_version, crate_description};

pub fn cli() -> clap::Command {
    command!()
        .author(crate_authors!(""))
        .version(crate_version!())
        .about(crate_description!())
        .help_template(r"
{before-help}{name} (v{version})
{author}
{about}

{usage-heading} {usage}

{all-args}{after-help}
")
        .args( [
            arg!(<REGEX> "Sets the regex pattern to search for (string)").group("args").required(true),
            arg!(<SOURCE> "Sets the source to search from (string/url/file)").group("args").required(true),
            ])

        .group(clap::ArgGroup::new("flags").multiple(true))
        .next_help_heading("Flags")
        .args( [
            arg!(-i --inverse "The regex is used as exclusion pattern").group("flags"),
            arg!(-c --count "Count the number of matches").group("flags"),
            arg!(-l --list "List the files that match the pattern").group("flags"),
            arg!(-n --linenumber "Print the line number of the match").group("flags"),
            arg!(-v --verbose "Print test information verbosely").group("flags"),
        ])
        
        .group(clap::ArgGroup::new("output").multiple(true))
        .next_help_heading("Output")
        .args( [
            arg!(-F --format <FORMAT> "Sets the output format to use").group("output").value_name("FORMAT").value_parser(["text", "json", "xml", "yaml", "csv"]).default_value("text"),
            arg!(-D --delimiter <DELIM> "Sets the delimiter to use").group("output").default_value(","),
            arg!(-Q --quote <QUOTE> "Sets the quote character to use").group("output").default_value("\""),
            arg!(-S --separator <SEP> "Sets the separator to use").group("output").default_value(":"),
            arg!(-N --newline <NEWLINE> "Sets the newline character to use").group("output"),
            arg!(-R --replace <PATTERN> "Sets the replacement pattern to use").group("output"),
        ])
}