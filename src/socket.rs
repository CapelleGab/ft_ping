use std::net::Ipv4Addr;

pub fn create(ttl: u8) -> i32 {
    let fd = unsafe {
        libc::socket(libc::AF_INET, libc::SOCK_RAW, libc::IPPROTO_ICMP)
    };

    if fd == -1 {
        eprintln!("ft_ping: socket: Operation not permitted");
        std::process::exit(1);
    }

    let ttl: i32 = ttl as i32;
    unsafe {
        libc::setsockopt(
            fd,
            libc::IPPROTO_IP,
            libc::IP_TTL,
            &ttl as *const i32 as *const libc::c_void,
            std::mem::size_of::<i32>() as libc::socklen_t,
        );
    }

    fd
}

pub fn send(fd: i32, ip: Ipv4Addr, packet: &[u8]) -> isize {
    let dest_addr = libc::sockaddr_in {
        sin_family: libc::AF_INET as libc::sa_family_t,
        sin_port: 0,
        sin_addr: libc::in_addr { s_addr: ip.to_bits().to_be() },
        sin_zero: [0; 8],
        sin_len: 0,
    };

    unsafe {
        libc::sendto(
            fd,
            packet.as_ptr() as *const libc::c_void,
            packet.len(),
            0,
            &dest_addr as *const libc::sockaddr_in as *const libc::sockaddr,
            std::mem::size_of::<libc::sockaddr_in>() as libc::socklen_t,
        )
    }
}

pub fn receive(fd: i32, buf: &mut [u8]) -> isize {
    let mut src_addr: libc::sockaddr_in = unsafe { std::mem::zeroed() };
    let mut addr_len: libc::socklen_t = std::mem::size_of::<libc::sockaddr_in>() as libc::socklen_t;

    unsafe {
        libc::recvfrom(
            fd,
            buf.as_mut_ptr() as *mut libc::c_void,
            buf.len(),
            0,
            &mut src_addr as *mut libc::sockaddr_in as *mut libc::sockaddr,
            &mut addr_len,
        )
    }
}

pub fn close(fd: i32) {
    unsafe { libc::close(fd); }
}
