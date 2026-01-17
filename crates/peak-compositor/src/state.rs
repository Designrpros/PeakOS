#![cfg(target_os = "linux")]
use smithay::{
    delegate_compositor, delegate_data_device, delegate_layer_shell, delegate_output,
    delegate_seat, delegate_shm, delegate_xdg_shell,
    desktop::{layer_map_for_output, Space, Window},
    reexports::wayland_server::{protocol::wl_surface::WlSurface, Client, DisplayHandle},
    wayland::{
        compositor::{ClientCompositorState, CompositorHandler, CompositorState},
        data_device::{DataDeviceHandler, DataDeviceState},
        output::{OutputHandler, OutputState},
        seat::{CursorImageStatus, Seat, SeatHandler, SeatState},
        shell::{
            layer::{LayerShellHandler, LayerShellState, LayerSurface},
            xdg::{PopupSurface, ToplevelSurface, XdgShellHandler, XdgShellState},
        },
        shm::{ShmHandler, ShmState},
    },
};

pub struct PeakCompositor {
    pub display_handle: DisplayHandle,
    pub space: Space<Window>,

    // Smithay States
    pub compositor_state: CompositorState,
    pub shm_state: ShmState,
    pub output_state: OutputState,
    pub seat_state: SeatState<Self>,
    pub data_device_state: DataDeviceState,
    pub xdg_shell_state: XdgShellState,
    pub layer_shell_state: LayerShellState,
}

impl PeakCompositor {
    pub fn new(display_handle: DisplayHandle) -> Self {
        let compositor_state = CompositorState::new::<Self>(&display_handle);
        let shm_state = ShmState::new::<Self>(&display_handle, Vec::new());
        let output_state = OutputState::new();
        let seat_state = SeatState::new::<Self>(&display_handle, "peak-seat");
        let data_device_state = DataDeviceState::new::<Self>(&display_handle);
        let xdg_shell_state = XdgShellState::new::<Self>(&display_handle);
        let layer_shell_state = LayerShellState::new::<Self>(&display_handle);
        let space = smithay::desktop::Space::default();

        Self {
            display_handle,
            space,
            compositor_state,
            shm_state,
            output_state,
            seat_state,
            data_device_state,
            xdg_shell_state,
            layer_shell_state,
        }
    }
}

// Implement required delegates
delegate_compositor!(PeakCompositor);
delegate_shm!(PeakCompositor);
delegate_output!(PeakCompositor);
delegate_seat!(PeakCompositor);
delegate_data_device!(PeakCompositor);
delegate_xdg_shell!(PeakCompositor);
delegate_layer_shell!(PeakCompositor);

impl CompositorHandler for PeakCompositor {
    fn compositor_state(&mut self) -> &mut CompositorState {
        &mut self.compositor_state
    }

    fn client_compositor_state<'a>(&self, _client: &'a Client) -> &'a ClientCompositorState {
        &ClientCompositorState::default()
    }

    fn commit(&mut self, _surface: &WlSurface) {
        // Handle surface commit
    }
}

impl ShmHandler for PeakCompositor {
    fn shm_state(&mut self) -> &mut ShmState {
        &mut self.shm_state
    }
}

impl SeatHandler for PeakCompositor {
    type KeyboardFocus = WlSurface;
    type PointerFocus = WlSurface;

    fn seat_state(&mut self) -> &mut SeatState<Self> {
        &mut self.seat_state
    }

    fn focus_changed(&mut self, _seat: &Seat<Self>, _focused: Option<&Self::KeyboardFocus>) {
        // Handle focus change
    }

    fn cursor_image(&mut self, _seat: &Seat<Self>, _image: CursorImageStatus) {
        // Handle cursor image change
    }
}

impl DataDeviceHandler for PeakCompositor {
    fn data_device_state(&mut self) -> &mut DataDeviceState {
        &mut self.data_device_state
    }
}

impl OutputHandler for PeakCompositor {}

impl XdgShellHandler for PeakCompositor {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState {
        &mut self.xdg_shell_state
    }

    fn new_toplevel(&mut self, surface: ToplevelSurface) {
        let window = Window::new_wayland_window(surface);
        self.space.map_element(window, (0, 0), true);
    }

    fn new_popup(&mut self, _surface: PopupSurface) {
        // Handle new popup
    }

    fn grab(
        &mut self,
        _surface: PopupSurface,
        _seat: smithay::reexports::wayland_server::protocol::wl_seat::WlSeat,
        _serial: smithay::utils::Serial,
    ) {
        // Handle grab
    }
}

impl LayerShellHandler for PeakCompositor {
    fn layer_shell_state(&mut self) -> &mut LayerShellState {
        &mut self.layer_shell_state
    }

    fn new_layer_surface(
        &mut self,
        surface: LayerSurface,
        output: Option<smithay::reexports::wayland_server::protocol::wl_output::WlOutput>,
        _layer: smithay::wayland::shell::layer::Layer,
        _namespace: String,
    ) {
        let output = output
            .and_then(|o| smithay::wayland::output::Output::from_resource(&o))
            .unwrap_or_else(|| {
                self.output_state
                    .outputs()
                    .next()
                    .expect("No output available")
                    .clone()
            });

        let mut map = layer_map_for_output(&output);
        map.add_layer(surface);
    }
}
