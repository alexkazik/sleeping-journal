use std::path::PathBuf;
use yew_bootstrap::icons::BIFiles;

fn main() -> Result<(), std::io::Error> {
    let staging_dir = PathBuf::from(
        std::env::var("TRUNK_STAGING_DIR").expect("Environment variable TRUNK_STAGING_DIR"),
    );

    // copy bootstrap icons
    let path = staging_dir.join(BIFiles::NAME);
    if !path.is_dir() {
        std::fs::create_dir(&path)?;
    }
    BIFiles::copy(&path)?;

    // update the index.html
    let path = staging_dir.join("index.html");
    let index = std::fs::read_to_string(&path)?;
    let index = index
        .replace(
            "<!include-bootstrap-icons>",
            r#"<link rel="stylesheet" href="bootstrap-icons-v1.11.3/bootstrap-icons.css"/>"#,
        )
        .replace("<!version>", env!("CARGO_PKG_VERSION"));
    std::fs::write(&path, index)?;

    Ok(())
}
