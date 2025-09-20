#![cfg(target_arch = "wasm32")]

use {
    crate::{
        App, config,
        input::{Input, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
        logger, theme,
    },
    console_error_panic_hook::set_once as set_panic_hook,
    ratzilla::{DomBackend, WebRenderer, web_sys::console},
    std::rc::Rc,
    tokio::sync::Mutex,
    wasm_bindgen::closure::Closure,
    wasm_bindgen::{JsCast, prelude::*},
    wasm_bindgen_futures::spawn_local,
    web_sys::{self, CompositionEvent, Event, HtmlTextAreaElement, InputEvent},
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
    let ime = init_ime_field()?;

    let draw_app = app.clone();
    let ime_for_keys = ime.clone();
    terminal.on_key_event({
        let app = app.clone();
        move |event| {
            if !event.ctrl && !event.alt {
                if matches!(event.code, ratzilla::event::KeyCode::Char(_)) {
                    return;
                }
            }

            if let Err(err) = ime_for_keys.focus() {
                console::error_1(&err);
            }

            let app = app.clone();
            let key_event = event.into();
            spawn_local(async move {
                let input = Input::Key(key_event);
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

    attach_ime_listeners(app.clone(), ime);

    Ok(())
}

fn to_js_error<E: std::fmt::Display>(err: E) -> JsValue {
    JsValue::from_str(&err.to_string())
}

fn init_ime_field() -> Result<Rc<HtmlTextAreaElement>, JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("missing window"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("missing document"))?;

    let textarea = document
        .create_element("textarea")?
        .dyn_into::<HtmlTextAreaElement>()?;

    textarea.set_attribute("autocomplete", "off")?;
    textarea.set_attribute("autocorrect", "off")?;
    textarea.set_attribute("autocapitalize", "none")?;
    textarea.set_attribute("spellcheck", "false")?;
    textarea.set_attribute("aria-hidden", "true")?;
    textarea.set_class_name("glues-ime");

    let style = textarea.style();
    let _ = style.set_property("position", "fixed");
    let _ = style.set_property("top", "0");
    let _ = style.set_property("left", "0");
    let _ = style.set_property("width", "100vw");
    let _ = style.set_property("height", "100vh");
    let _ = style.set_property("opacity", "0");
    let _ = style.set_property("background", "transparent");
    let _ = style.set_property("color", "transparent");
    let _ = style.set_property("caret-color", "transparent");
    let _ = style.set_property("border", "0");
    let _ = style.set_property("resize", "none");
    let _ = style.set_property("pointer-events", "none");
    let _ = style.set_property("z-index", "10000");

    textarea.set_value("");

    if let Some(body) = document.body() {
        let _ = body.append_child(&textarea);
    }

    if let Err(err) = textarea.focus() {
        console::error_1(&err);
    }

    Ok(Rc::new(textarea))
}

fn attach_ime_listeners(app: Rc<Mutex<App>>, ime: Rc<HtmlTextAreaElement>) {
    ime.set_value("");

    let ime_for_input = ime.clone();
    let app_for_input = app.clone();
    let handle_input = Closure::<dyn FnMut(InputEvent)>::new(move |event: InputEvent| {
        event.prevent_default();

        if event.is_composing() {
            return;
        }

        let data = event.data().unwrap_or_default();
        if !data.is_empty() {
            dispatch_text(app_for_input.clone(), data);
        } else {
            let input_type = event.input_type();
            match input_type.as_str() {
                "deleteContentBackward" => dispatch_key(app_for_input.clone(), KeyCode::Backspace),
                "deleteContentForward" => dispatch_key(app_for_input.clone(), KeyCode::Delete),
                _ => {}
            }
        }

        ime_for_input.set_value("");
    });

    if let Err(err) =
        ime.add_event_listener_with_callback("input", handle_input.as_ref().unchecked_ref())
    {
        console::error_1(&err);
    }
    handle_input.forget();

    if let Some(window) = web_sys::window() {
        let ime_focus = ime.clone();
        let handle_mouse = Closure::<dyn FnMut(Event)>::new(move |ev: Event| {
            if let Err(err) = ime_focus.focus() {
                console::error_1(&err);
            }
            ev.prevent_default();
        });

        if let Err(err) = window
            .add_event_listener_with_callback("mousedown", handle_mouse.as_ref().unchecked_ref())
        {
            console::error_1(&err);
        }
        handle_mouse.forget();
    }

    let ime_for_composition = ime.clone();
    let app_for_composition = app.clone();
    let handle_composition =
        Closure::<dyn FnMut(CompositionEvent)>::new(move |event: CompositionEvent| {
            let data = event.data().unwrap_or_else(|| ime_for_composition.value());
            if !data.is_empty() {
                dispatch_text(app_for_composition.clone(), data);
            }
            ime_for_composition.set_value("");
        });

    if let Err(err) = ime.add_event_listener_with_callback(
        "compositionend",
        handle_composition.as_ref().unchecked_ref(),
    ) {
        console::error_1(&err);
    }
    handle_composition.forget();
}

fn dispatch_text(app: Rc<Mutex<App>>, text: String) {
    if text.is_empty() {
        return;
    }

    spawn_local(async move {
        let mut guard = app.lock().await;
        let mut should_save = false;

        for ch in text.chars() {
            let key_event = KeyEvent {
                code: KeyCode::Char(ch),
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
            };
            let input = Input::Key(key_event);
            let action = guard.context.consume(&input).await;
            let quit = guard.handle_action(action, input).await;
            should_save = true;
            if quit {
                break;
            }
        }

        if should_save {
            guard.save().await;
        }
    });
}

fn dispatch_key(app: Rc<Mutex<App>>, key_code: KeyCode) {
    spawn_local(async move {
        let key_event = KeyEvent {
            code: key_code,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
        };

        let input = Input::Key(key_event);
        let mut guard = app.lock().await;
        let action = guard.context.consume(&input).await;
        let _ = guard.handle_action(action, input).await;
        guard.save().await;
    });
}
