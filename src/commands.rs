use tauri::{command, AppHandle, Runtime};

use crate::InAppReviewExt;
use crate::Result;

#[command]
pub(crate) async fn request_review<R: Runtime>(app: AppHandle<R>) -> Result<()> {
    return app.in_app_review().request_review();
}
