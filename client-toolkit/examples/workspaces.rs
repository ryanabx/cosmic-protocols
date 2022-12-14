use cosmic_client_toolkit::workspace::{WorkspaceHandler, WorkspaceState};
use sctk::{
    output::{OutputHandler, OutputState},
    registry::{ProvidesRegistryState, RegistryState},
};
use wayland_client::{globals::registry_queue_init, protocol::wl_output, Connection, QueueHandle};

struct AppData {
    output_state: OutputState,
    registry_state: RegistryState,
    workspace_state: WorkspaceState,
}

impl ProvidesRegistryState for AppData {
    fn registry(&mut self) -> &mut RegistryState {
        &mut self.registry_state
    }

    sctk::registry_handlers!(OutputState);
}

// Need to bind output globals just so workspace can get output events
impl OutputHandler for AppData {
    fn output_state(&mut self) -> &mut OutputState {
        &mut self.output_state
    }

    fn new_output(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        output: wl_output::WlOutput,
    ) {
    }

    fn update_output(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _output: wl_output::WlOutput,
    ) {
    }

    fn output_destroyed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        output: wl_output::WlOutput,
    ) {
    }
}

impl WorkspaceHandler for AppData {
    fn workspace_state(&mut self) -> &mut WorkspaceState {
        &mut self.workspace_state
    }

    fn done(&mut self) {
        for group in self.workspace_state.workspace_groups() {
            let output_name = group.output.as_ref().and_then(|output| {
                let info = self.output_state.info(output).unwrap();
                info.name.clone()
            });
            println!(
                "Group: output: {}, capabilities: {:?}",
                output_name.as_deref().unwrap_or("None"),
                group.capabilities,
            );
            for workspace in &group.workspaces {
                println!(
                    "  Workspace: name: {}, coordinates: {:?}, capabilties: {:?}, state: {:?}",
                    &workspace.name,
                    &workspace.coordinates,
                    &workspace.capabilities,
                    &workspace.state
                );
            }
        }
    }
}

fn main() {
    let conn = Connection::connect_to_env().unwrap();
    let (globals, mut event_queue) = registry_queue_init(&conn).unwrap();
    let qh = event_queue.handle();

    let registry_state = RegistryState::new(&globals);
    let mut app_data = AppData {
        output_state: OutputState::new(&globals, &qh),
        workspace_state: WorkspaceState::new(&registry_state, &qh),
        registry_state,
    };

    loop {
        event_queue.blocking_dispatch(&mut app_data).unwrap();
    }
}

sctk::delegate_output!(AppData);
sctk::delegate_registry!(AppData);
cosmic_client_toolkit::delegate_workspace!(AppData);
