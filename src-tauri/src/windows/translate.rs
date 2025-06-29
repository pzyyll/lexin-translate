//! Copyright: 2024 Lizc. All rights reserved.
//! License: MIT License
//! You may obtain a copy of the License at https://opensource.org/licenses/MIT
//!
//! Author: Lizc
//! Created Data: 2024-05-27
//!
//! Description: Translate windows.

use crate::consts;
use kmhook::enginer as mouse_enginer;
use kmhook::types::{ClickState, EventType, MouseButton, Pos};

use lazy_static::lazy_static;
use std::f64;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::Mutex;
use std::sync::{atomic::AtomicUsize, Arc};
use tauri::{
    Emitter, EventTarget, Listener, Manager, PhysicalPosition, PhysicalSize,
    Pixel, Runtime, WebviewWindow,
};
use tauri_plugin_clipboard_manager::ClipboardExt;

lazy_static! {
    static ref MOUSE_EVENT_ID: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    static ref WIN_SIZE: Arc<Mutex<(f64, f64)>> =
        Arc::new(Mutex::new((300.0, 270.0)));
}

#[derive(Debug, Default)]
pub struct TWinState {
    is_pin: std::sync::Mutex<bool>,
}

impl TWinState {
    pub fn reset(&self) {
        *self.is_pin.lock().unwrap() = false;
    }
}

fn record_win_outer_size<R: Runtime>(win: &WebviewWindow<R>) {
    let _ = win.outer_size().inspect(|outer_size| {
        if outer_size.width == 0 && outer_size.height == 0 {
            return;
        }
        let mut size = WIN_SIZE.lock().unwrap();
        *size = (outer_size.width as f64, outer_size.height as f64);
    });
}

fn adjust_win_position<R: Runtime, P: Pixel>(
    win: &WebviewWindow<R>,
    cursor: Option<PhysicalPosition<P>>,
) {
    let cursor: PhysicalPosition<P> = cursor.unwrap_or_else(|| {
        let cursor: PhysicalPosition<f64> =
            win.outer_position().unwrap().cast();
        PhysicalPosition::new(P::from_f64(cursor.x), P::from_f64(cursor.y))
    });

    let (mut x, mut y) = (cursor.x.into(), cursor.y.into());

    let _ = win
        .app_handle()
        .monitor_from_point(cursor.x.into(), cursor.y.into())
        .inspect(|monitor| {
            let m = match monitor {
                Some(m) => m,
                None => return,
            };

            let max_x = m.position().x + m.size().width as i32;
            let max_y = m.position().y + m.size().height as i32;

            let win_size = {
                let size = WIN_SIZE.lock().unwrap();
                PhysicalSize::new(size.0, size.1)
            };

            let win_right_x = x + win_size.width as f64;
            let offset_x = max_x as f64 - win_right_x;
            if offset_x < 0.0 {
                x += offset_x;
            }

            let win_bottom_y = y + win_size.height as f64;
            let offset_y = max_y as f64 - win_bottom_y;
            if offset_y < 0.0 {
                y += offset_y;
            }
            println!("offset_y: {}", offset_y);
        });

    let _ =
        win.set_position(PhysicalPosition::new(P::from_f64(x), P::from_f64(y)));
}

fn emit_on_cpcp<R: Runtime>(win: WebviewWindow<R>) {
    let text = win.app_handle().clipboard().read_text().unwrap();
    let _ = win.emit_to(
        EventTarget::webview_window(consts::WIN_LABEL_TRANSLATE),
        consts::WindowEvent::CPCP.to_string(),
        text,
    );
}

#[tauri::command]
pub async fn open_translate_window(app: tauri::AppHandle<impl Runtime>) {
    let _ = crate::windows::translate::show(&app, None::<fn(_)>);
}

#[tauri::command]
pub async fn set_pin<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    state: tauri::State<'_, TWinState>,
    is_pin: bool,
) -> Result<(), String> {
    let mut ispin = state.is_pin.lock().unwrap();
    *ispin = is_pin;
    Ok(())
}

pub fn try_show_on_double_alt<R: Runtime>(app: &tauri::AppHandle<R>) {
    if let Some(win) = app.get_webview_window(consts::WIN_LABEL_TRANSLATE) {
        set_win_visible(app, &win);
    } else {
        let _ = show(app, None::<fn(_)>);
    }
}

pub fn try_show_on_cpcp<R: Runtime>(app: &tauri::AppHandle<R>) {
    if let Some(win) = app.get_webview_window(consts::WIN_LABEL_TRANSLATE) {
        if let Ok(is_visible) = win.is_visible() {
            if !is_visible {
                set_win_visible(app, &win);
            }
        }
        emit_on_cpcp(win);
    } else {
        let win = show(app, None::<fn(_)>).unwrap();
        let arc_app = std::sync::Arc::new(app.clone());

        win.once("on_ready", move |_event| {
            let result =
                arc_app.get_webview_window(consts::WIN_LABEL_TRANSLATE);
            match result {
                Some(win) => {
                    emit_on_cpcp(win.clone());
                }
                None => {
                    log::error!("window not found");
                }
            }
        });
    }
}

fn set_win_visible<R: Runtime>(
    app: &tauri::AppHandle<R>,
    win: &WebviewWindow<R>,
) {
    let _ = win.show();
    let _ = win.unminimize();
    // let _ = win.set_position(cursor);
    let cursor = app.cursor_position().unwrap();
    adjust_win_position(win, Some(cursor));
    reg_mouse_event(Arc::new(win.clone()));
}

pub fn show<R, F>(
    app: &tauri::AppHandle<R>,
    on_page_load: Option<F>,
) -> Result<WebviewWindow<R>, String>
where
    R: Runtime,
    F: Fn(WebviewWindow<R>) + Send + Sync + 'static,
{
    let cursor = app.cursor_position().unwrap();

    let (x, y) = app
        .monitor_from_point(cursor.x, cursor.y)
        .map(|monitor| {
            monitor.map_or((cursor.x, cursor.y), |m| {
                let logical = cursor.to_logical::<f64>(m.scale_factor());
                (logical.x, logical.y)
            })
        })
        .map_err(|op| op.to_string())?;

    let mut window_builder = tauri::WebviewWindowBuilder::new(
        app,
        consts::WIN_LABEL_TRANSLATE,
        tauri::WebviewUrl::App(consts::WIN_LABEL_TRANSLATE.into()),
    );

    #[cfg(target_os = "windows")]
    {
        window_builder = window_builder.decorations(false);
    }

    let window = window_builder
    // .decorations(false)
    .title(consts::APP_NAME)
    .always_on_top(true)
    .resizable(true)
    .auto_resize()
    .min_inner_size(400.0, 300.0)
    .inner_size(400.0, 300.0)
    .skip_taskbar(true)
    //.transparent
    .position(x.into(), y.into())
    .fullscreen(false)
    .on_page_load(move |window, payload| {
        // println!("page loaded: {:?}", payload);
        use tauri::webview::PageLoadEvent;
        match payload.event() {
            PageLoadEvent::Finished => {
                record_win_outer_size(&window);
                adjust_win_position::<_, f64>(&window, None);
                if let Some(on_page_load) = &on_page_load {
                    on_page_load(window);
                }
            }
            _ => {}
        }
    })
    .build()
    .map_err(|e| e.to_string())?;

    match window.try_state::<TWinState>() {
        Some(state) => {
            state.reset();
        }
        None => {
            window.manage(TWinState::default());
        }
    }

    let arcw = std::sync::Arc::new(window.clone());

    let windowr = arcw.clone();
    window.on_window_event(move |event| {
        // println!("window event: {:?}", event);
        match event {
            tauri::WindowEvent::Resized(size) => {
                record_win_outer_size(&windowr);
                println!("window resized: {:?}", size);
            }
            tauri::WindowEvent::Focused(focused) => {
                if *focused {
                    println!("window focused");
                } else {
                    println!("window unfocused");
                }
            }
            tauri::WindowEvent::CloseRequested { api, .. } => {
                api.prevent_close();
                let _ = windowr.hide();
                // let _ = windowr.minimize();
                del_mouse_event();
            }
            tauri::WindowEvent::Destroyed => {
                // crate::plugin::keyevent::unregister_mouse_event(
                //     crate::consts::MouseEvent::LeftDown,
                // );
                del_mouse_event();
            }
            _ => {}
        }
    });

    reg_mouse_event(arcw);

    Ok(window)

    // #[cfg(target_os = "windows")]
    // {
    //     // use window_vibrancy::apply_mica;
    //     // apply_mica(&window, true.into()).unwrap();
    //     use tauri::window::{Color, Effect, EffectState, EffectsBuilder};
    //     window.set_effects(
    //         EffectsBuilder::new()
    //             .color(Color(18, 18, 18, 18))
    //             .effect(Effect::TabbedLight)
    //             .state(EffectState::Active)
    //             .build()
    //     ).unwrap();
    // }
}

fn check_pos_in_window<R: Runtime>(
    window: &WebviewWindow<R>,
    pos: &Pos,
) -> bool {
    if let (Ok(position), Ok(size)) =
        (window.outer_position(), window.outer_size())
    {
        let (xmin, ymin) = (position.x, position.y);
        let (xmax, ymax) =
            (xmin + size.width as i32, ymin + size.height as i32);
        let pt = pos;
        if pt.x < xmin || pt.x > xmax || pt.y < ymin || pt.y > ymax {
            return false;
        }
    }
    true
}

fn reg_mouse_event<R: Runtime>(window: Arc<WebviewWindow<R>>) {
    if MOUSE_EVENT_ID.load(Relaxed) != 0 {
        return;
    }
    let once_focus = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let reg_id = mouse_enginer::add_event_listener(
        move |event_type| {
            if let EventType::MouseEvent(Some(event)) = event_type {
                // println!("mouse event: {:?}", event);
                if let Some(MouseButton::Left(state)) = event.button {
                    if state == ClickState::Released {
                        return;
                    }
                    let pin_state: tauri::State<TWinState> = window.state();
                    if *pin_state.is_pin.lock().unwrap() {
                        return;
                    }
                    if check_pos_in_window(&window, &event.pos) {
                        if state == ClickState::Pressed {
                            once_focus.store(true, Relaxed);
                        }
                        return;
                    }
                    println!("close when mouse left button out");
                    let _ = window.close();
                } else if event.button.is_none() {
                    // is mouse move
                    let pin_state: tauri::State<TWinState> = window.state();
                    if check_pos_in_window(&window, &event.pos)
                        | *pin_state.is_pin.lock().unwrap()
                        | once_focus.load(Relaxed)
                    {
                        return;
                    }
                    println!("close when mouse move out");
                    let _ = window.close();
                } else {
                    // println!("mouse event: {:?}", event);
                }
            }
        },
        Some(EventType::MouseEvent(None)),
    );
    if let Ok(id) = reg_id {
        MOUSE_EVENT_ID.store(id, Relaxed);
    }
}

fn del_mouse_event() {
    if MOUSE_EVENT_ID.load(Relaxed) == 0 {
        return;
    }
    // println!("del_mouse_event {:?}", MOUSE_EVENT_ID.load(Relaxed));
    mouse_enginer::del_event_by_id(MOUSE_EVENT_ID.load(Relaxed));
    MOUSE_EVENT_ID.store(0, Relaxed);
}
