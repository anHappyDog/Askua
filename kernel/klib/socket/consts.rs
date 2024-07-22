use crate::mm::page::PAGE_SIZE;

const SOCKET_COUNT: usize = 128;
const PENDING_COUNT: usize = 128;
const MESSAGE_COUNT: usize = 512;
const SOCKET_BUFFER_SIZE: usize = PAGE_SIZE * 32; // 确保 PAGE_SIZE 已经在某处定义

// 地址族
const AF_UNIX: i32 = 1; // Unix 域套接字
const AF_LOCAL: i32 = 1; // POSIX 名称为 AF_UNIX
const AF_INET: i32 = 2;
const AF_INET6: i32 = 10;

// 套接字类型
const SOCK_STREAM: i32 = 1;
const SOCK_DGRAM: i32 = 2;

// 套接字选项级别
const SOL_SOCKET: i32 = 1;

// 套接字选项名称
const SO_RCVBUF: i32 = 8;
const SO_SNDBUF: i32 = 7;

// 关闭套接字的方式
const SHUT_RD: i32 = 0;
const SHUT_WR: i32 = 1;
const SHUT_RDWR: i32 = 2;
