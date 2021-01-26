use nix::libc;
use nix::unistd::Pid;

pub fn wait4(pid: Pid) -> (libc::c_int, libc::rusage) {
    unsafe {
        let mut status = 0;
        let mut rus = std::mem::MaybeUninit::uninit().assume_init();
        libc::wait4(
            pid.into(),
            &mut status as *mut libc::c_int,
            0,
            &mut rus as *mut libc::rusage,
        );
        (status, rus)
    }
}
