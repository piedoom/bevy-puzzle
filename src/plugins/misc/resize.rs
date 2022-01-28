// Adapted from:
//
// MIT License

// Copyright (c) 2020 John Peel

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use bevy::prelude::*;
use futures::channel::mpsc;
use gloo_events;
use wasm_bindgen::prelude::*;

#[derive(Debug)]
struct ViewportResized {
    width: f32,
    height: f32,
}

#[derive(Debug)]
struct ViewportState {
    receiver: mpsc::UnboundedReceiver<ViewportResized>,
}

pub struct ViewportResizedPlugin;

impl From<(f32, f32)> for ViewportResized {
    fn from((width, height): (f32, f32)) -> Self {
        ViewportResized { width, height }
    }
}

fn get_viewport_size() -> (f32, f32) {
    let window = web_sys::window().expect("could not get window");
    let document_element = window
        .document()
        .expect("could not get document")
        .document_element()
        .expect("could not get document element");

    (
        document_element.client_width() as f32,
        document_element.client_height() as f32,
    )
}

fn resized_event_system(mut windows: ResMut<Windows>, mut state: ResMut<ViewportState>) {
    if let Ok(Some(event)) = state.receiver.try_next() {
        if let Some(window) = windows.get_primary_mut() {
            window.set_resolution(event.width, event.height);
        }
    }
}

fn initial_size_system(mut windows: ResMut<Windows>) {
    let (width, height) = get_viewport_size();
    if let Some(window) = windows.get_primary_mut() {
        window.set_resolution(width, height);
    }
}

impl Plugin for ViewportResizedPlugin {
    fn build(&self, app: &mut App) {
        let (sender, receiver) = mpsc::unbounded();
        let window = web_sys::window().expect("could not get window");
        gloo_events::EventListener::new(&window, "resize", move |_event| {
            sender
                .unbounded_send(get_viewport_size().into())
                .unwrap_throw();
        })
        .forget();

        app.insert_resource(ViewportState { receiver })
            .add_system(resized_event_system)
            .add_startup_system(initial_size_system);
    }
}
