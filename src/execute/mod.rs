mod proxy;

use crate::config::Config;
use crate::unit;

use crate::sys::{setitimer, ProcessKeeper};
use nix::errno::Errno;
use nix::libc;
use nix::sched::clone;
use nix::sched::CloneFlags;
use nix::sys::signal::{sigaction, SaFlags, SigAction, SigHandler, SigSet, Signal};
use std::convert::TryInto;
use std::time::{Duration, Instant};

pub fn run(config: &Config) -> Result<(), ()> {
    config
        .cg
        .reset()
        .map_err(|_| info!("Failed to reset control group usages"))?;

    let mut stack = vec![0u8; config.rlim.stack_size.byte() as usize];
    let proxy_pid = clone(
        Box::new(|| proxy::entry(config)),
        stack.as_mut(),
        CloneFlags::CLONE_NEWUTS
            | CloneFlags::CLONE_NEWIPC
            | CloneFlags::CLONE_NEWNS
            | CloneFlags::CLONE_NEWNET,
        Some(libc::SIGCHLD),
    )
    .map_err(|e| info!("Failed to clone proxy process: {:?}", e))?;
    let proxy_keeper = ProcessKeeper::new(proxy_pid);

    if config.rlim.cpu_limit.is_some() {
        unsafe {
            extern "C" fn alarm_handler(_: libc::c_int) {}
            sigaction(
                Signal::SIGALRM,
                &SigAction::new(
                    SigHandler::Handler(alarm_handler),
                    SaFlags::empty(),
                    SigSet::empty(),
                ),
            )
            .map_err(|e| info!("Failed to set signal handler for SIGALRM: {:?}", e))?;
        }
        setitimer(
            libc::ITIMER_REAL,
            &Duration::from_millis(100),
            &Duration::from_millis(100),
        )
        .map_err(|e| info!("Failed to set itimer: {:?}", e))?;
    }

    let timer = Instant::now();
    let (status, rus) = loop {
        let res = proxy_keeper.wait4();
        match res {
            Err(Errno::EINTR) => {
                let real_time: unit::ms = timer
                    .elapsed()
                    .try_into()
                    .map_err(|e| info!("Failed to get real running time: {:?}", e))?;
                debug!("Real time {:?}", real_time);
                if &real_time > config.rlim.cpu_limit.as_ref().unwrap() {
                    info!("Real time limit exceeded");
                    proxy_keeper.kill();
                }
            }
            Ok(res) => break res,
            Err(e) => {
                info!("Failed to wait4 proxy process: {:?}", e);
                return Err(());
            }
        }
    };
    let real_time: unit::ms = timer
        .elapsed()
        .try_into()
        .map_err(|e| info!("Failed to get real running time: {:?}", e))?;
    let cpu_usage = config
        .cg
        .cpu_usage()
        .map_err(|_| info!("Failed to get cpu usage"))?;
    let mem_usage = config
        .cg
        .mem_usage()
        .map_err(|_| info!("Failed to get memory usage"))?;

    info!(
        "Execute result: {}\nCgroup: {:?} {:?}\nReal time: {:?}\n{:#?}",
        status, cpu_usage, mem_usage, real_time, rus
    );

    Ok(())
}
