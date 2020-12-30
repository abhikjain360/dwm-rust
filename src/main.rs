use std::ffi::CString;

use nix::sys::{signal, wait};
use x11rb::{
    connection::Connection, errors::ReplyError, protocol::xproto::*, COPY_DEPTH_FROM_PARENT,
};

mod cursor;
use cursor::*;

mod border;
use border::*;

mod config;
use config::*;

struct WMAtoms {
    protocols: Atom,
    delete: Atom,
    state: Atom,
    take_focus: Atom,
}

struct NetAtoms {
    supported: Atom,
    name: Atom,
    state: Atom,
    check: Atom,
    fullscreen: Atom,
    active_window: Atom,
    window_type: Atom,
    window_type_dialog: Atom,
    client_list: Atom,
}

fn become_wm<C: Connection>(conn: &C, screen: &Screen) -> Result<(), ReplyError> {
    let values = ChangeWindowAttributesAux::default().event_mask(
        EventMask::SubstructureRedirect | EventMask::SubstructureNotify | EventMask::EnterWindow,
    );
    conn.change_window_attributes(screen.root, &values)?.check()
}

// originally `sigchld`
extern "C" fn kill_zombies(_: libc::c_int) {
    let sig_handler = signal::SigHandler::Handler(kill_zombies);
    unsafe { signal::signal(signal::Signal::SIGCHLD, sig_handler) }
        .expect("unable to install SIGCHLD handler");
    loop {
        if wait::waitpid(
            Some(nix::unistd::Pid::from_raw(-1)),
            Some(wait::WaitPidFlag::WNOHANG),
        )
        .is_ok()
        {
            break;
        }
    }
}

fn get_atoms<C: Connection>(conn: &C) -> (WMAtoms, NetAtoms) {
    (
        WMAtoms {
            protocols: conn
                .intern_atom(false, b"UTF8_STRING")
                .unwrap()
                .reply()
                .unwrap()
                .atom,
            delete: conn
                .intern_atom(false, b"WM_DELETE_WINDOW")
                .unwrap()
                .reply()
                .unwrap()
                .atom,
            state: conn
                .intern_atom(false, b"WM_STATE")
                .unwrap()
                .reply()
                .unwrap()
                .atom,
            take_focus: conn
                .intern_atom(false, b"WM_TAKE_FOCUS")
                .unwrap()
                .reply()
                .unwrap()
                .atom,
        },
        NetAtoms {
            active_window: conn
                .intern_atom(false, b"_NET_ACTIVE_WINDOW")
                .unwrap()
                .reply()
                .unwrap()
                .atom,
            supported: conn
                .intern_atom(false, b"_NET_SUPPORTED")
                .unwrap()
                .reply()
                .unwrap()
                .atom,
            name: conn
                .intern_atom(false, b"_NET_WM_NAME")
                .unwrap()
                .reply()
                .unwrap()
                .atom,
            state: conn
                .intern_atom(false, b"_NET_WM_STATE")
                .unwrap()
                .reply()
                .unwrap()
                .atom,
            check: conn
                .intern_atom(false, b"_NET_SUPPORTING_WM_CHECK")
                .unwrap()
                .reply()
                .unwrap()
                .atom,
            fullscreen: conn
                .intern_atom(false, b"_NET_WM_STATE_FULLSCREEN")
                .unwrap()
                .reply()
                .unwrap()
                .atom,
            window_type: conn
                .intern_atom(false, b"_NET_WM_WINDOW_TYPE")
                .unwrap()
                .reply()
                .unwrap()
                .atom,
            window_type_dialog: conn
                .intern_atom(false, b"_NET_WM_WINDOW_TYPE_DIALOG")
                .unwrap()
                .reply()
                .unwrap()
                .atom,
            client_list: conn
                .intern_atom(false, b"_NET_CLIENT_LIST")
                .unwrap()
                .reply()
                .unwrap()
                .atom,
        },
    )
}

fn main() {
    // TODO: setup up argument parsing

    // setting up locale
    if unsafe { libc::setlocale(libc::LC_CTYPE, CString::default().as_ptr()) }.is_null() {
        panic!("unable to set locale");
    }

    let (conn, screen_num) = x11rb::connect(None).expect("unable to open display");

    let screen = &conn.setup().roots[screen_num];

    // become the wm if no other running
    become_wm(&conn, &screen).expect("another window manager running");

    /* `setup()` begins here */

    // TODO: multimonitor support

    // kill of all the zombies
    kill_zombies(0);

    // extract config
    let config = Config::new();

    // create cursor
    let cursor = cursor::Cursor::new(&conn);

    // boders
    let borders = Border::new(&conn, &screen, &config);

    // init atoms
    let (wm_atoms, net_atoms) = get_atoms(&conn);

    /* `setup()` ends here */
}
