use clap::clap_app;

fn main() {
    let _matches = clap_app!(fsling =>
        (version: "0.0.1")
        (author: "Ellie Huxtable <e@elm.sh>")
        (about: "fsling - File Sling")
        (@arg CONFIG: -c --config +takes_value "Sets a custom config file")
        (@arg DIR: +required "Set the directory to watch")
    ).get_matches();
}
