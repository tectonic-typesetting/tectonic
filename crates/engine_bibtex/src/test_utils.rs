use tectonic_bridge_core::{CoreBridgeLauncher, CoreBridgeState, MinimalDriver};
use tectonic_io_base::IoProvider;
use tectonic_io_base::stack::IoStack;
use tectonic_status_base::NoopStatusBackend;

// TODO: Create context without backend? Use custom backend-like type?
//       Implement the relevant interfaces ourself?
pub(crate) fn with_cbs(f: impl FnOnce(&mut CoreBridgeState<'_>)) {
    let io_list: Vec<&mut dyn IoProvider> = vec![];
    let io = IoStack::new(io_list);
    let mut hooks = MinimalDriver::new(io);
    let mut status = NoopStatusBackend::default();
    let mut cbl = CoreBridgeLauncher::new(&mut hooks, &mut status);
    cbl.with_global_lock(|cbs| {
        f(cbs);
        Ok(())
    })
        .unwrap();
}
