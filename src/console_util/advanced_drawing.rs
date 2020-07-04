use super::console::Console;
use amethyst::renderer::palette::Srgba;

/// Draws list of entries to console, each under the previous. Centers around the longest
/// entry, top left corner is (x,y). Current selection is highlighted using the
/// provided color, everything is non selection color. Background is black.
pub fn selection_box(
    console: &mut Console,
    x: u32,
    y: u32,
    entries: Vec<String>,
    curr_sel: u32,
    non_sel_col: Srgba,
    sel_col: Srgba,
) {
    let mut idy: u32 = 0; // Current y index
    let bg = Srgba::new(0.0, 0.0, 0.0, 1.0); // Default black background

    // Find length of longest entry, and halve to find center
    let x_center: u32 = {
        let mut longest_str_len = 0;
        for entry in &entries {
            if entry.len() > longest_str_len {
                longest_str_len = entry.len();
            }
        }

        (longest_str_len / 2) as u32
    };

    for entry in &entries {
        if idy == curr_sel {
            console.print_str_cl(x - x_center, y + idy, &entry, sel_col, bg);
        } else {
            console.print_str_cl(x - x_center, y + idy, &entry, non_sel_col, bg);
        }

        idy += 1;
    }
}
