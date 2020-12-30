use x11rb::{connection::Connection, protocol::xproto::*};

use crate::config::*;

pub struct Border {
    pub active: u32,
    pub inactive: u32,
    pub width: u8,
}

impl Border {
    pub fn new<C: Connection>(
        conn: &C,
        screen: &Screen,
        config: &Config,
    ) -> Self {
        // allocating colors
        let active = conn
            .alloc_color(
                screen.default_colormap,
                config.border.active[0],
                config.border.active[1],
                config.border.active[2],
            )
            .expect("default colormap doesn't exist")
            .reply()
            .expect("Couldn't get pixel value for border_active color");
        let inactive = conn
            .alloc_color(
                screen.default_colormap,
                config.border.inactive[0],
                config.border.inactive[1],
                config.border.inactive[2],
            )
            .expect("default colormap doesn't exist")
            .reply()
            .expect("Couldn't get pixel value for border_inactive color");

        Border {
            active: active.pixel,
            inactive: inactive.pixel,
            width: config.border.width,
        }
    }
}
