use std::error::Error;
use x11rb::{connection::Connection, protocol::xproto::*};

pub struct Border {
    pub active: Colormap,
    pub inactive: Colormap,
    pub width: u8,
}

impl Border {
    pub fn new<C: Connection>(conn: &C, screen: &Screen) -> Result<Self, Box<dyn Error>> {
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
        //conn.alloc_color(active);

        Ok(Border {
            active,
            inactive,
            width: 5,
        })
    }
}
