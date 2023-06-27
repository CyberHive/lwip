// Rust bindings for LwIP TCP/IP stack.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Interfaces to LwIP socket calls.

pub const EPERM: i32 = 1; // Operation not permitted
pub const ENOENT: i32 = 2; // No such file or directory
pub const ESRCH: i32 = 3; // No such process
pub const EINTR: i32 = 4; // Interrupted system call
pub const EIO: i32 = 5; // I/O error
pub const ENXIO: i32 = 6; // No such device or address
pub const E2BIG: i32 = 7; // Arg list too long
pub const ENOEXEC: i32 = 8; // Exec format error
pub const EBADF: i32 = 9; // Bad file number
pub const ECHILD: i32 = 10; // No child processes
pub const EAGAIN: i32 = 11; // Try again
pub const ENOMEM: i32 = 12; // Out of memory
pub const EACCES: i32 = 13; // Permission denied
pub const EFAULT: i32 = 14; // Bad address
pub const ENOTBLK: i32 = 15; // Block device required
pub const EBUSY: i32 = 16; // Device or resource busy
pub const EEXIST: i32 = 17; // File exists
pub const EXDEV: i32 = 18; // Cross-device link
pub const ENODEV: i32 = 19; // No such device
pub const ENOTDIR: i32 = 20; // Not a directory
pub const EISDIR: i32 = 21; // Is a directory
pub const EINVAL: i32 = 22; // Invalid argument
pub const ENFILE: i32 = 23; // File table overflow
pub const EMFILE: i32 = 24; // Too many open files
pub const ENOTTY: i32 = 25; // Not a typewriter
pub const ETXTBSY: i32 = 26; // Text file busy
pub const EFBIG: i32 = 27; // File too large
pub const ENOSPC: i32 = 28; // No space left on device
pub const ESPIPE: i32 = 29; // Illegal seek
pub const EROFS: i32 = 30; // Read-only file system
pub const EMLINK: i32 = 31; // Too many links
pub const EPIPE: i32 = 32; // Broken pipe
pub const EDOM: i32 = 33; // Math argument out of domain of func
pub const ERANGE: i32 = 34; // Math result not representable
pub const EDEADLK: i32 = 35; // Resource deadlock would occur
pub const ENAMETOOLONG: i32 = 36; // File name too long
pub const ENOLCK: i32 = 37; // No record locks available
pub const ENOSYS: i32 = 38; // Function not implemented
pub const ENOTEMPTY: i32 = 39; // Directory not empty
pub const ELOOP: i32 = 40; // Too many symbolic links encountered
pub const EWOULDBLOCK: i32 = EAGAIN; // Operation would block
pub const ENOMSG: i32 = 42; // No message of desired type
pub const EIDRM: i32 = 43; // Identifier removed
pub const ECHRNG: i32 = 44; // Channel number out of range
pub const EL2NSYNC: i32 = 45; // Level 2 not synchronized
pub const EL3HLT: i32 = 46; // Level 3 halted
pub const EL3RST: i32 = 47; // Level 3 reset
pub const ELNRNG: i32 = 48; // Link number out of range
pub const EUNATCH: i32 = 49; // Protocol driver not attached
pub const ENOCSI: i32 = 50; // No CSI structure available
pub const EL2HLT: i32 = 51; // Level 2 halted
pub const EBADE: i32 = 52; // Invalid exchange
pub const EBADR: i32 = 53; // Invalid request descriptor
pub const EXFULL: i32 = 54; // Exchange full
pub const ENOANO: i32 = 55; // No anode
pub const EBADRQC: i32 = 56; // Invalid request code
pub const EBADSLT: i32 = 57; // Invalid slot
pub const EDEADLOCK: i32 = EDEADLK;
pub const EBFONT: i32 = 59; // Bad font file format
pub const ENOSTR: i32 = 60; // Device not a stream
pub const ENODATA: i32 = 61; // No data available
pub const ETIME: i32 = 62; // Timer expired
pub const ENOSR: i32 = 63; // Out of streams resources
pub const ENONET: i32 = 64; // Machine is not on the network
pub const ENOPKG: i32 = 65; // Package not installed
pub const EREMOTE: i32 = 66; // Object is remote
pub const ENOLINK: i32 = 67; // Link has been severed
pub const EADV: i32 = 68; // Advertise error
pub const ESRMNT: i32 = 69; // Srmount error
pub const ECOMM: i32 = 70; // Communication error on send
pub const EPROTO: i32 = 71; // Protocol error
pub const EMULTIHOP: i32 = 72; // Multihop attempted
pub const EDOTDOT: i32 = 73; // RFS specific error
pub const EBADMSG: i32 = 74; // Not a data message
pub const EOVERFLOW: i32 = 75; // Value too large for defined data type
pub const ENOTUNIQ: i32 = 76; // Name not unique on network
pub const EBADFD: i32 = 77; // File descriptor in bad state
pub const EREMCHG: i32 = 78; // Remote address changed
pub const ELIBACC: i32 = 79; // Can not access a needed shared library
pub const ELIBBAD: i32 = 80; // Accessing a corrupted shared library
pub const ELIBSCN: i32 = 81; // .lib section in a.out corrupted
pub const ELIBMAX: i32 = 82; // Attempting to link in too many shared libraries
pub const ELIBEXEC: i32 = 83; // Cannot exec a shared library directly
pub const EILSEQ: i32 = 84; // Illegal byte sequence
pub const ERESTART: i32 = 85; // Interrupted system call should be restarted
pub const ESTRPIPE: i32 = 86; // Streams pipe error
pub const EUSERS: i32 = 87; // Too many users
pub const ENOTSOCK: i32 = 88; // Socket operation on non-socket
pub const EDESTADDRREQ: i32 = 89; // Destination address required
pub const EMSGSIZE: i32 = 90; // Message too long
pub const EPROTOTYPE: i32 = 91; // Protocol wrong type for socket
pub const ENOPROTOOPT: i32 = 92; // Protocol not available
pub const EPROTONOSUPPORT: i32 = 93; // Protocol not supported
pub const ESOCKTNOSUPPORT: i32 = 94; // Socket type not supported
pub const EOPNOTSUPP: i32 = 95; // Operation not supported on transport endpoint
pub const EPFNOSUPPORT: i32 = 96; // Protocol family not supported
pub const EAFNOSUPPORT: i32 = 97; // Address family not supported by protocol
pub const EADDRINUSE: i32 = 98; // Address already in use
pub const EADDRNOTAVAIL: i32 = 99; // Cannot assign requested address
pub const ENETDOWN: i32 = 100; // Network is down
pub const ENETUNREACH: i32 = 101; // Network is unreachable
pub const ENETRESET: i32 = 102; // Network dropped connection because of reset
pub const ECONNABORTED: i32 = 103; // Software caused connection abort
pub const ECONNRESET: i32 = 104; // Connection reset by peer
pub const ENOBUFS: i32 = 105; // No buffer space available
pub const EISCONN: i32 = 106; // Transport endpoint is already connected
pub const ENOTCONN: i32 = 107; // Transport endpoint is not connected
pub const ESHUTDOWN: i32 = 108; // Cannot send after transport endpoint shutdown
pub const ETOOMANYREFS: i32 = 109; // Too many references: cannot splice
pub const ETIMEDOUT: i32 = 110; // Connection timed out
pub const ECONNREFUSED: i32 = 111; // Connection refused
pub const EHOSTDOWN: i32 = 112; // Host is down
pub const EHOSTUNREACH: i32 = 113; // No route to host
pub const EALREADY: i32 = 114; // Operation already in progress
pub const EINPROGRESS: i32 = 115; // Operation now in progress
pub const ESTALE: i32 = 116; // Stale NFS file handle
pub const EUCLEAN: i32 = 117; // Structure needs cleaning
pub const ENOTNAM: i32 = 118; // Not a XENIX named type file
pub const ENAVAIL: i32 = 119; // No XENIX semaphores available
pub const EISNAM: i32 = 120; // Is a named type file
pub const EREMOTEIO: i32 = 121; // Remote I/O error
pub const EDQUOT: i32 = 122; // Quota exceeded
pub const ENOMEDIUM: i32 = 123; // No medium found
pub const EMEDIUMTYPE: i32 = 124; // Wrong medium type

#[allow(nonstandard_style)]
use core::ffi::{c_char, c_int, c_void};
use std::os::freertos::io::RawSocket;

// Rust bindings for LwIP TCP/IP stack.
include!("lwip-rs.rs");

// Descriptor for default network interface, which we snoop on to ascertain readiness for operation. Read-only from here.
extern "C" {
    static gnetif: netif;
}

// This constant not in LwIP Rust bindings, but needed by sys_common\net.rs
pub const IPV6_MULTICAST_LOOP: i32 = 19; // Not supported in LwIP

pub fn socket(family: c_int, socket_type: c_int, protocol: c_int) -> c_int {
    let socket_handle = unsafe { lwip_socket(family, socket_type, protocol) };
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
    unsafe { lwip_send(sock, mem, len, flags) }
}

pub fn sendto(
    sock: RawSocket,
    mem: *const c_void,
    len: i32,
    flags: c_int,
    to: *const sockaddr,
    tolen: socklen_t,
) -> i32 {
    // Call lwip_sendto regardless of socket type. It will return an error for invalid combinations.
    // Previously only SOCK_DGRAM was supported, but we also need raw socket support.
    unsafe { lwip_sendto(sock, mem, len, flags, to, tolen) }
}

pub fn sendmsg(sock: RawSocket, message: *const msghdr, flags: c_int) -> i32 {
    unsafe { lwip_sendmsg(sock, message, flags) }
}

pub fn recv(sock: RawSocket, mem: *mut c_void, len: i32, flags: c_int) -> i32 {
    unsafe { lwip_recv(sock, mem, len as size_t, flags) }
}

pub fn recvfrom(
    sock: RawSocket,
    mem: *mut c_void,
    len: i32,
    flags: c_int,
    from: *mut sockaddr,
    fromlen: *mut socklen_t,
) -> i32 {
    // Call lwip_recvfrom regardless of socket type. It will return an error for invalid combinations.
    // Previously only SOCK_DGRAM was supported, but we also need raw socket support.
    unsafe { lwip_recvfrom(sock, mem, len as size_t, flags, from, fromlen) }
}

pub fn recvmsg(sock: RawSocket, message: *mut msghdr, flags: c_int) -> i32 {
    unsafe { lwip_recvmsg(sock, message, flags) }
}

pub fn getpeername(sock: RawSocket, name: *mut sockaddr, namelen: *mut socklen_t) -> c_int {
    unsafe { lwip_getpeername(sock, name, namelen) }
}

pub fn getaddrinfo(
    nodename: *const c_char,
    servname: *const c_char,
    hints: *const addrinfo,
    res: *mut *mut addrinfo,
) -> c_int {
    unsafe { lwip_getaddrinfo(nodename, servname, hints, res) }
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
    unsafe { lwip_shutdown(sock, how) }
}

pub fn poll(fds: *const pollfd, nfds: nfds_t, timeout: core::ffi::c_int) -> i32 {
    unsafe { lwip_poll(fds, nfds, timeout) }
}

pub fn fcntl(s: core::ffi::c_int, cmd: core::ffi::c_int, val: core::ffi::c_int) -> i32 {
    unsafe { lwip_fcntl(s, cmd, val) }
}

pub fn ioctl(s: core::ffi::c_int, cmd: core::ffi::c_long, argp: *mut core::ffi::c_void) -> i32 {
    unsafe { lwip_ioctl(s, cmd, argp) }
}
