use ::{
    core::{
        ops::Not as _,
    },
    std::{
        process::Command,
    },
};

#[test]
fn fmt_self ()
{
    Command::new(env!("CARGO_BIN_EXE_prettyplease-fmt"))
        .arg("src/main.rs")
        .status()
            .unwrap()
        .success()
            .not().then(|| panic!())
    ;
    Command::new(env!("CARGO"))
        .args([
            "test", // "-q",
            "--test", "bootstrap",
            "--no-default-features",
            "--",
            "--ignored",
            "--exact", "check",
        ])
        .status()
            .unwrap()
        .success()
            .not().then(|| panic!())
    ;
}

#[test]
#[ignore]
fn check ()
{
    Command::new(env!("CARGO_BIN_EXE_prettyplease-fmt"))
        .args(["--check", "src/main.rs"])
        .status()
            .unwrap()
        .success()
            .not().then(|| panic!())
    ;
}
