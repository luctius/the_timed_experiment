use structopt::{clap::AppSettings, StructOpt};

/// The Timed Experiment <Terminal Edition>
///
#[derive(StructOpt, Debug)]
#[structopt(verbatim_doc_comment, name = "tte", global_settings = &[AppSettings::ColoredHelp, AppSettings::ArgRequiredElseHelp])]
struct Opt {
    /// Activate debug mode
    #[structopt(short = "d", long)]
    debug: bool,

    /// Show the available types, the exit
    #[structopt(short = "s", long)]
    show_types: bool,

    /// Use the Default Random number to convert an Duid to an Muid
    #[structopt(short = "r", long)]
    with_default_random: bool,
}

fn main() {
    let opt = Opt::from_args();

    if opt.debug {
        println!("{:#?}", opt);
    }
}
