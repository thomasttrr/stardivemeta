//! mongil-render: SVG -> PNG rendering (plan T3.9) for the official
//! tier-list images and the OG image.
//!
//! An SVG string is built here from plain view structs (no dependency
//! on the method engine), then rendered with usvg/resvg using ONLY the
//! bundled Inter variable font (`fonts/Inter-Variable.ttf`, OFL 1.1) —
//! the fontdb starts empty, so no system font can leak in and change
//! the output. Names only, no game art (plan §B.5 PNG).
//!
//! Colours are the DESIGN.md tokens (the same values as
//! `site/src/styles/tokens.css`).

use std::fmt;
use std::sync::Arc;

// DESIGN.md palette.
const BG: &str = "#10131a";
const PANEL_2: &str = "#1f2637";
const LINE: &str = "#2c3549";
const TEXT: &str = "#f2f4f8";
const SUB: &str = "#aeb8c9";
const TEAL: &str = "#3fd0c9";
const ON_TIER: &str = "#12141c";
const ON_TEAL: &str = "#0b2b29";
const ON_GOLD: &str = "#221a02";

/// Tier rail colours + the text colour that sits on them.
fn tier_colors(tier: &str) -> (&'static str, &'static str) {
    match tier {
        "S" => ("#ffd166", ON_GOLD),
        "A" => ("#ef6aa5", ON_TIER),
        "B" => ("#3fd0c9", ON_TEAL),
        "C" => ("#9d8cff", ON_TIER),
        "D" => ("#8d99ae", ON_TIER),
        "MODE_SPLIT" => ("#7bd88f", ON_TIER),
        _ => ("#8d99ae", ON_TIER), // UNRANKED
    }
}

/// Label glyph colours in the PNG (the glyph shape differs per label
/// too — colour is never the only signal).
fn glyph_color(label: &str) -> &'static str {
    match label {
        "SOLID" => "#3fd0c9",
        "CONTESTED" => "#ef6aa5",
        "SPECULATION" => "#8d99ae",
        "MODE_SPLIT" => "#7bd88f",
        _ => SUB,
    }
}

/// One character chip: display name + confidence-label glyph.
#[derive(Clone, Debug, PartialEq)]
pub struct ChipView {
    pub name: String,
    /// Contract label: SOLID / CONTESTED / SPECULATION / MODE_SPLIT / UNRANKED.
    pub label: String,
    /// ● / ◐ / ○ / ⇄ / —.
    pub glyph: String,
}

/// One tier row of a board (only non-empty rows are rendered).
#[derive(Clone, Debug, PartialEq)]
pub struct RowView {
    /// "S" | "A" | "B" | "C" | "D" | "MODE_SPLIT" | "UNRANKED".
    pub tier: String,
    pub chips: Vec<ChipView>,
}

/// Everything a per-level PNG needs.
#[derive(Clone, Debug, PartialEq)]
pub struct BoardView {
    /// "Low" | "High" | "Max".
    pub level_name: String,
    pub method_version: String,
    /// Pre-formatted check date, e.g. "Oct 29, 2026".
    pub date_text: String,
    pub rows: Vec<RowView>,
}

#[derive(Debug)]
pub struct RenderError(pub String);

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "render: {}", self.0)
    }
}
impl std::error::Error for RenderError {}

fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

pub const WIDTH: u32 = 1200;
const MARGIN: f64 = 48.0;
const ROW_H: f64 = 120.0;
const HEADER_H: f64 = 124.0;
const CHIP_H: f64 = 44.0;
const CHIP_GAP: f64 = 10.0;
const BADGE_W: f64 = 84.0;
const CHIPS_X: f64 = MARGIN + BADGE_W + 16.0;

/// PNG height for a board with `rows` rendered rows (plan T3.9):
/// 220 + 120 per row, capped at 2000.
pub fn board_height(rows: usize) -> u32 {
    (220 + 120 * rows as u32).min(2000)
}

/// Estimated chip width at 19px Inter semibold. Deliberately generous
/// (0.56em average advance + padding + glyph) so the flow layout never
/// under-allocates; the layout test pins the real boards to 2 lines.
fn chip_width(chip: &ChipView) -> f64 {
    30.0 + 26.0 + chip.name.chars().count() as f64 * 10.6
}

/// Lay chips out in lines inside the chips area; returns one Vec of
/// (chip, x offset) per line.
fn flow(chips: &[ChipView], area_w: f64) -> Vec<Vec<(&ChipView, f64)>> {
    let mut lines: Vec<Vec<(&ChipView, f64)>> = Vec::new();
    let mut line: Vec<(&ChipView, f64)> = Vec::new();
    let mut x = 0.0;
    for chip in chips {
        let w = chip_width(chip).min(area_w);
        if !line.is_empty() && x + w > area_w {
            lines.push(std::mem::take(&mut line));
            x = 0.0;
        }
        line.push((chip, x));
        x += w + CHIP_GAP;
    }
    if !line.is_empty() {
        lines.push(line);
    }
    lines
}

/// The label glyph drawn as vector shapes (Inter has no ◐/⇄ glyphs,
/// so font text would render as tofu). Centered at (cx, cy); `r` is
/// the circle radius / half-size of the mark.
fn glyph_shapes(label: &str, cx: f64, cy: f64, r: f64, color: &str) -> String {
    match label {
        "SOLID" => format!("<circle cx=\"{cx:.1}\" cy=\"{cy:.1}\" r=\"{r:.1}\" fill=\"{color}\"/>"),
        "CONTESTED" => format!(
            "<circle cx=\"{cx:.1}\" cy=\"{cy:.1}\" r=\"{r:.1}\" fill=\"none\" stroke=\"{color}\" stroke-width=\"1.6\"/>\
             <path d=\"M {cx:.1} {t:.1} A {r:.1} {r:.1} 0 0 0 {cx:.1} {b:.1} Z\" fill=\"{color}\"/>",
            t = cy - r,
            b = cy + r
        ),
        "SPECULATION" => format!(
            "<circle cx=\"{cx:.1}\" cy=\"{cy:.1}\" r=\"{r:.1}\" fill=\"none\" stroke=\"{color}\" stroke-width=\"1.6\"/>"
        ),
        "MODE_SPLIT" => {
            let w = r * 1.25;
            let dy = r * 0.52;
            let ah = r * 0.62; // arrowhead size
            format!(
                "<path d=\"M {l:.1} {t:.1} H {rgt:.1} M {rgt:.1} {t:.1} l -{ah:.1} -{ah2:.1} M {rgt:.1} {t:.1} l -{ah:.1} {ah2:.1}\" stroke=\"{color}\" stroke-width=\"{sw:.1}\" fill=\"none\"/>\
                 <path d=\"M {rgt:.1} {b:.1} H {l:.1} M {l:.1} {b:.1} l {ah:.1} -{ah2:.1} M {l:.1} {b:.1} l {ah:.1} {ah2:.1}\" stroke=\"{color}\" stroke-width=\"{sw:.1}\" fill=\"none\"/>",
                l = cx - w,
                rgt = cx + w,
                t = cy - dy,
                b = cy + dy,
                ah2 = ah * 0.62,
                sw = (r * 0.24).max(1.4)
            )
        }
        _ => format!(
            "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{color}\"/>",
            cx - r,
            cy - r * 0.14,
            r * 2.0,
            r * 0.28
        ),
    }
}

/// Render one tier row band whose top edge is `y` (band height ROW_H).
fn row_svg(row: &RowView, y: f64) -> String {
    let (fill, on) = tier_colors(&row.tier);
    let mut s = String::new();
    // Badge rail.
    s.push_str(&format!(
        "<rect x=\"{MARGIN}\" y=\"{:.0}\" width=\"{BADGE_W}\" height=\"96\" rx=\"12\" fill=\"{fill}\"/>",
        y + 12.0
    ));
    if row.tier == "MODE_SPLIT" || row.tier == "UNRANKED" {
        let label = if row.tier == "MODE_SPLIT" {
            "MODE_SPLIT"
        } else {
            "UNRANKED"
        };
        s.push_str(&glyph_shapes(
            label,
            MARGIN + BADGE_W / 2.0,
            y + 12.0 + 36.0,
            15.0,
            on,
        ));
    } else {
        s.push_str(&format!(
            "<text x=\"{:.0}\" y=\"{:.0}\" font-size=\"42\" font-weight=\"900\" fill=\"{on}\" text-anchor=\"middle\">{}</text>",
            MARGIN + BADGE_W / 2.0,
            y + 12.0 + 56.0,
            esc(&row.tier)
        ));
    }
    s.push_str(&format!(
        "<text x=\"{:.0}\" y=\"{:.0}\" font-size=\"13\" font-weight=\"700\" fill=\"{on}\" text-anchor=\"middle\">{} {}</text>",
        MARGIN + BADGE_W / 2.0,
        y + 12.0 + 80.0,
        row.chips.len(),
        if row.chips.len() == 1 { "char" } else { "chars" }
    ));
    // Chips, up to two lines inside the band.
    let area_w = WIDTH as f64 - CHIPS_X - MARGIN;
    let lines = flow(&row.chips, area_w);
    for (i, line) in lines.iter().take(2).enumerate() {
        let cy = y + 14.0 + i as f64 * (CHIP_H + 8.0);
        for (chip, xoff) in line {
            let cx = CHIPS_X + xoff;
            let w = chip_width(chip).min(area_w);
            s.push_str(&format!(
                "<rect x=\"{cx:.0}\" y=\"{cy:.0}\" width=\"{w:.0}\" height=\"{CHIP_H}\" rx=\"10\" fill=\"{PANEL_2}\" stroke=\"{LINE}\"/>"
            ));
            s.push_str(&format!(
                "<text x=\"{:.0}\" y=\"{:.0}\" font-size=\"19\" font-weight=\"600\" fill=\"{TEXT}\">{}</text>",
                cx + 14.0,
                cy + 29.0,
                esc(&chip.name)
            ));
            s.push_str(&glyph_shapes(
                &chip.label,
                cx + w - 19.0,
                cy + CHIP_H / 2.0,
                7.0,
                glyph_color(&chip.label),
            ));
        }
    }
    s
}

fn legend_svg(y: f64) -> String {
    // Two items per line in fixed columns; each item is a vector glyph
    // plus a text segment (§B.5 Home legend wording).
    let items: [(&str, &str, &str); 4] = [
        ("SOLID", "●", "Solid — the lists agree within 1 tier."),
        (
            "CONTESTED",
            "◐",
            "Contested — the lists are more than 1 tier apart.",
        ),
        (
            "SPECULATION",
            "○",
            "One list — just 1 list has placed them so far.",
        ),
        (
            "MODE_SPLIT",
            "⇄",
            "Mode-split — strong in one mode, weak in the other.",
        ),
    ];
    let mut s = String::new();
    for (i, (label, _glyph, text)) in items.iter().enumerate() {
        let col_x = if i % 2 == 0 { MARGIN } else { 628.0 };
        let baseline = y + (i / 2) as f64 * 26.0;
        s.push_str(&glyph_shapes(label, col_x + 8.0, baseline - 5.0, 7.0, SUB));
        s.push_str(&format!(
            "<text x=\"{:.0}\" y=\"{baseline:.0}\" font-size=\"17\" fill=\"{SUB}\">{}</text>",
            col_x + 26.0,
            esc(text)
        ));
    }
    s
}

fn footer_svg(height: u32) -> String {
    format!(
        "<text x=\"{MARGIN}\" y=\"{:.0}\" font-size=\"16\" fill=\"{SUB}\">stardivemeta.com · Fan-made. Not affiliated with Netmarble.</text>",
        height as f64 - 18.0
    )
}

/// Render a per-level board PNG (plan T3.9 + §B.5 PNG contents).
pub fn render_board_png(board: &BoardView) -> Result<Vec<u8>, RenderError> {
    let height = board_height(board.rows.len());
    let mut svg = String::new();
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" font-family=\"Inter\" width=\"{WIDTH}\" height=\"{height}\" viewBox=\"0 0 {WIDTH} {height}\">"
    ));
    svg.push_str(&format!(
        "<rect width=\"{WIDTH}\" height=\"{height}\" fill=\"{BG}\"/>"
    ));
    svg.push_str(&format!(
        "<text x=\"{MARGIN}\" y=\"58\" font-size=\"40\" font-weight=\"800\" fill=\"{TEXT}\">MONGIL: STAR DIVE — combined tier list</text>"
    ));
    svg.push_str(&format!(
        "<text x=\"{MARGIN}\" y=\"92\" font-size=\"21\" fill=\"{SUB}\">Investment: {} · Rules v{} · 3 lists · {}</text>",
        esc(&board.level_name),
        esc(&board.method_version),
        esc(&board.date_text)
    ));
    for (i, row) in board.rows.iter().enumerate() {
        svg.push_str(&row_svg(row, HEADER_H + i as f64 * ROW_H));
    }
    let legend_y = HEADER_H + board.rows.len() as f64 * ROW_H + 28.0;
    svg.push_str(&legend_svg(legend_y));
    svg.push_str(&footer_svg(height));
    svg.push_str("</svg>");
    rasterize(&svg, WIDTH, height)
}

/// Render the OG image (1200×630): S and A rows at Low + the site name
/// + the positioning line (§B.5 PNG).
pub fn render_og_png(low: &BoardView) -> Result<Vec<u8>, RenderError> {
    let height = 630;
    let mut svg = String::new();
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" font-family=\"Inter\" width=\"{WIDTH}\" height=\"{height}\" viewBox=\"0 0 {WIDTH} {height}\">"
    ));
    svg.push_str(&format!(
        "<rect width=\"{WIDTH}\" height=\"{height}\" fill=\"{BG}\"/>"
    ));
    svg.push_str(&format!(
        "<text x=\"{MARGIN}\" y=\"94\" font-size=\"56\" font-weight=\"800\" fill=\"{TEXT}\">StarDive<tspan fill=\"{TEAL}\">Meta</tspan></text>"
    ));
    svg.push_str(&format!(
        "<text x=\"{MARGIN}\" y=\"142\" font-size=\"27\" fill=\"{SUB}\">Every list disagrees. We show you by how much.</text>"
    ));
    svg.push_str(&format!(
        "<text x=\"{MARGIN}\" y=\"180\" font-size=\"20\" fill=\"{SUB}\">MONGIL: STAR DIVE — combined tier list · Low investment · Rules v{}</text>",
        esc(&low.method_version)
    ));
    let mut y = 208.0;
    for tier in ["S", "A"] {
        if let Some(row) = low.rows.iter().find(|r| r.tier == tier) {
            svg.push_str(&row_svg(row, y));
            y += ROW_H;
        }
    }
    svg.push_str(&footer_svg(height));
    svg.push_str("</svg>");
    rasterize(&svg, WIDTH, height)
}

fn rasterize(svg: &str, width: u32, height: u32) -> Result<Vec<u8>, RenderError> {
    use resvg::usvg::{fontdb, Options, Tree};
    let font_path = concat!(env!("CARGO_MANIFEST_DIR"), "/fonts/Inter-Variable.ttf");
    let mut db = fontdb::Database::new();
    db.load_font_file(font_path)
        .map_err(|e| RenderError(format!("loading bundled Inter font: {e}")))?;
    db.set_sans_serif_family("Inter");
    let opt = Options {
        fontdb: Arc::new(db),
        font_family: "Inter".to_string(),
        ..Options::default()
    };
    let tree = Tree::from_str(svg, &opt)
        .map_err(|e| RenderError(format!("parsing generated SVG: {e}")))?;
    let mut pixmap = resvg::tiny_skia::Pixmap::new(width, height)
        .ok_or_else(|| RenderError("allocating pixmap".into()))?;
    resvg::render(
        &tree,
        resvg::tiny_skia::Transform::identity(),
        &mut pixmap.as_mut(),
    );
    pixmap
        .encode_png()
        .map_err(|e| RenderError(format!("encoding PNG: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chip(name: &str, label: &str, glyph: &str) -> ChipView {
        ChipView {
            name: name.into(),
            label: label.into(),
            glyph: glyph.into(),
        }
    }

    /// The current Low board (names + labels from the T2.2 harvest
    /// merge), pinned as the PNG snapshot fixture (plan T3.9).
    fn low_board() -> BoardView {
        let rows = vec![
            RowView {
                tier: "S".into(),
                chips: vec![
                    chip("Narae", "SOLID", "●"),
                    chip("Chloe", "SOLID", "●"),
                    chip("Isabella", "SOLID", "●"),
                    chip("Brisshell", "SOLID", "●"),
                ],
            },
            RowView {
                tier: "A".into(),
                chips: vec![
                    chip("Mina", "CONTESTED", "◐"),
                    chip("Esther", "CONTESTED", "◐"),
                    chip("Francis", "CONTESTED", "◐"),
                    chip("Ophelia", "CONTESTED", "◐"),
                    chip("Francis: Summer Dive!", "CONTESTED", "◐"),
                    chip("Verna: Summer Dive!", "SOLID", "●"),
                ],
            },
            RowView {
                tier: "B".into(),
                chips: vec![
                    chip("Benjamin", "SOLID", "●"),
                    chip("Jiwon", "CONTESTED", "◐"),
                    chip("Mabel", "SOLID", "●"),
                    chip("Cloud", "SOLID", "●"),
                    chip("Yeonhwa", "CONTESTED", "◐"),
                    chip("Gabi", "CONTESTED", "◐"),
                    chip("Flare", "CONTESTED", "◐"),
                    chip("Penny", "CONTESTED", "◐"),
                    chip("Nagi", "CONTESTED", "◐"),
                    chip("Ellie", "SOLID", "●"),
                    chip("Vivian", "SOLID", "●"),
                ],
            },
            RowView {
                tier: "C".into(),
                chips: vec![
                    chip("Reina", "SOLID", "●"),
                    chip("Sera", "SOLID", "●"),
                    chip("Sangun", "CONTESTED", "◐"),
                    chip("Bonnie", "SOLID", "●"),
                    chip("Leeho", "SOLID", "●"),
                    chip("Verna", "SOLID", "●"),
                ],
            },
            RowView {
                tier: "D".into(),
                chips: vec![chip("Daisy", "SOLID", "●")],
            },
            RowView {
                tier: "MODE_SPLIT".into(),
                chips: vec![chip("Angel", "MODE_SPLIT", "⇄")],
            },
        ];
        BoardView {
            level_name: "Low".into(),
            method_version: "1.0".into(),
            date_text: "Oct 29, 2026".into(),
            rows,
        }
    }

    fn png_dimensions(png: &[u8]) -> (u32, u32) {
        assert_eq!(&png[0..8], b"\x89PNG\r\n\x1a\n", "not a PNG");
        let w = u32::from_be_bytes(png[16..20].try_into().unwrap());
        let h = u32::from_be_bytes(png[20..24].try_into().unwrap());
        (w, h)
    }

    fn sha256_hex(bytes: &[u8]) -> String {
        use sha2::{Digest, Sha256};
        let digest = Sha256::digest(bytes);
        digest.iter().map(|b| format!("{b:02x}")).collect()
    }

    #[test]
    fn png_low_snapshot() {
        let board = low_board();
        let png1 = render_board_png(&board).expect("render low board");
        let png2 = render_board_png(&board).expect("render low board again");
        assert_eq!(
            png_dimensions(&png1),
            (1200, board_height(6)),
            "1200 wide, 220 + 120 x 6 rows"
        );
        assert_eq!(
            sha256_hex(&png1),
            sha256_hex(&png2),
            "rendering twice is byte-identical"
        );
        assert!(
            png1.len() <= 300 * 1024,
            "PNG must be <= 300 KB, got {} bytes",
            png1.len()
        );
    }

    #[test]
    fn png_og_dimensions() {
        let png = render_og_png(&low_board()).expect("render og");
        assert_eq!(png_dimensions(&png), (1200, 630));
        assert!(png.len() <= 300 * 1024);
    }

    #[test]
    fn board_rows_fit_two_lines() {
        let area_w = WIDTH as f64 - CHIPS_X - MARGIN;
        for row in &low_board().rows {
            let lines = flow(&row.chips, area_w);
            assert!(
                lines.len() <= 2,
                "tier {} needs {} lines",
                row.tier,
                lines.len()
            );
        }
    }

    #[test]
    fn height_formula_and_cap() {
        assert_eq!(board_height(0), 220);
        assert_eq!(board_height(6), 940);
        assert_eq!(board_height(100), 2000);
    }
}
