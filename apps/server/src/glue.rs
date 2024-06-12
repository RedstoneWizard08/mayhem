use anyhow::Result;
use glue::{config::GlueConfig, framework::Framework, glue::Glue};

// For some reason the `client!` macro doesn't work.
// I'll need to look into that.
#[cfg(debug_assertions)]
mod client {
    use glue::include_dir::Dir;

    pub const CLIENT_DIR: Option<Dir<'static>> = None;
}

#[cfg(not(debug_assertions))]
mod client {
    use glue::include_dir::{self, include_dir, Dir};

    pub const CLIENT_DIR: Option<Dir<'static>> =
        Some(include_dir!("$CARGO_MANIFEST_DIR/../client/build"));
}

// DISCLAIMER: No horses were harmed in the making of this function.
pub fn make_glue() -> Result<Glue> {
    Ok(Glue::new(
        GlueConfig::builder()
            .base("http://localhost:4001")
            .dir(client::CLIENT_DIR)
            .project(format!("{}/../client", env!("CARGO_MANIFEST_DIR")))
            .cmd("pnpm")
            .arg("run")
            .arg("dev")
            .framework(Framework::Vite("/vite-hmr"))
            .build()?,
    ))
}
