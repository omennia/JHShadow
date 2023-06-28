use nix::errno::Errno;
use shadow_shim_helper_rs::syscall_types::SysCallArgs;
use shadow_shim_helper_rs::syscall_types::SysCallReg;

use crate::cshadow as c;
use crate::host::context::{ThreadContext, ThreadContextObjs};
use crate::host::descriptor::descriptor_table::{DescriptorHandle, DescriptorTable};
use crate::host::descriptor::Descriptor;
use crate::host::syscall_types::SyscallReturn;
use crate::host::syscall_types::{SyscallError, SyscallResult};

// ADDED: JOAO HUGO
use std::ffi::CString;
use zermia_lib::message::Message;
use zermia_lib::send_zermia_message;
use std::cmp;
// // END


mod eventfd;
mod fcntl;
mod file;
mod ioctl;
mod mman;
mod random;
mod sched;
mod socket;
mod sysinfo;
mod time;
mod uio;
mod unistd;

type LegacySyscallFn =
    unsafe extern "C" fn(*mut c::SysCallHandler, *const SysCallArgs) -> SyscallReturn;

pub struct SyscallHandler {
    // Will eventually contain syscall handler state once migrated from the c handler
}

// ADDED - JH
static mut N:u32 = 0;

fn should_make_syscall(sys_number: u32, ip_src: u32) -> i32 {
  let my_desc = CString::new(format!{"[handler/mod.rs] Uma syscall, número {}, está a ser agora feita", sys_number});
  let mut msg = Message {
      code: sys_number, 
      ip_src: ip_src,
      ip_dest: 987654321,
      msg: [0; 32],
      return_status: false,
  };

  // let text_bytes = my_desc.as_bytes();
  // let text_bytes = my_desc.expect("REASON").as_bytes();
  let binding = my_desc.expect("REASON");
  let text_bytes = binding.as_bytes();

  msg.msg[..cmp::min(32, text_bytes.len())].copy_from_slice(&text_bytes[0..cmp::min(32, text_bytes.len())]);

  return send_zermia_message(msg);
}
// END

impl SyscallHandler {
    #[allow(clippy::new_without_default)]
    pub fn new() -> SyscallHandler {
        SyscallHandler {}
    }

    pub fn syscall(&self, mut ctx: SyscallContext) -> SyscallResult {
        //ADDED JSoares
        println!("[{:?}] [main.host.sycall.handler.mod.rs] syscall  {:?} from {:?} with address {:?}", std::thread::current().id(), ctx.args.number, ctx.objs.process.name(), ctx.objs.thread.get_tid_address());
        println!("{:?}", ctx.objs.host.info());
        println!("{:?}", ctx.objs.host.default_ip());

        if ctx.args.number == 44 { //send_to
          let action_to_do = should_make_syscall(44, ctx.objs.host.default_ip().into());
          match action_to_do {
            1 => { // Devolvemos erro (not connected)
              return SyscallResult::Err(Errno::ENOTCONN.into());
            }

            2 => { // Devolvemos a union
              return SyscallResult::Ok(ctx.args.get(2).into());
            }

            3 => { // Devolvemos a union com size 0
              return SyscallResult::Ok(SysCallReg::from(0));
            }

            6 => { // Fazemos duas chamadas ao sistema. Podemos configurar o monitor para devolver também a quantidade de vezes que chamamos esta função
              let _first_call = SyscallHandlerFn::call(Self::sendto, &mut ctx);              
              let _second_call = SyscallHandlerFn::call(Self::sendto, &mut ctx);              
              return _first_call;
            }

            _ => {
              // Não fazemos nada ( continuamos a syscall )
            }
          }          
          
          
          // if {
          //   println!("[{:?}] [{:?}", std::thread::current().id(), ctx.args.get(0));
          //   println!("[{:?}] [{:?}", std::thread::current().id(), ctx.args.get(1));
          //   println!("[{:?}] [{:?}", std::thread::current().id(), ctx.args.get(2));
          //   println!("[{:?}] [{:?}", std::thread::current().id(), ctx.args.get(3));
          //   println!("[{:?}] [{:?}", std::thread::current().id(), ctx.args.get(4));
          //   let ret = SyscallResult::Ok(ctx.args.get(2).into());
          //   println!("RERTURNING [{:?}]", ret);
          //   return ret;
          //   // return SyscallResult::Err(Errno::ENOTCONN.into())
          // }
        }

        else if ctx.args.number == 45 { // recv_from
            let action_to_do = should_make_syscall(45, ctx.objs.host.default_ip().into());
            match action_to_do {
              1 => { // Devolvemos erro (not connected)
                return SyscallResult::Err(Errno::ENOTCONN.into());
              }

              2 => { // Devolvemos a union
                return SyscallResult::Ok(ctx.args.get(2).into());
              }

              3 => { // Devolvemos a union com size 0
                return SyscallResult::Ok(SysCallReg::from(0));
              }

              _ => {
                // Não fazemos nada ( continuamos a syscall )
              }
            }    
            

            // println!("[{:?}] [{:?}", std::thread::current().id(), ctx.args.get(0));
            // println!("[{:?}] [{:?}", std::thread::current().id(), ctx.args.get(1));
            // println!("[{:?}] [{:?}", std::thread::current().id(), ctx.args.get(2));
            // println!("[{:?}] [{:?}", std::thread::current().id(), ctx.args.get(3));
            // println!("[{:?}] [{:?}", std::thread::current().id(), ctx.args.get(4));
            // let ret = SyscallResult::Ok(SysCallReg::from(0)); // Estavamos a usar este
            // println!("RERTURNING [{:?}]", ret);
            // return ret;
            // return SyscallResult::Err(Errno::ENOTCONN.into())

        }
        else if ctx.args.number == 68 {
          should_make_syscall(68686868, ctx.objs.host.default_ip().into());
        }
        else {
          should_make_syscall(ctx.args.number.try_into().unwrap(), ctx.objs.host.default_ip().into());
        }
        // END

        match ctx.args.number {
            libc::SYS_accept => SyscallHandlerFn::call(Self::accept, &mut ctx),
            libc::SYS_accept4 => SyscallHandlerFn::call(Self::accept4, &mut ctx),
            libc::SYS_bind => SyscallHandlerFn::call(Self::bind, &mut ctx),
            libc::SYS_brk => SyscallHandlerFn::call(Self::brk, &mut ctx),
            libc::SYS_close => SyscallHandlerFn::call(Self::close, &mut ctx),
            libc::SYS_connect => SyscallHandlerFn::call(Self::connect, &mut ctx),
            libc::SYS_dup => SyscallHandlerFn::call(Self::dup, &mut ctx),
            libc::SYS_dup2 => SyscallHandlerFn::call(Self::dup2, &mut ctx),
            libc::SYS_dup3 => SyscallHandlerFn::call(Self::dup3, &mut ctx),
            libc::SYS_eventfd => SyscallHandlerFn::call(Self::eventfd, &mut ctx),
            libc::SYS_eventfd2 => SyscallHandlerFn::call(Self::eventfd2, &mut ctx),
            libc::SYS_fcntl => SyscallHandlerFn::call(Self::fcntl, &mut ctx),
            libc::SYS_getitimer => SyscallHandlerFn::call(Self::getitimer, &mut ctx),
            libc::SYS_getpeername => SyscallHandlerFn::call(Self::getpeername, &mut ctx),
            libc::SYS_getrandom => SyscallHandlerFn::call(Self::getrandom, &mut ctx),
            libc::SYS_getsockname => SyscallHandlerFn::call(Self::getsockname, &mut ctx),
            libc::SYS_getsockopt => SyscallHandlerFn::call(Self::getsockopt, &mut ctx),
            libc::SYS_ioctl => SyscallHandlerFn::call(Self::ioctl, &mut ctx),
            libc::SYS_listen => SyscallHandlerFn::call(Self::listen, &mut ctx),
            libc::SYS_mmap => SyscallHandlerFn::call(Self::mmap, &mut ctx),
            libc::SYS_mprotect => SyscallHandlerFn::call(Self::mprotect, &mut ctx),
            libc::SYS_mremap => SyscallHandlerFn::call(Self::mremap, &mut ctx),
            libc::SYS_munmap => SyscallHandlerFn::call(Self::munmap, &mut ctx),
            libc::SYS_open => SyscallHandlerFn::call(Self::open, &mut ctx),
            libc::SYS_openat => SyscallHandlerFn::call(Self::openat, &mut ctx),
            libc::SYS_pipe => SyscallHandlerFn::call(Self::pipe, &mut ctx),
            libc::SYS_pipe2 => SyscallHandlerFn::call(Self::pipe2, &mut ctx),
            libc::SYS_pread64 => SyscallHandlerFn::call(Self::pread64, &mut ctx),
            libc::SYS_preadv => SyscallHandlerFn::call(Self::preadv, &mut ctx),
            libc::SYS_preadv2 => SyscallHandlerFn::call(Self::preadv2, &mut ctx),
            libc::SYS_pwrite64 => SyscallHandlerFn::call(Self::pwrite64, &mut ctx),
            libc::SYS_pwritev => SyscallHandlerFn::call(Self::pwritev, &mut ctx),
            libc::SYS_pwritev2 => SyscallHandlerFn::call(Self::pwritev2, &mut ctx),
            libc::SYS_rseq => SyscallHandlerFn::call(Self::rseq, &mut ctx),
            libc::SYS_read => SyscallHandlerFn::call(Self::read, &mut ctx),
            libc::SYS_readv => SyscallHandlerFn::call(Self::readv, &mut ctx),
            libc::SYS_recvfrom => SyscallHandlerFn::call(Self::recvfrom, &mut ctx),
            libc::SYS_recvmsg => SyscallHandlerFn::call(Self::recvmsg, &mut ctx),
            libc::SYS_sched_getaffinity => {
                SyscallHandlerFn::call(Self::sched_getaffinity, &mut ctx)
            }
            libc::SYS_sched_setaffinity => {
                SyscallHandlerFn::call(Self::sched_setaffinity, &mut ctx)
            }
            libc::SYS_sched_yield => SyscallHandlerFn::call(Self::sched_yield, &mut ctx),
            libc::SYS_sendmsg => SyscallHandlerFn::call(Self::sendmsg, &mut ctx),
            libc::SYS_sendto => { 
              let cenas = SyscallHandlerFn::call(Self::sendto, &mut ctx);
              println!("A mensagem de erro é {:?}", cenas);
              cenas
            },
            libc::SYS_setitimer => SyscallHandlerFn::call(Self::setitimer, &mut ctx),
            libc::SYS_setsockopt => SyscallHandlerFn::call(Self::setsockopt, &mut ctx),
            libc::SYS_shutdown => SyscallHandlerFn::call(Self::shutdown, &mut ctx),
            libc::SYS_socket => SyscallHandlerFn::call(Self::socket, &mut ctx),
            libc::SYS_socketpair => SyscallHandlerFn::call(Self::socketpair, &mut ctx),
            libc::SYS_sysinfo => SyscallHandlerFn::call(Self::sysinfo, &mut ctx),
            libc::SYS_write => SyscallHandlerFn::call(Self::write, &mut ctx),
            libc::SYS_writev => SyscallHandlerFn::call(Self::writev, &mut ctx),

            // ADDED - JH
            // libc::SYS_msgget => SyscallHandlerFn::call(Self::msgget, &mut ctx),
            //
            _ => {
                // if we added a HANDLE_RUST() macro for this syscall in
                // 'syscallhandler_make_syscall()' but didn't add an entry here, we should get a
                // warning
                log::warn!("Rust syscall {} is not mapped", ctx.args.number);
                Err(Errno::ENOSYS.into())
            }
        }
    }

    /// Internal helper that returns the `Descriptor` for the fd if it exists, otherwise returns
    /// EBADF.
    fn get_descriptor(
        descriptor_table: &DescriptorTable,
        fd: impl TryInto<DescriptorHandle>,
    ) -> Result<&Descriptor, nix::errno::Errno> {
        // check that fd is within bounds
        let fd = fd.try_into().or(Err(nix::errno::Errno::EBADF))?;

        match descriptor_table.get(fd) {
            Some(desc) => Ok(desc),
            None => Err(nix::errno::Errno::EBADF),
        }
    }

    /// Internal helper that returns the `Descriptor` for the fd if it exists, otherwise returns
    /// EBADF.
    fn get_descriptor_mut(
        descriptor_table: &mut DescriptorTable,
        fd: impl TryInto<DescriptorHandle>,
    ) -> Result<&mut Descriptor, nix::errno::Errno> {
        // check that fd is within bounds
        let fd = fd.try_into().or(Err(nix::errno::Errno::EBADF))?;

        match descriptor_table.get_mut(fd) {
            Some(desc) => Ok(desc),
            None => Err(nix::errno::Errno::EBADF),
        }
    }

    /// Run a legacy C syscall handler.
    fn legacy_syscall(syscall: LegacySyscallFn, ctx: &mut SyscallContext) -> SyscallResult {
        unsafe { syscall(ctx.objs.thread.csyscallhandler(), ctx.args as *const _) }.into()
    }
}

pub struct SyscallContext<'a, 'b> {
    pub objs: &'a mut ThreadContext<'b>,
    pub args: &'a SysCallArgs,
}

pub trait SyscallHandlerFn<T> {
    fn call(self, ctx: &mut SyscallContext) -> SyscallResult;
}

impl<F, T0> SyscallHandlerFn<()> for F
where
    F: Fn(&mut SyscallContext) -> Result<T0, SyscallError>,
    T0: Into<SysCallReg>,
{
    fn call(self, ctx: &mut SyscallContext) -> SyscallResult {
        self(ctx).map(Into::into)
    }
}

impl<F, T0, T1> SyscallHandlerFn<(T1,)> for F
where
    F: Fn(&mut SyscallContext, T1) -> Result<T0, SyscallError>,
    T0: Into<SysCallReg>,
    T1: From<SysCallReg>,
{
    fn call(self, ctx: &mut SyscallContext) -> SyscallResult {
        self(ctx, ctx.args.get(0).into()).map(Into::into)
    }
}

impl<F, T0, T1, T2> SyscallHandlerFn<(T1, T2)> for F
where
    F: Fn(&mut SyscallContext, T1, T2) -> Result<T0, SyscallError>,
    T0: Into<SysCallReg>,
    T1: From<SysCallReg>,
    T2: From<SysCallReg>,
{
    fn call(self, ctx: &mut SyscallContext) -> SyscallResult {
        self(ctx, ctx.args.get(0).into(), ctx.args.get(1).into()).map(Into::into)
    }
}

impl<F, T0, T1, T2, T3> SyscallHandlerFn<(T1, T2, T3)> for F
where
    F: Fn(&mut SyscallContext, T1, T2, T3) -> Result<T0, SyscallError>,
    T0: Into<SysCallReg>,
    T1: From<SysCallReg>,
    T2: From<SysCallReg>,
    T3: From<SysCallReg>,
{
    fn call(self, ctx: &mut SyscallContext) -> SyscallResult {
        self(
            ctx,
            ctx.args.get(0).into(),
            ctx.args.get(1).into(),
            ctx.args.get(2).into(),
        )
        .map(Into::into)
    }
}

impl<F, T0, T1, T2, T3, T4> SyscallHandlerFn<(T1, T2, T3, T4)> for F
where
    F: Fn(&mut SyscallContext, T1, T2, T3, T4) -> Result<T0, SyscallError>,
    T0: Into<SysCallReg>,
    T1: From<SysCallReg>,
    T2: From<SysCallReg>,
    T3: From<SysCallReg>,
    T4: From<SysCallReg>,
{
    fn call(self, ctx: &mut SyscallContext) -> SyscallResult {
        self(
            ctx,
            ctx.args.get(0).into(),
            ctx.args.get(1).into(),
            ctx.args.get(2).into(),
            ctx.args.get(3).into(),
        )
        .map(Into::into)
    }
}

impl<F, T0, T1, T2, T3, T4, T5> SyscallHandlerFn<(T1, T2, T3, T4, T5)> for F
where
    F: Fn(&mut SyscallContext, T1, T2, T3, T4, T5) -> Result<T0, SyscallError>,
    T0: Into<SysCallReg>,
    T1: From<SysCallReg>,
    T2: From<SysCallReg>,
    T3: From<SysCallReg>,
    T4: From<SysCallReg>,
    T5: From<SysCallReg>,
{
    fn call(self, ctx: &mut SyscallContext) -> SyscallResult {
        self(
            ctx,
            ctx.args.get(0).into(),
            ctx.args.get(1).into(),
            ctx.args.get(2).into(),
            ctx.args.get(3).into(),
            ctx.args.get(4).into(),
        )
        .map(Into::into)
    }
}

impl<F, T0, T1, T2, T3, T4, T5, T6> SyscallHandlerFn<(T1, T2, T3, T4, T5, T6)> for F
where
    F: Fn(&mut SyscallContext, T1, T2, T3, T4, T5, T6) -> Result<T0, SyscallError>,
    T0: Into<SysCallReg>,
    T1: From<SysCallReg>,
    T2: From<SysCallReg>,
    T3: From<SysCallReg>,
    T4: From<SysCallReg>,
    T5: From<SysCallReg>,
    T6: From<SysCallReg>,
{
    fn call(self, ctx: &mut SyscallContext) -> SyscallResult {
        self(
            ctx,
            ctx.args.get(0).into(),
            ctx.args.get(1).into(),
            ctx.args.get(2).into(),
            ctx.args.get(3).into(),
            ctx.args.get(4).into(),
            ctx.args.get(5).into(),
        )
        .map(Into::into)
    }
}

mod export {
    use shadow_shim_helper_rs::notnull::*;

    use super::*;
    use crate::core::worker::Worker;

    #[no_mangle]
    pub extern "C" fn rustsyscallhandler_new() -> *mut SyscallHandler {
        Box::into_raw(Box::new(SyscallHandler::new()))
    }

    #[no_mangle]
    pub extern "C" fn rustsyscallhandler_free(handler_ptr: *mut SyscallHandler) {
        if handler_ptr.is_null() {
            return;
        }
        unsafe { Box::from_raw(handler_ptr) };
    }

    #[no_mangle]
    pub extern "C" fn rustsyscallhandler_syscall(
        sys: *mut SyscallHandler,
        csys: *mut c::SysCallHandler,
        args: *const SysCallArgs,
    ) -> SyscallReturn {
        assert!(!sys.is_null());
        let sys = unsafe { &mut *sys };
        Worker::with_active_host(|host| {
            let mut objs =
                unsafe { ThreadContextObjs::from_syscallhandler(host, notnull_mut_debug(csys)) };
            objs.with_ctx(|ctx| {
                let ctx = SyscallContext {
                    objs: ctx,
                    args: unsafe { args.as_ref().unwrap() },
                };
                sys.syscall(ctx).into()
            })
        })
        .unwrap()
    }
}
