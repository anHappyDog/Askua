mod sysno;

use super::context::UserContext;
use crate::klib::socket::SocketAddr;
use crate::klib::sys::{time, ustname};
use crate::mm::address::UserObj;

fn sys_times(utms: UserObj<time::Tms>) -> usize {
    unimplemented!()
}

fn sys_exit() -> ! {
    loop {}
}

fn sys_getppid() -> usize {
    unimplemented!()
}

fn sys_getpid() -> usize {
    unimplemented!()
}

fn sys_uname(uuname: UserObj<ustname::Ustname>) -> usize {
    unimplemented!()
}

fn sys_gettimeofday(utv: UserObj<time::TimeVal>, utz: UserObj<time::TimeZone>) -> usize {
    unimplemented!()
}

fn sys_wait4(pid: usize, status: UserObj<usize>, options: usize, rusage: UserObj<u8>) -> usize {
    unimplemented!()
}

fn sys_pipe2(fds: UserObj<[usize; 2]>, flags: usize) -> usize {
    unimplemented!()
}

fn sys_dup(fd: usize) -> usize {
    unimplemented!()
}

fn sys_getcwd(buf: UserObj<&[u8]>, size: usize) -> usize {
    unimplemented!()
}

fn sys_dup3(oldfd: usize, newfd: usize, flags: usize) -> usize {
    unimplemented!()
}

fn sys_chdir(path: UserObj<&[u8]>) -> usize {
    unimplemented!()
}

fn sys_openat(dirfd: usize, path: UserObj<&[u8]>, flags: usize, mode: usize) -> usize {
    unimplemented!()
}

fn sys_close(fd: usize) -> usize {
    unimplemented!()
}

fn sys_getdents64(fd: usize, buf: UserObj<&[u8]>, count: usize) -> usize {
    unimplemented!()
}

fn sys_read(fd: usize, buf: UserObj<&[u8]>, count: usize) -> usize {
    unimplemented!()
}

fn sys_write(fd: usize, buf: UserObj<&[u8]>, count: usize) -> usize {
    unimplemented!()
}

fn sys_lseek(fd: usize, offset: usize, whence: usize) -> usize {
    unimplemented!()
}

fn sys_faccessat(dirfd: usize, path: UserObj<u8>, mode: usize, flags: usize) -> usize {
    unimplemented!()
}

fn sys_ppoll() -> usize {
    unimplemented!()
}

fn sys_pselect6() -> usize {
    unimplemented!()
}

fn sys_fcntl(fd: usize, cmd: usize, arg: usize) -> usize {
    unimplemented!()
}

fn sys_utimenset() -> usize {
    unimplemented!()
}

fn sys_pread64() -> usize {
    unimplemented!()
}

fn sys_pwrite64() -> usize {
    unimplemented!()
}

fn sys_statfs() -> usize {
    unimplemented!()
}

fn sys_ftruncate() -> usize {
    unimplemented!()
}

fn sys_readlinkat() -> usize {
    unimplemented!()
}

fn sys_socketpair() -> usize {
    unimplemented!()
}

fn sys_linkat(
    olddirfd: usize,
    oldpath: UserObj<&[u8]>,
    newdirfd: usize,
    newpath: UserObj<&[u8]>,
    flags: usize,
) -> usize {
    unimplemented!()
}

fn sys_unlinkat(dirfd: usize, path: UserObj<&[u8]>, flags: usize) -> usize {
    unimplemented!()
}

fn sys_mkdirat(dirfd: usize, path: UserObj<&[u8]>, mode: usize) -> usize {
    unimplemented!()
}

fn sys_umount2(target: UserObj<&[u8]>, flags: usize) -> usize {
    unimplemented!()
}

fn sys_mount(
    source: UserObj<&[u8]>,
    target: UserObj<&[u8]>,
    filesystemtype: UserObj<&[u8]>,
    flags: usize,
    data: UserObj<&[u8]>,
) -> usize {
    unimplemented!()
}

fn sys_fstat(fd: usize, statbuf: UserObj<&[u8]>) -> usize {
    unimplemented!()
}

fn sys_nanosleep(rqtp: UserObj<time::TimeSpec>, rmtp: UserObj<time::TimeSpec>) -> usize {
    unimplemented!()
}

fn sys_clone(
    flags: usize,
    child_stack: UserObj<&[u8]>,
    ptid: UserObj<usize>,
    ctid: UserObj<usize>,
    newtls: usize,
) -> usize {
    unimplemented!()
}

fn sys_execve(
    filename: UserObj<&[u8]>,
    argv: UserObj<&[UserObj<&[u8]>]>,
    envp: UserObj<&[UserObj<&[u8]>]>,
) -> usize {
    unimplemented!()
}

fn sys_yield() -> usize {
    unimplemented!()
}

fn sys_brk(brk: usize) -> usize {
    unimplemented!()
}

fn sys_mmap(
    addr: usize,
    length: usize,
    prot: usize,
    flags: usize,
    fd: usize,
    offset: usize,
) -> usize {
    unimplemented!()
}

fn sys_munmap(addr: usize, length: usize) -> usize {
    unimplemented!()
}

fn sys_socket(domain: usize, _type: usize, protocol: usize) -> usize {
    unimplemented!()
}

fn sys_bind(sockfd: usize, addr: UserObj<SocketAddr>, addrlen: usize) -> usize {
    unimplemented!()
}

fn sys_listen(sockfd: usize, backlog: usize) -> usize {
    unimplemented!()
}

fn sys_connect(sockfd: usize, addr: UserObj<SocketAddr>, addrlen: usize) -> usize {
    unimplemented!()
}

fn sys_accept(sockfd: usize, addr: UserObj<SocketAddr>, addrlen: usize) -> usize {
    unimplemented!()
}

fn sys_recvfrom(
    sockfd: usize,
    buf: UserObj<u8>,
    len: usize,
    flags: usize,
    src_addr: UserObj<SocketAddr>,
    addrlen: UserObj<usize>,
) -> usize {
    unimplemented!()
}

fn sys_sendto(
    sockfd: usize,
    buf: UserObj<u8>,
    len: usize,
    flags: usize,
    dest_addr: UserObj<SocketAddr>,
    addrlen: usize,
) -> usize {
    unimplemented!()
}

fn sys_getsocketname(sockfd: usize, addr: UserObj<SocketAddr>, addrlen: UserObj<usize>) -> usize {
    unimplemented!()
}

fn sys_getpeername(sockfd: usize, addr: UserObj<SocketAddr>, addrlen: UserObj<usize>) -> usize {
    unimplemented!()
}

fn sys_getsockopt(
    sockfd: usize,
    level: usize,
    optname: usize,
    optval: UserObj<u8>,
    optlen: UserObj<usize>,
) -> usize {
    unimplemented!()
}

fn sys_setsockopt(
    sockfd: usize,
    level: usize,
    optname: usize,
    optval: UserObj<u8>,
    optlen: usize,
) -> usize {
    unimplemented!()
}

fn sys_sigaction(signum: usize, act: usize, oldact: usize, sigset_size: usize) -> usize {
    unimplemented!()
}

fn sys_sigprocmask(how: usize, set: usize, oldset: usize) -> usize {
    unimplemented!()
}

fn sys_sigsuspend(usigset: usize) -> usize {
    unimplemented!()
}

fn sys_kill(pid: usize, sig: usize) -> usize {
    unimplemented!()
}

fn sys_shmget(key: usize, size: usize, shmflg: usize) -> usize {
    unimplemented!()
}

fn sys_shmat(shmid: usize, shmaddr: usize, shmflg: usize) -> usize {
    unimplemented!()
}

fn sys_shmctl(shmid: usize, cmd: usize, buf: UserObj<u8>) -> usize {
    unimplemented!()
}

fn sys_getpgid(pid: usize) -> usize {
    unimplemented!()
}

fn sys_setpgid(pid: usize, pgid: usize) -> usize {
    unimplemented!()
}

fn sys_getrusage(who: usize, rusage: UserObj<u8>) -> usize {
    unimplemented!()
}

fn sys_syslog(level: usize, buf: UserObj<&[u8]>, len: usize) -> usize {
    unimplemented!()
}

fn sys_sysinfo(info: UserObj<u8>) -> usize {
    unimplemented!()
}

fn sys_fchmod(fd: usize, filename: UserObj<u8>, flags: usize, mode: usize) -> usize {
    unimplemented!()
}

fn sys_readv() -> usize {
    unimplemented!()
}

fn sys_writev() -> usize {
    unimplemented!()
}

fn sys_fsync(fd: usize) -> usize {
    unimplemented!()
}

fn do_syscall(context: &mut UserContext) -> usize {
    
    unimplemented!("syscall")
}
