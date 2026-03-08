use std::ffi::CString;
use std::net::Ipv4Addr;
use std::ptr;

pub fn resolve(host: &str) -> Result<Ipv4Addr, String> {
    let c_host = CString::new(host)
        .map_err(|_| format!("ft_ping: {}: Invalid hostname", host))?;

    let mut hints: libc::addrinfo = unsafe { std::mem::zeroed() };
    hints.ai_family = libc::AF_INET;
    hints.ai_socktype = libc::SOCK_RAW;

    let mut res: *mut libc::addrinfo = ptr::null_mut();

    let ret = unsafe {
        libc::getaddrinfo(c_host.as_ptr(), ptr::null(), &hints, &mut res)
    };

    if ret != 0 || res.is_null() {
        return Err(format!("ft_ping: {}: Name or service not known", host));
    }

    let addr = unsafe {
        let sockaddr = (*res).ai_addr as *const libc::sockaddr_in;
        let ip = (*sockaddr).sin_addr.s_addr.to_be();
        libc::freeaddrinfo(res);
        Ipv4Addr::from(ip)
    };

    Ok(addr)
}
