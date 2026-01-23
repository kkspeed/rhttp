#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
    
    // Install the default crypto provider for rustls (required for HTTP/3)
    rustls::crypto::ring::default_provider()
        .install_default()
        .ok();
}

