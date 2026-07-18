//! Audio abstraction placeholder.
//!
//! The original QBasic game used `PLAY` strings for short tunes/effects. Audio
//! output is intentionally out of scope for the first playable local port, but
//! gameplay code calls this abstraction at the same event boundaries so a later
//! pass can map the original effects without changing turn or rendering logic.

#[derive(Default)]
pub struct Audio;

impl Audio {
    pub fn new() -> Self {
        Self
    }

    /// Original intro tune: `MBT160O1L8CDEDCDL4ECC`.
    pub fn play_intro(&self) {
        // No-op until audio output is in scope.
    }

    /// Original banana throw effect: `MBo0L32A-L64CL16BL64A+`.
    pub fn play_throw(&self) {
        // No-op until audio output is in scope.
    }

    /// Original generic explosion effect: `MBO0L32EFGEFDC`.
    pub fn play_explosion(&self) {
        // No-op until audio output is in scope.
    }

    /// Original direct-hit gorilla explosion effect: `MBO0L16EFGEFDC`.
    pub fn play_gorilla_explosion(&self) {
        // No-op until audio output is in scope.
    }

    /// Original victory-dance effect repeated during the dance: `MFO0L32EFGEFDC`.
    pub fn play_victory(&self) {
        // No-op until audio output is in scope.
    }
}
