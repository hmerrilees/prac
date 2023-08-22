use anyhow::{Context, Result};
use dialoguer::Editor;

/// Write content to file
pub fn long_edit(intitial_content: Option<&str>) -> Result<String> {
    Editor::new()
        .require_save(false)
        .edit(intitial_content.unwrap_or_default())?
        .context("Content not saved")
}

/// generate a bar for a practice
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
pub fn bar(bar_width: usize, mut fraction: f64) -> String {
    fraction = fraction.clamp(0.0, 1.0);
    //.with_context(|| "fraction must be between 0 and 1")?;
    let filled = (fraction * bar_width as f64) as usize;
    let empty = bar_width - filled;
    assert!(filled + empty == bar_width);
    format!("{}{}", "\u{025AC}".repeat(filled), " ".repeat(empty))
}
