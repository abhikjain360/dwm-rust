use x11rb::protocol::xproto::{self, ConnectionExt};

pub struct Cursor {
    pub cur_normal: xproto::Cursor,
    pub cur_resize: xproto::Cursor,
    pub cur_move: xproto::Cursor,
    pub cur_last: xproto::Cursor,
}

impl Cursor {
    pub fn new<C: x11rb::connection::Connection>(conn: &C) -> Self {
        let font = conn.generate_id().unwrap();
        xproto::open_font(conn, font, b"cursor").unwrap();

        Cursor {
            cur_normal: {
                let cursor_id = conn.generate_id().unwrap();
                conn.create_glyph_cursor(
                    cursor_id,
                    font,
                    font,
                    68,
                    68 + 1,
                    0,
                    0,
                    0,
                    0xFFFF,
                    0xFFFF,
                    0xFFFF,
                )
                .unwrap();
                cursor_id
            },
            cur_resize: {
                let cursor_id = conn.generate_id().unwrap();
                conn.create_glyph_cursor(
                    cursor_id,
                    font,
                    font,
                    120,
                    120 + 1,
                    0,
                    0,
                    0,
                    0xFFFF,
                    0xFFFF,
                    0xFFFF,
                )
                .unwrap();
                cursor_id
            },
            cur_move: {
                let cursor_id = conn.generate_id().unwrap();
                conn.create_glyph_cursor(
                    cursor_id,
                    font,
                    font,
                    52,
                    52 + 1,
                    0,
                    0,
                    0,
                    0xFFFF,
                    0xFFFF,
                    0xFFFF,
                )
                .unwrap();
                cursor_id
            },
            cur_last: {
                let cursor_id = conn.generate_id().unwrap();
                conn.create_glyph_cursor(
                    cursor_id,
                    font,
                    font,
                    68,
                    68 + 1,
                    0,
                    0,
                    0,
                    0xFFFF,
                    0xFFFF,
                    0xFFFF,
                )
                .unwrap();
                cursor_id
            },
        }
    }
}
