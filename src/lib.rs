// Rust bindings for LwIP TCP/IP stack.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Interfaces to LwIP socket calls.
// This module is used by:
// - UdpSocket, TcpListener and TcpStream (in sys_common/net.rs)
// - Socket (below)

#[allow(nonstandard_style)]
use core::ffi::{c_char, c_int, c_void};
use std::mem::size_of;
use std::os::freertos::io::RawSocket;

// Rust bindings for LwIP TCP/IP stack.
include!("lwip-rs.rs");

// Descriptor for default network interface, which we snoop on to ascertain readiness for operation. Read-only from here.
extern "C" {
    static gnetif: netif;
}

// This constant not in LwIP Rust bindings, but needed by sys_common\net.rs
pub const IPV6_MULTICAST_LOOP: i32 = 19; // Not supported in LwIP

pub fn socket(family: c_int, socket_type: c_int, _protocol: c_int) -> c_int {
    let socket_handle = unsafe { lwip_socket(family, socket_type, IPPROTO_IP) };
    socket_handle
}

pub fn setsockopt(
    sock: RawSocket,
    level: c_int,
    optname: c_int,
    optval: *const c_void,
    optlen: socklen_t,
) -> c_int {
    let retval = unsafe { lwip_setsockopt(sock, level, optname, optval, optlen) };
    match retval {
        0 => 0,
        _ => -1,
    }
}

pub fn getsockopt(
    sock: RawSocket,
    level: c_int,
    optname: c_int,
    optval: *mut c_void,
    optlen: *mut socklen_t,
) -> c_int {
    let retval = unsafe { lwip_getsockopt(sock, level, optname, optval, optlen) };
    match retval {
        0 => 0,
        _ => -1,
    }
}

pub fn bind(sock: RawSocket, name: *const sockaddr, namelen: socklen_t) -> c_int {
    let retval = unsafe { lwip_bind(sock, name, namelen) };
    match retval {
        0 => 0,
        _ => -1,
    }
}

pub fn connect(sock: RawSocket, name: *const sockaddr, namelen: socklen_t) -> c_int {
    let retval = unsafe { lwip_connect(sock, name, namelen) };
    match retval {
        0 => 0,
        _ => -1,
    }
}

pub fn listen(sock: RawSocket, backlog: c_int) -> c_int {
    let retval = unsafe { lwip_listen(sock, backlog) };
    match retval {
        0 => 0,
        _ => -1,
    }
}

pub fn accept(sock: RawSocket, name: *mut sockaddr, namelen: *mut socklen_t) -> c_int {
    let retval = unsafe { lwip_accept(sock, name, namelen) };
    match retval {
        0 => 0,
        _ => -1,
    }
}

pub fn getsockname(sock: RawSocket, name: *mut sockaddr, namelen: *mut socklen_t) -> c_int {
    unsafe {
        let retval = lwip_getsockname(sock, name, namelen);
        retval
    }
}

pub fn send(sock: RawSocket, mem: *const c_void, len: i32, flags: c_int) -> i32 {
    unsafe {
        let retval = lwip_send(sock, mem, len, flags);

        retval
    }
}

pub fn sendto(
    sock: RawSocket,
    mem: *const c_void,
    len: i32,
    flags: c_int,
    to: *const sockaddr,
    tolen: socklen_t,
) -> i32 {
    // Get the socket type using getsockopt
    let mut option: c_int = 0;
    let mut option_len = size_of::<c_int>() as socklen_t;
    let retval: c_int = getsockopt(
        sock,
        SOL_SOCKET,
        SO_TYPE,
        &mut option as *mut _ as *mut c_void,
        &mut option_len,
    );
    if retval == -1 {
        return -1;
    }
    match option {
        SOCK_DGRAM => unsafe {
            let retval = lwip_sendto(sock, mem, len, flags, to, tolen);

            retval
        },
        // TCP sendto? Makes no sense.
        SOCK_STREAM => -1,
        // Catch-all
        _ => -1,
    }
}

pub fn sendmsg(sock: RawSocket, message: *const msghdr, flags: c_int) -> i32 {
    let retval = unsafe { lwip_sendmsg(sock, message, flags) };
    retval
}

pub fn recv(sock: RawSocket, mem: *mut c_void, len: i32, flags: c_int) -> i32 {
    let retval = unsafe { lwip_recv(sock, mem, len as size_t, flags) };

    retval
}

pub fn recvfrom(
    sock: RawSocket,
    mem: *mut c_void,
    len: i32,
    flags: c_int,
    from: *mut sockaddr,
    fromlen: *mut socklen_t,
) -> i32 {
    // Get the socket type using getsockopt
    let mut option: c_int = 0;
    let mut option_len = size_of::<c_int>() as socklen_t;
    let retval: c_int = getsockopt(
        sock,
        SOL_SOCKET,
        SO_TYPE,
        &mut option as *mut _ as *mut c_void,
        &mut option_len,
    );
    if retval == -1 {
        return -1;
    }
    match option {
        SOCK_DGRAM => {
            let retval = unsafe { lwip_recvfrom(sock, mem, len as size_t, flags, from, fromlen) };

            retval
        }
        // TCP recvfrom? Makes no sense.
        SOCK_STREAM => -1,
        // Catch-all
        _ => -1,
    }
}

pub fn recvmsg(sock: RawSocket, message: *mut msghdr, flags: c_int) -> i32 {
    let retval = unsafe { lwip_recvmsg(sock, message, flags) };
    retval
}

pub fn getpeername(sock: RawSocket, name: *mut sockaddr, namelen: *mut socklen_t) -> c_int {
    unsafe {
        let retval = lwip_getpeername(sock, name, namelen);
        retval
    }
}

pub fn getaddrinfo(
    nodename: *const c_char,
    servname: *const c_char,
    hints: *const addrinfo,
    res: *mut *mut addrinfo,
) -> c_int {
    let retval = unsafe { lwip_getaddrinfo(nodename, servname, hints, res) };
    retval
}

pub fn freeaddrinfo(ai: *mut addrinfo) {
    unsafe { lwip_freeaddrinfo(ai) };
}

pub fn is_netif_initialised() -> bool {
    // Crude check that the interface is up by seeing if an IP address has been assigned.
    // Unfortunately, LwIP does not provide a clean API function to do this.
    unsafe { gnetif.ip_addr.addr != 0 }
}

pub fn shutdown(sock: RawSocket, how: c_int) -> i32 {
    let retval = unsafe { lwip_shutdown(sock, how) };
    retval
}

pub fn poll(fds: *const pollfd, nfds: nfds_t, timeout: core::ffi::c_int) -> i32 {
    let retval = unsafe { lwip_poll(fds, nfds, timeout) };
    retval
}

pub fn fcntl(s: core::ffi::c_int, cmd: core::ffi::c_int, val: core::ffi::c_int) -> i32 {
    let retval = unsafe { lwip_fcntl(s, cmd, val) };
    retval
}

pub fn ioctl(s: core::ffi::c_int, cmd: core::ffi::c_long, argp: *mut core::ffi::c_void) -> i32 {
    let retval = unsafe { lwip_ioctl(s, cmd, argp) };
    retval
}