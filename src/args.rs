#[derive(StructOpt, Debug)]
#[structopt(name = "raven")]
pub enum Raven {
    #[structopt(name = "load", about = "Load a complete theme")]
    Load {
        theme: String,
        #[structopt(flatten)]
        verbose: clap_verbosity_flag::Verbosity,
    },
    #[structopt(name = "new", about = "Create a new theme")]
    New {
        name: String,
        #[structopt(flatten)]
        verbose: clap_verbosity_flag::Verbosity,
    },
    #[structopt(
        name = "modify",
        about = "Open the currently edited themes's option in $EDITOR"
    )]
    Modify {
        /// Use custom editor
        #[structopt(short = "e", long = "editor")]
        editor: Option<String>,
        name: String,
        #[structopt(flatten)]
        verbose: clap_verbosity_flag::Verbosity,
    },
    #[structopt(name = "delete", about = "Delete a theme")]
    Delete {
        name: String,
        #[structopt(flatten)]
        verbose: clap_verbosity_flag::Verbosity,
    },
    #[structopt(
        name = "info",
        about = "Print info about the theme being currently edited"
    )]
    Info {
        #[structopt(flatten)]
        verbose: clap_verbosity_flag::Verbosity,
    },
    #[structopt(name = "refresh", about = "Load last loaded theme")]
    Refresh {
        #[structopt(flatten)]
        verbose: clap_verbosity_flag::Verbosity,
    },
    #[structopt(name = "install", about = "Install a theme from ThemeHub repo")]
    Install {
        name: String,
        /// Don't prompt for confirmation
        #[structopt(short = "f", long = "force")]
        force: bool,
        #[structopt(flatten)]
        verbose: clap_verbosity_flag::Verbosity,
    },
    #[structopt(name = "add", about = "Add option to current theme")]
    Add {
        option: String,
        name: String,
        #[structopt(flatten)]
        verbose: clap_verbosity_flag::Verbosity,
    },
    #[structopt(name = "rm", about = "Remove an option from edited theme")]
    Rm {
        name: String,
        #[structopt(flatten)]
        verbose: clap_verbosity_flag::Verbosity,
    },
    #[structopt(name = "key", about = "Add a key-value option")]
    Key {
        key: String,
        value: String,
        #[structopt(flatten)]
        verbose: clap_verbosity_flag::Verbosity,
    },
    #[structopt(name = "edit", about = "Edit theme")]
    Edit {
        name: String,
        #[structopt(flatten)]
        verbose: clap_verbosity_flag::Verbosity,
    },
    #[structopt(name = "menu", about = "Show theme menu")]
    Menu {
        #[structopt(flatten)]
        verbose: clap_verbosity_flag::Verbosity,
    },
    #[structopt(name = "manage", about = "Manage online themes")]
    ManageO(Manage),
    #[structopt(name = "cycle", about = "Control cycle daemon")]
    CycleD(Cycle),
}
#[derive(StructOpt, Debug)]
pub enum Manage {
    #[structopt(name = "export", about = "Export a theme to a tarball")]
    Export {
        name: String,
        #[structopt(flatten)]
        verbose: clap_verbosity_flag::Verbosity,
    },
    #[structopt(name = "import", about = "Import a theme from a tarball")]
    Import {
        name: String,
        #[structopt(flatten)]
        verbose: clap_verbosity_flag::Verbosity,
    },
    #[structopt(name = "publish", about = "Publish an account online")]
    Publish {
        name: String,
        #[structopt(flatten)]
        verbose: clap_verbosity_flag::Verbosity,
    },
    #[structopt(name = "create", about = "Create an account")]
    Create {
        name: String,
        pass1: String,
        pass2: String,
        #[structopt(flatten)]
        verbose: clap_verbosity_flag::Verbosity,
    },
    #[structopt(
        name = "delete_user",
        about = "Delete an online user's profile and owned themes"
    )]
    DUser {
        pass: String,
        #[structopt(flatten)]
        verbose: clap_verbosity_flag::Verbosity,
    },
    #[structopt(name = "logout", about = "Log out of your user profile")]
    Logout {
        #[structopt(flatten)]
        verbose: clap_verbosity_flag::Verbosity,
    },
    #[structopt(name = "unpublish", about = "Delete an online theme")]
    Unpublish {
        name: String,
        #[structopt(flatten)]
        verbose: clap_verbosity_flag::Verbosity,
    },
    #[structopt(name = "login", about = "Log in to an user's account")]
    Login {
        name: String,
        pass: String,
        #[structopt(flatten)]
        verbose: clap_verbosity_flag::Verbosity,
    },
}
#[derive(StructOpt, Debug)]
pub enum Cycle {
    #[structopt(name = "start", about = "Start the daemon")]
    Start {
        #[structopt(flatten)]
        verbose: clap_verbosity_flag::Verbosity,
    },
    #[structopt(name = "check", about = "Check if daemon is running")]
    Check {
        #[structopt(flatten)]
        verbose: clap_verbosity_flag::Verbosity,
    },
    #[structopt(name = "stop", about = "Stop the daemon")]
    Stop {
        #[structopt(flatten)]
        verbose: clap_verbosity_flag::Verbosity,
    },
}
