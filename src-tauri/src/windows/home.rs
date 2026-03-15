//! Copyright: 2024 Lizc. All rights reserved.
//! License: GNU GPL v3 or later
//! You may obtain a copy of the License at https://www.gnu.org/licenses/gpl-3.0.html
//!
//! Author: Lizc
//! Created Data: 2024-05-27
//!
//! Description: The home window module for the application.

use crate::consts;
use tauri::{Manager, Runtime, WebviewWindow, WebviewWindowBuilder};
use tauri_plugin_decorum::WebviewWindowExt;
use tauri_plugin_window_state::{StateFlags, WindowExt};

#[allow(unused)]
pub fn show<R: Runtime>(app: &tauri::AppHandle<R>)
where
    WebviewWindow<R>: WebviewWindowExt,
{
    match app.get_webview_window(consts::WIN_LABEL_HOME) {
        Some(win) => {
            win.show().expect("failed to show window");
            if win.is_minimized().unwrap_or(false) {
                win.unminimize().expect("failed to unminimize window");
            }
            win.set_focus();
        }
        None => {
            let url = tauri::WebviewUrl::App(consts::WIN_LABEL_HOME.into());
            println!("Creating home window with URL: {:?}", url.to_string());

            let mut web_build = WebviewWindowBuilder::new(app, consts::WIN_LABEL_HOME, url);

            #[cfg(windows)]
            {
                // Enable overlay titlebar on Windows
                web_build = web_build.decorations(false)
            }

            let win = web_build
                .resizable(true)
                .fullscreen(false)
                .title(consts::APP_NAME)
                .inner_size(600.0, 500.0)
                .min_inner_size(600.0, 500.0)
                .disable_drag_drop_handler()
                .visible(false)
                .center()
                .build()
                .unwrap();
            win.create_overlay_titlebar()
                .expect("failed to create overlay titlebar");

            win.restore_state(StateFlags::POSITION | StateFlags::SIZE)
                .expect("failed to restore window state");
        }
    }
}
