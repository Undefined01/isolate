use nix::errno::Errno;
use nix::libc;
use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;
use std::time::Duration;

pub fn wait4(pid: Pid) -> Result<(libc::c_int, libc::rusage), Errno> {
    unsafe {
        let mut status = 0;
        let mut rus = std::mem::MaybeUninit::uninit().assume_init();
        let res = libc::wait4(
            pid.into(),
            &mut status as *mut libc::c_int,
            0,
            &mut rus as *mut libc::rusage,
        );
        if res < 0 {
            Err(Errno::last())
        } else {
            Ok((status, rus))
        }
    }
}

mod libc_ext {
    use nix::libc;
    extern "C" {
        pub fn setitimer(
            which: libc::c_int,
            new_value: *const libc::itimerval,
            old_value: *mut libc::itimerval,
        ) -> libc::c_int;
    }
}

pub fn setitimer(which: libc::c_int, interval: &Duration, value: &Duration) -> Result<(), Errno> {
    let new_value = libc::itimerval {
        it_interval: libc::timeval {
            tv_sec: interval.as_secs() as i64,
            tv_usec: interval.subsec_micros() as i64,
        },
        it_value: libc::timeval {
            tv_sec: value.as_secs() as i64,
            tv_usec: value.subsec_micros() as i64,
        },
    };
    let res = unsafe {
        libc_ext::setitimer(
            which,
            &new_value as *const _,
            std::ptr::null::<libc::itimerval>() as *mut _,
        )
    };
    if res < 0 {
        Err(Errno::last())
    } else {
        Ok(())
    }
}

pub struct ProcessKeeper(Pid);

impl ProcessKeeper {
    pub fn new(pid: Pid) -> Self {
        Self(pid)
    }

    pub fn wait4(&self) -> Result<(libc::c_int, libc::rusage), Errno> {
        wait4(self.0)
    }

    pub fn kill(&self) {
        debug!("Killing process {:?}", self.0);
        let pgid = Pid::from_raw(-self.0.as_raw());
        if let Err(e) = kill(pgid, Signal::SIGKILL) {
            info!("Failed to kill {:?}: {:?}", pgid, e);
        }
        if let Err(e) = kill(self.0, Signal::SIGKILL) {
            info!("Failed to kill {:?}: {:?}", self.0, e);
        }
    }
}

impl Drop for ProcessKeeper {
    fn drop(&mut self) {
        self.kill()
    }
}
