#[allow(unused_imports)]
#[cfg(not(feature = "bb_js"))]
#[cfg(test)]
mod tests {
    // Some of these imports are consumed by the injected tests
    use std::collections::BTreeMap;
    use std::fs;
    use std::path::{Path, PathBuf};

    use super::*;

    // include tests generated by `build.rs`
    include!(concat!(env!("OUT_DIR"), "/prove_and_verify.rs"));
}
