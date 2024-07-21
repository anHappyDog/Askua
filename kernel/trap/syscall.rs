use super::context::UserContext;
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
