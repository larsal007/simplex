use clap::{arg, command};

pub fn cli() -> clap::Command {
    command!()
        .group(clap::ArgGroup::new("search").multiple(false))
        .next_help_heading("Pattern")
        .args( [
            arg!(-r --regex <PATTERN> "Sets the regex pattern to search for").group("search").required(true),
            ])
        // TODO: make call as simplex <regex> <source>  
        .group(clap::ArgGroup::new("inputs").multiple(false))
        .next_help_heading("Inputs")
        .args( [
            arg!(-f --file <FILE> "Sets the text file to search in").group("inputs"),
            arg!(-u --url <URL> "Sets the URL to use").group("inputs"),
            arg!(-s --urlfile <FILE> "Sets the source file of URLs to use").group("inputs"),
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
            // arg!(-O --output <FILE> "Sets the output file").group("output"),
            arg!(-F --format <FORMAT> "Sets the output format to use").group("output").value_name("FORMAT").value_parser(["text", "json", "xml", "yaml", "csv"]).default_value("text"),
            arg!(-D --delimiter <DELIM> "Sets the delimiter to use").group("output").default_value(","),
            arg!(-Q --quote <QUOTE> "Sets the quote character to use").group("output").default_value("\""),
            arg!(-S --separator <SEP> "Sets the separator to use").group("output").default_value(":"),
            arg!(-N --newline <NEWLINE> "Sets the newline character to use").group("output"),
            arg!(-R --replace <PATTERN> "Sets the replacement pattern to use").group("output"),
        ])
}