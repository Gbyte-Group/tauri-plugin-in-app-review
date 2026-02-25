use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_in_app_review);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<InAppReview<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("com.plugin.inappreview", "InAppReviewPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_in_app_review)?;
    Ok(InAppReview(handle))
}

/// Access to the in-app-review APIs.
pub struct InAppReview<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> InAppReview<R> {
    pub fn request_review(&self) -> crate::Result<()> {
        // println!("Rust: InAppReview::request_review called");
        #[cfg(target_os = "android")]
        {
            // println!("Rust: calling android run_mobile_plugin");
            return self.0.run_mobile_plugin("requestReview", ()).map_err(|e| {
                // println!("Rust: android error: {:?}", e);
                e.into()
            });
        }
        #[cfg(target_os = "ios")]
        {
            // println!("Rust: calling ios run_mobile_plugin");
            return self.0.run_mobile_plugin("requestReview", ()).map_err(|e| {
                // println!("Rust: ios error: {:?}", e);
                e.into()
            });
        }
    }
}
