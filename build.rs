#![allow(missing_docs)]

use rhusky::Rhusky;

fn main() {
    Rhusky::new()
        .hooks_dir(".githooks")
        .skip_in_env("CI")
        .skip_in_env("GITHUB_ACTIONS")
        .with_default_hooks()
        .install_from_build_script()
        .expect("failed to install repository Git hooks");
}
