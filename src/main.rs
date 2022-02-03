#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

#[allow(unused_imports)]
use ::{
    anyhow::{
        anyhow, bail, Error, Result,
    },
    std::{
        fs,
        io::{Read as _, Write as _},
        mem,
        ops::Not as _,
        path::{Path, PathBuf},
    },
};

/// Current workaround to write "comments" that are part of the AST and thus,
/// preserved. Important when testing this tool in a boostrapping fashion.
#[doc(hidden)]
macro_rules! rem {( $($_:tt)* ) => ()}

fn main ()
  -> Result<()>
{
    let ref mut args = ::std::env::args().peekable();
    let ref binary_name = args.next().expect("Missing argv[0]");
    let check_only = match args.peek().map(String::as_str) {
        | Some("--help" | "-h") | None => help(binary_name),
        | Some("--check") => {
            drop(args.next());
            true
        },
        | Some(_thing_else) => false,
    };

    let mut found_diff = false;
    rem! {"TODO: walkdir + ideally in parallel."}
    for ref fname in args.map(PathBuf::from) {
        let ref contents = fs::read_to_string(fname)?;
        let ref ast_contents = ::syn::parse_file(contents)?;
        let ref formatted_code = ::prettyplease::unparse(ast_contents);
        if contents != formatted_code {
            if check_only {
                found_diff = true;
                eprintln!(
                    "\n\n\t>>> Ill-formatted file: `{}` <<<\n",
                    fname.display(),
                );
                #[cfg(feature = "check-shows-diffs")]
                eprintln!(
                    "{}",
                    ::prettydiff::diff_lines(contents, formatted_code),
                );
            } else {
                fs::write(fname, formatted_code)?;
            }
        }
    }
    if found_diff && check_only {
        bail!("Found ill-formatted code.");
    }
    Ok(())
}

#[doc(hidden)]
fn help (binary_name: &'_ str)
  -> !
{
    eprintln! {"\
Unofficial CLI wrapper around `::prettyplease::unparse` to format files and \
trim comments.

This modifies them IN PLACE, unless the `--check` flag is passed.

USAGE:
    {binary_name} [OPTION] <files…>

OPTIONS:
        --check     Runs in 'check' mode. Exits with 0 if input is formatted
                    correctly. Exits with 1 if formatting is required, printing
                    a diff unless the `check-shows-diffs` Cargo feature were
                    disabled at compile-time.
    -h, --help      Shows this very message.

See https://crates.io/crates/prettyplease for more info about the formatting \
itself.\
    "}
    ::std::process::exit(-1);
}
