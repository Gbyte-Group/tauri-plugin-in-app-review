use tauri::{command, AppHandle, Runtime};

use crate::InAppReviewExt;
use crate::Result;

#[command]
pub(crate) async fn request_review<R: Runtime>(app: AppHandle<R>) -> Result<()> {
    // println!("Rust: request_review command called");
    #[cfg(mobile)]
    {
        // println!("Rust: calling mobile request_review");
        return app.in_app_review().request_review();
    }
    #[cfg(not(mobile))]
    {
        // println!("Rust: unsupported platform");
        return Err(crate::Error::UnsupportedPlatform);
    }
}
