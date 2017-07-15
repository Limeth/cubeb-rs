use ffi;
use std::mem;

/// Documentation of the variants specifies the channel layout
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Layout {
    /// Indicate the speaker's layout is undefined.
    Undefined,
    /// Left, Right
    DualMono,
    /// Left, Right, Low Frequency Effects
    DualMonoLFE,
    /// Mono
    Mono,
    /// Mono, Low Frequency Effects
    MonoLFE,
    /// Left, Right
    Stereo,
    /// Left, Right, Low Frequency Effects
    StereoLFE,
    /// Left, Right, Center
    Front3,
    /// Left, Right, Center, Low Frequency Effects
    Front3LFE,
    /// Left, Right, Surround
    Front2Surround1,
    /// Left, Right, Low Frequency Effects, Surround
    Front2Surround1LFE,
    /// Left, Right, Center, Surround
    Front3Surround1,
    /// Left, Right, Center, Low Frequency Effects, Surround
    Front3Surround1LFE,
    /// Left, Right, Left Surround, Right Surround
    Front2Surround2,
    /// Left, Right, Low Frequency Effects, Left Surround, Right Surround
    Front2Surround2LFE,
    /// Left, Right, Center, Left Surround, Right Surround
    Front3Surround2,
    /// Left, Right, Center, Low Frequency Effects, Left Surround, Right Surround
    Front3Surround2LFE,
    /// Left, Right, Center, Low Frequency Effects, Rear Center, Left Surround, Right Surround
    Front3Surround3LFE,
    /// Left, Right, Center, Low Frequency Effects, Rear Left Surround, Rear Right Surround, Left Surround, Right Surround
    Front3Surround4LFE,
}

impl From<ffi::cubeb_channel_layout> for Layout {
    fn from(other: ffi::cubeb_channel_layout) -> Self {
        unsafe { mem::transmute(other) }
    }
}

impl From<Layout> for ffi::cubeb_channel_layout {
    fn from(other: Layout) -> Self {
        unsafe { mem::transmute(other) }
    }
}
