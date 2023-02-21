use self_update::cargo_crate_version;

fn main() {
    println!("Hello, world {}!", cargo_crate_version!());
    if let Err(err) = update() {
        println!("Update failed: {:?}", err);
    }
}

fn update() -> Result<(), Box<dyn std::error::Error>> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("FredrikNoren")
        .repo_name("release_test")
        .bin_name("github")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;
    println!("Update status: `{}`!", status.version());
    Ok(())
}
