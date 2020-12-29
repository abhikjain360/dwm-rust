use std::error::Error;

use x11rb::{connection::Connection, protocol::xproto::*};

use crate::config::*;

pub struct Border {
    pub active: Colormap,
    pub inactive: Colormap,
    pub width: u8,
}

impl Border {
    pub fn new<C: Connection>(
        conn: &C,
        screen: &Screen,
        config: &Config,
    ) -> Result<Self, Box<dyn Error>> {
        // generating ids
        let active = conn.generate_id()?;
        let inactive = conn.generate_id()?;

        // creating colormaps
        conn.create_colormap(ColormapAlloc::All, active, screen.root, screen.root_visual)?;
        conn.create_colormap(
            ColormapAlloc::All,
            inactive,
            screen.root,
            screen.root_visual,
        )?;

        // allocating colors
        conn.alloc_color(
            active,
            config.border.active[0],
            config.border.active[1],
            config.border.active[2],
        )?;
        conn.alloc_color(
            inactive,
            config.border.inactive[0],
            config.border.inactive[1],
            config.border.inactive[2],
        )?;

        Ok(Border {
            active,
            inactive,
            width: config.border.width,
        })
    }
}
