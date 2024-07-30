use lazy_static::lazy_static;
use wayland_client::globals::{self, registry_queue_init, GlobalListContents};
use wayland_client::protocol::wl_registry;
use std::collections::HashMap;
use tokio::sync::Mutex;
use wayland_client::{Connection, QueueHandle};

use wayland_protocols_wlr::foreign_toplevel::v1::client::zwlr_foreign_toplevel_handle_v1::ZwlrForeignToplevelHandleV1 as ToplevelHandle;
use wayland_protocols_wlr::foreign_toplevel::v1::client::zwlr_foreign_toplevel_manager_v1::{self, ZwlrForeignToplevelManagerV1 as ToplevelManager};

#[derive(Clone)]
pub struct Window {
    pub title: String,
    pub appid: String,
}

pub struct State {
    pub current_window: Option<u32>,
    pub all_windows: HashMap<u32, Window>,
}

impl wayland_client::Dispatch<ToplevelManager, GlobalListContents> for State {
    fn event(
        state: &mut State,
        proxy: &ToplevelManager,
        event: zwlr_foreign_toplevel_manager_v1::Event,
        // This mutex contains an up-to-date list of the currently known globals
        // including the one that was just added or destroyed
        data: &GlobalListContents,
        conn: &Connection,
        qhandle: &QueueHandle<State>,
    ) {
        /* react to dynamic global events here */
    }
}

lazy_static! {
    static ref WINDOW_STATE_LOCKED: Mutex<State> = Mutex::new(State {
        current_window: None,
        all_windows: HashMap::new(),
    });
}

pub fn get_focused_window() -> Option<Window> {
    let window_state = WINDOW_STATE_LOCKED.lock().expect("Unable to take lock");
    let current_window_id = match window_state.current_window {
        Some(id) => id,
        None => {
            println!("No focused window");
            return None;
        }
    };
    match window_state.all_windows.get(&current_window_id) {
        Some(window_ref) => Some(window_ref.clone()),
        None => None,
    }
}

