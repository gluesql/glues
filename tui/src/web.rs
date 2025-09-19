#![cfg(target_arch = "wasm32")]

use {
    crate::{App, config, input::Input, logger, theme},
    console_error_panic_hook::set_once as set_panic_hook,
    ratzilla::{DomBackend, WebRenderer, web_sys::console},
    std::rc::Rc,
    tokio::sync::Mutex,
    wasm_bindgen::prelude::*,
    wasm_bindgen_futures::spawn_local,
};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    set_panic_hook();

    spawn_local(async {
        if let Err(err) = run().await {
            console::error_1(&err);
        }
    });

    Ok(())
}

async fn run() -> Result<(), JsValue> {
    config::init().await;
    logger::init().await;
    theme::set_theme(theme::DARK_THEME);

    let backend = DomBackend::new().map_err(to_js_error)?;
    let terminal = ratzilla::ratatui::Terminal::new(backend).map_err(to_js_error)?;

    let app = Rc::new(Mutex::new(App::new()));

    let draw_app = app.clone();
    terminal.on_key_event({
        let app = app.clone();
        move |event| {
            let app = app.clone();
            spawn_local(async move {
                let input = Input::Key(event.into());
                let mut guard = app.lock().await;
                let action = guard.context.consume(&input).await;
                let _ = guard.handle_action(action, input).await;
                guard.save().await;
            });
        }
    });

    terminal.draw_web(move |frame| {
        if let Ok(mut guard) = draw_app.try_lock() {
            guard.draw(frame);
        }
    });

    Ok(())
}

fn to_js_error<E: std::fmt::Display>(err: E) -> JsValue {
    JsValue::from_str(&err.to_string())
}
