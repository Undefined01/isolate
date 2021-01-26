mod proxy;

use crate::config::Config;

use crate::sys::wait4;
use nix::libc;
use nix::sched::clone;
use nix::sched::CloneFlags;

pub fn run(config: &Config) -> Result<(), ()> {
    config
        .cg
        .reset()
        .expect("Failed to reset control group usages");

    let mut stack = vec![0u8; config.rlim.stack_size.0 as usize * 1024];
    let res = clone(
        Box::new(|| proxy::entry(config)),
        stack.as_mut(),
        CloneFlags::CLONE_NEWPID
            | CloneFlags::CLONE_NEWUTS
            | CloneFlags::CLONE_NEWIPC
            | CloneFlags::CLONE_NEWNS
            | CloneFlags::CLONE_NEWNET,
        Some(libc::SIGCHLD),
    )
    .map_err(|e| info!("Failed to clone proxy process: {:?}", e))?;

    let (status, rus) = wait4(res);
    let cpu_usage = config.cg.cpu_usage().expect("Failed to get cpu usage");
    let mem_usage = config.cg.mem_usage().expect("Failed to get memory usage");
    println!(
        "Execute result: {}\nCgroup: {:?} {:?}\n{:#?}",
        status, cpu_usage, mem_usage, rus
    );

    Ok(())
}
