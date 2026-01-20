/// Represents a physical display monitor.
#[derive(Clone, Debug)]
pub struct MonitorInfo {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub refresh_rate: f32,
    pub scale_factor: f64,
}

/// Provides access to system information.
pub struct SystemContext {
    pub(crate) gpu_name: Option<String>,
    pub(crate) backend: Option<String>,
    pub(crate) frame_count: u32,
    pub(crate) monitors: Vec<MonitorInfo>,
}

impl SystemContext {
    /// Get the name of the operating system (e.g., "linux", "windows", "macos").
    pub fn os(&self) -> &'static str {
        std::env::consts::OS
    }

    /// Get the architecture of the CPU (e.g., "x86_64", "aarch64").
    pub fn arch(&self) -> &'static str {
        std::env::consts::ARCH
    }

    /// Get the family of the operating system (e.g., "unix", "windows").
    pub fn family(&self) -> &'static str {
        std::env::consts::FAMILY
    }

    /// Get the number of logical CPU cores available.
    /// Returns 1 if the value cannot be determined.
    pub fn cores(&self) -> usize {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
    }

    /// Get the name of the active GPU (e.g., "NVIDIA GeForce RTX 3080").
    /// Returns "Unknown" if the renderer is not active.
    pub fn gpu_name(&self) -> String {
        self.gpu_name.clone().unwrap_or_else(|| "Unknown".to_string())
    }

    /// Get the graphics backend being used (e.g., "Vulkan", "Metal", "Dx12").
    /// Returns "Unknown" if the renderer is not active.
    pub fn backend(&self) -> String {
        self.backend.clone().unwrap_or_else(|| "Unknown".to_string())
    }

    /// Get the total number of frames rendered since the app started.
    pub fn frame_count(&self) -> u32 {
        self.frame_count
    }

    /// Get a list of all detected monitors.
    pub fn monitors(&self) -> &[MonitorInfo] {
        &self.monitors
    }

    /// Get the resolution of the primary monitor as a tuple (width, height).
    /// Returns (0,0) if no monitors are detected.
    pub fn primary_resolution(&self) -> (u32, u32) {
        self.monitors.first()
            .map(|m| (m.width, m.height))
            .unwrap_or((0, 0))
    }
}