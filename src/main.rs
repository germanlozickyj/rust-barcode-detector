#[macro_use] extern crate rocket;

use rocket::form::Form;
use rocket::fs::TempFile;
use tokio::fs;
use std::os::unix::fs::PermissionsExt;

#[derive(FromForm)]
struct Upload<'r> {
    file: TempFile<'r>,
}

#[post("/format", data = "<upload>")]
async fn submit(upload: Form<Upload<'_>>) -> std::io::Result<()> {
    let temp_path = upload.file.path().expect("No temp path");

    let target_path = "storage/file.png";

    fs::copy(temp_path, target_path).await?;

    let metadata = std::fs::metadata(target_path)?;
    let mut permissions = metadata.permissions();
    
    permissions.set_mode(0o444);

    std::fs::set_permissions(target_path, permissions)?;
    
    Ok(())
}

#[launch] 
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![submit])
}