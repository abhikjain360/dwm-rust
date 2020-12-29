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

fn main() {
    // TODO: setup up argument parsing

    if unsafe { libc::setlocale(libc::LC_CTYPE, CString::default().as_ptr()) }.is_null() {
        panic!("unable to set locale");
    }

    let (conn, screen_num) = x11rb::connect(None).expect("unable to open display");

    let screen = &conn.setup().roots[screen_num];

    // become the wm if no other running
    become_wm(&conn, &screen).expect("another window manager running");

    // extract config
    let config = Config::new();
    println!("{:?}", config);

    // originally the `setup()` function, but all done here as it felt that libraries are already
    // doing most of the stuff

    // kill of all the zombies
    kill_zombies(0);

    // create cursor
    let cursor = cursor::Cursor::new(&conn);
}
