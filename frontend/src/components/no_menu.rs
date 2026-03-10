/*
 * SizeMatters - a ticket sizing util
 * Copyright (C) 2026 Andre Onuki
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use crate::ws::WsContext;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

/// Shown in the sidebar when not connected. Clicking "Enter" initiates the WebSocket connection.
#[component]
pub fn NoMenu() -> impl IntoView {
    let ws = expect_context::<WsContext>();
    let navigate = use_navigate();
    let connecting = RwSignal::new(false);

    // Reactively navigate once connection succeeds
    let nav = navigate.clone();
    Effect::new(move |_| {
        if connecting.get() && ws.is_connected() {
            nav("/main", Default::default());
        }
    });

    // Timeout fallback: navigate to error if not connected after 5 seconds
    let nav2 = navigate.clone();
    Effect::new(move |prev: Option<bool>| {
        let is_connecting = connecting.get();
        if is_connecting && prev != Some(true) {
            let nav = nav2.clone();
            set_timeout(
                move || {
                    // Only navigate to error if still not connected
                    if !ws.is_connected() {
                        nav("/error/connection", Default::default());
                    }
                },
                std::time::Duration::from_secs(5),
            );
        }
        is_connecting
    });

    let on_enter = move |_| {
        connecting.set(true);
        ws.connect();
    };

    view! {
        <Show when=move || connecting.get()>
            <p>"Connecting..."</p>
        </Show>
        <Show when=move || !connecting.get()>
            <button class="btn btn-raised btn-primary" on:click=on_enter>
                "Enter"
            </button>
        </Show>
    }
}
