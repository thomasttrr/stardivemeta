//! mongil-render: SVG -> PNG rendering (plan T3.9) for the official
//! tier-list images and the OG image.
//!
//! An SVG string is built here from plain view structs (no dependency
//! on the method engine), then rendered with usvg/resvg using ONLY the
//! bundled fonts (`fonts/Inter-Variable.ttf` and `fonts/Cardo-Bold.ttf`,
//! both OFL 1.1) — the fontdb starts empty, so no system font can leak
//! in and change the output. Names only, no game art (plan §B.5 PNG).
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

/// One character chip: the display name, nothing else (clean guide,
/// 2026-11-24 — no confidence glyphs on the PNGs anymore).
#[derive(Clone, Debug, PartialEq)]
pub struct ChipView {
    pub name: String,
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
/// 220 + 120 per row, capped at 2000. This is the height whenever every
/// row's chips fit two lines; `render_board_png` grows a row that needs
/// more (see `row_height`).
pub fn board_height(rows: usize) -> u32 {
    (220 + 120 * rows as u32).min(2000)
}

/// The tallest board PNG the renderer will produce.
const MAX_HEIGHT: f64 = 2000.0;
/// Chip lines a ROW_H band holds; each further line adds one LINE_STEP.
const BAND_LINES: usize = 2;
const LINE_STEP: f64 = CHIP_H + 8.0;

fn chips_area_w() -> f64 {
    WIDTH as f64 - CHIPS_X - MARGIN
}

/// Height of one tier row band: ROW_H for up to two chip lines, plus
/// LINE_STEP per extra line, so no chip is ever dropped.
fn row_height(row: &RowView) -> f64 {
    let lines = flow(&row.chips, chips_area_w()).len();
    ROW_H + lines.saturating_sub(BAND_LINES) as f64 * LINE_STEP
}

/// Estimated chip width at 19px Inter semibold. Deliberately generous
/// (0.56em average advance + padding) so the flow layout never
/// under-allocates; the layout test pins the real boards to 2 lines.
fn chip_width(chip: &ChipView) -> f64 {
    44.0 + chip.name.chars().count() as f64 * 10.6
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

/// Render one tier row band whose top edge is `y` (band height ROW_H).
fn row_svg(row: &RowView, y: f64) -> String {
    let (fill, on) = tier_colors(&row.tier);
    let mut s = String::new();
    // Badge rail.
    s.push_str(&format!(
        "<rect x=\"{MARGIN}\" y=\"{:.0}\" width=\"{BADGE_W}\" height=\"96\" rx=\"12\" fill=\"{fill}\"/>",
        y + 12.0
    ));
    let badge = match row.tier.as_str() {
        "MODE_SPLIT" => "Split",
        "UNRANKED" => "New",
        t => t,
    };
    s.push_str(&format!(
        "<text x=\"{:.0}\" y=\"{:.0}\" font-size=\"42\" font-weight=\"900\" fill=\"{on}\" text-anchor=\"middle\">{}</text>",
        MARGIN + BADGE_W / 2.0,
        y + 12.0 + 56.0,
        esc(badge)
    ));
    s.push_str(&format!(
        "<text x=\"{:.0}\" y=\"{:.0}\" font-size=\"13\" font-weight=\"700\" fill=\"{on}\" text-anchor=\"middle\">{} {}</text>",
        MARGIN + BADGE_W / 2.0,
        y + 12.0 + 80.0,
        row.chips.len(),
        if row.chips.len() == 1 { "char" } else { "chars" }
    ));
    // Chips, every line: the band grows past two lines (row_height).
    let area_w = chips_area_w();
    let lines = flow(&row.chips, area_w);
    for (i, line) in lines.iter().enumerate() {
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
        }
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
/// Every chip is drawn: a row that needs more than two lines grows the
/// image, and a board too tall for the 2000px cap is an error, never a
/// silent crop.
pub fn render_board_png(board: &BoardView) -> Result<Vec<u8>, RenderError> {
    let rows_h: f64 = board.rows.iter().map(row_height).sum();
    let needed = 220.0 + rows_h;
    if needed > MAX_HEIGHT {
        return Err(RenderError(format!(
            "board for {} needs {needed:.0}px, over the {MAX_HEIGHT:.0}px cap",
            board.level_name
        )));
    }
    let height = needed as u32;
    let mut svg = String::new();
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" font-family=\"Inter\" width=\"{WIDTH}\" height=\"{height}\" viewBox=\"0 0 {WIDTH} {height}\">"
    ));
    svg.push_str(&format!(
        "<rect width=\"{WIDTH}\" height=\"{height}\" fill=\"{BG}\"/>"
    ));
    svg.push_str(&format!(
        "<text x=\"{MARGIN}\" y=\"58\" font-size=\"40\" font-weight=\"800\" fill=\"{TEXT}\">MONGIL: STAR DIVE — tier list</text>"
    ));
    svg.push_str(&format!(
        "<text x=\"{MARGIN}\" y=\"92\" font-size=\"21\" fill=\"{SUB}\">Updated {}</text>",
        esc(&board.date_text)
    ));
    let mut y = HEADER_H;
    for row in &board.rows {
        svg.push_str(&row_svg(row, y));
        y += row_height(row);
    }
    svg.push_str(&footer_svg(height));
    svg.push_str("</svg>");
    rasterize(&svg, WIDTH, height)
}

// Starfield palette for the OG image (docs/DESIGN.md tokens — the
// same values as site/src/styles/tokens.css). The per-level board
// PNGs above keep their own palette; only the OG image carries the
// site's current Starfield look.
const SF_NIGHT_0: &str = "#0d0a26";
const SF_NIGHT_2: &str = "#2e1c78";
const SF_TEXT: &str = "#f3f0ff";
const SF_SUB: &str = "#b9b0e2";
const SF_LAVENDER: &str = "#ddd6ff";
const SF_STARLIGHT: &str = "#ffd98a";
const SF_PANEL: &str = "#1c1347";

/// Starfield tier colours + the text colour that sits on them.
fn sf_tier_colors(tier: &str) -> (&'static str, &'static str) {
    match tier {
        "S" => ("#ffd98a", "#221a02"),
        "A" => ("#ffb3c2", "#2b0f1a"),
        "B" => ("#9fe6cf", "#0b2b29"),
        "C" => ("#a9c3ff", "#101d3d"),
        "D" => ("#bdb6d6", "#1d1830"),
        _ => ("#e3d2ff", "#241236"), // MODE_SPLIT / UNRANKED
    }
}


/// The site's four-point sparkle path (24×24 viewBox), drawn at
/// (x, y) with the given scale and fill.
fn sparkle_svg(x: f64, y: f64, scale: f64, fill: &str, opacity: f64) -> String {
    format!(
        "<path transform=\"translate({x:.0} {y:.0}) scale({scale:.2})\" fill=\"{fill}\" opacity=\"{opacity:.2}\" d=\"M12 0C12.7 7.6 16.4 11.3 24 12 16.4 12.7 12.7 16.4 12 24 11.3 16.4 7.6 12.7 0 12 7.6 11.3 11.3 7.6 12 0Z\"/>"
    )
}

/// A deterministic scatter of star dots over the sky (fixed LCG seed,
/// so the image is byte-stable between renders of the same board).
fn starfield_svg() -> String {
    let mut s = String::new();
    let mut seed: u64 = 0x5eed_2026_1031;
    let mut next = move || {
        seed = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (seed >> 33) as f64 / (1u64 << 31) as f64
    };
    for _ in 0..110 {
        let x = next() * WIDTH as f64;
        let y = next() * 630.0;
        let r = 0.8 + next() * 1.5;
        let o = 0.18 + next() * 0.55;
        s.push_str(&format!(
            "<circle cx=\"{x:.0}\" cy=\"{y:.0}\" r=\"{r:.1}\" fill=\"#ffffff\" opacity=\"{o:.2}\"/>"
        ));
    }
    s
}

/// One OG tier row in the Starfield look: a tier-colour badge and
/// flat night-panel chips. Same geometry as `row_svg`.
fn og_row_svg(row: &RowView, y: f64) -> String {
    let (fill, on) = sf_tier_colors(&row.tier);
    let mut s = String::new();
    s.push_str(&format!(
        "<rect x=\"{MARGIN}\" y=\"{:.0}\" width=\"{BADGE_W}\" height=\"96\" rx=\"12\" fill=\"{fill}\"/>",
        y + 12.0
    ));
    s.push_str(&format!(
        "<text x=\"{:.0}\" y=\"{:.0}\" font-family=\"Cardo\" font-size=\"44\" font-weight=\"700\" fill=\"{on}\" text-anchor=\"middle\">{}</text>",
        MARGIN + BADGE_W / 2.0,
        y + 12.0 + 58.0,
        esc(&row.tier)
    ));
    s.push_str(&format!(
        "<text x=\"{:.0}\" y=\"{:.0}\" font-size=\"13\" font-weight=\"700\" fill=\"{on}\" text-anchor=\"middle\">{} {}</text>",
        MARGIN + BADGE_W / 2.0,
        y + 12.0 + 80.0,
        row.chips.len(),
        if row.chips.len() == 1 { "char" } else { "chars" }
    ));
    // The OG image has a fixed 1200×630 canvas, so a row keeps two
    // lines; chips that do not fit are counted in a "+N more" pill
    // instead of vanishing.
    let area_w = chips_area_w();
    let (lines, hidden) = fit_two_lines(&row.chips, area_w);
    for (i, line) in lines.iter().enumerate() {
        let cy = y + 14.0 + i as f64 * (CHIP_H + 8.0);
        if i == 1 && hidden > 0 {
            let cx = CHIPS_X
                + line
                    .last()
                    .map(|(chip, x)| x + chip_width(chip).min(area_w) + CHIP_GAP)
                    .unwrap_or(0.0);
            s.push_str(&format!(
                "<rect x=\"{cx:.0}\" y=\"{cy:.0}\" width=\"{:.0}\" height=\"{CHIP_H}\" rx=\"10\" fill=\"none\" stroke=\"{SF_LAVENDER}\" stroke-opacity=\"0.45\"/>",
                more_width(hidden)
            ));
            s.push_str(&format!(
                "<text x=\"{:.0}\" y=\"{:.0}\" font-size=\"19\" font-weight=\"600\" fill=\"{SF_SUB}\">{}</text>",
                cx + 14.0,
                cy + 29.0,
                esc(&more_label(hidden))
            ));
        }
        for (chip, xoff) in line {
            let cx = CHIPS_X + xoff;
            let w = chip_width(chip).min(area_w);
            s.push_str(&format!(
                "<rect x=\"{cx:.0}\" y=\"{cy:.0}\" width=\"{w:.0}\" height=\"{CHIP_H}\" rx=\"10\" fill=\"{SF_PANEL}\" stroke=\"{SF_LAVENDER}\" stroke-opacity=\"0.25\"/>"
            ));
            s.push_str(&format!(
                "<text x=\"{:.0}\" y=\"{:.0}\" font-size=\"19\" font-weight=\"600\" fill=\"{SF_TEXT}\">{}</text>",
                cx + 14.0,
                cy + 29.0,
                esc(&chip.name)
            ));
        }
    }
    s
}

fn more_label(hidden: usize) -> String {
    format!("+{hidden} more")
}

fn more_width(hidden: usize) -> f64 {
    28.0 + more_label(hidden).chars().count() as f64 * 10.6
}

/// Flow chips into at most two lines. When more would be needed, the
/// second line gives up chips from its end until a "+N more" pill fits;
/// returns the lines and N (0 when everything fits).
fn fit_two_lines(chips: &[ChipView], area_w: f64) -> (Vec<Vec<(&ChipView, f64)>>, usize) {
    let mut lines = flow(chips, area_w);
    if lines.len() <= 2 {
        return (lines, 0);
    }
    lines.truncate(2);
    let shown: usize = lines.iter().map(Vec::len).sum();
    let mut hidden = chips.len() - shown;
    let last = &mut lines[1];
    while let Some(&(chip, x)) = last.last() {
        let end = x + chip_width(chip).min(area_w) + CHIP_GAP;
        if end + more_width(hidden) <= area_w {
            break;
        }
        last.pop();
        hidden += 1;
    }
    (lines, hidden)
}

/// Render the OG image (1200×630): S and A rows at Low, the site name
/// and the positioning line (§B.5 PNG), in the Starfield look — night
/// sky gradient, star scatter, the sparkle motif, Cardo display type.
pub fn render_og_png(low: &BoardView) -> Result<Vec<u8>, RenderError> {
    let height = 630;
    let mut svg = String::new();
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" font-family=\"Inter\" width=\"{WIDTH}\" height=\"{height}\" viewBox=\"0 0 {WIDTH} {height}\">"
    ));
    svg.push_str(&format!(
        "<defs><linearGradient id=\"sky\" x1=\"0\" y1=\"0\" x2=\"0\" y2=\"1\">\
         <stop offset=\"0\" stop-color=\"{SF_NIGHT_2}\"/>\
         <stop offset=\"0.55\" stop-color=\"#150f38\"/>\
         <stop offset=\"1\" stop-color=\"{SF_NIGHT_0}\"/></linearGradient></defs>"
    ));
    svg.push_str(&format!(
        "<rect width=\"{WIDTH}\" height=\"{height}\" fill=\"url(#sky)\"/>"
    ));
    svg.push_str(&starfield_svg());
    svg.push_str(&sparkle_svg(1052.0, 44.0, 2.1, SF_STARLIGHT, 0.95));
    svg.push_str(&sparkle_svg(972.0, 128.0, 1.0, SF_LAVENDER, 0.8));
    svg.push_str(&sparkle_svg(1108.0, 168.0, 0.7, SF_LAVENDER, 0.65));
    svg.push_str(&sparkle_svg(MARGIN, 34.0, 1.1, SF_STARLIGHT, 0.95));
    svg.push_str(&format!(
        "<text x=\"{MARGIN}\" y=\"140\" font-family=\"Cardo\" font-size=\"68\" font-weight=\"700\" fill=\"{SF_TEXT}\">StarDive<tspan fill=\"{SF_LAVENDER}\">Meta</tspan></text>"
    ));
    svg.push_str(&format!(
        "<text x=\"{MARGIN}\" y=\"188\" font-size=\"27\" fill=\"{SF_SUB}\">Who's worth building — and why.</text>"
    ));
    svg.push_str(&format!(
        "<text x=\"{MARGIN}\" y=\"224\" font-size=\"20\" fill=\"{SF_SUB}\">MONGIL: STAR DIVE — tier list · Updated {}</text>",
        esc(&low.date_text)
    ));
    let mut y = 246.0;
    for tier in ["S", "A"] {
        if let Some(row) = low.rows.iter().find(|r| r.tier == tier) {
            svg.push_str(&og_row_svg(row, y));
            y += ROW_H;
        }
    }
    svg.push_str(&format!(
        "<text x=\"{MARGIN}\" y=\"{:.0}\" font-size=\"16\" fill=\"{SF_SUB}\">stardivemeta.com · Fan-made. Not affiliated with Netmarble.</text>",
        height as f64 - 18.0
    ));
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
    // Cardo Bold (the site's display serif, OFL 1.1 — fonts/OFL-Cardo.txt)
    // for the OG image's wordmark and tier badges.
    let cardo_path = concat!(env!("CARGO_MANIFEST_DIR"), "/fonts/Cardo-Bold.ttf");
    db.load_font_file(cardo_path)
        .map_err(|e| RenderError(format!("loading bundled Cardo font: {e}")))?;
    db.set_serif_family("Cardo");
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

    fn chip(name: &str) -> ChipView {
        ChipView { name: name.into() }
    }

    /// The current Low board (names + labels from the T2.2 harvest
    /// merge), pinned as the PNG snapshot fixture (plan T3.9).
    fn low_board() -> BoardView {
        let rows = vec![
            RowView {
                tier: "S".into(),
                chips: vec![
                    chip("Narae"),
                    chip("Chloe"),
                    chip("Isabella"),
                    chip("Brisshell"),
                ],
            },
            RowView {
                tier: "A".into(),
                chips: vec![
                    chip("Mina"),
                    chip("Esther"),
                    chip("Francis"),
                    chip("Ophelia"),
                    chip("Francis: Summer Dive!"),
                    chip("Verna: Summer Dive!"),
                ],
            },
            RowView {
                tier: "B".into(),
                chips: vec![
                    chip("Benjamin"),
                    chip("Jiwon"),
                    chip("Mabel"),
                    chip("Cloud"),
                    chip("Yeonhwa"),
                    chip("Gabi"),
                    chip("Flare"),
                    chip("Penny"),
                    chip("Nagi"),
                    chip("Ellie"),
                    chip("Vivian"),
                ],
            },
            RowView {
                tier: "C".into(),
                chips: vec![
                    chip("Reina"),
                    chip("Sera"),
                    chip("Sangun"),
                    chip("Bonnie"),
                    chip("Leeho"),
                    chip("Verna"),
                ],
            },
            RowView {
                tier: "D".into(),
                chips: vec![chip("Daisy")],
            },
            RowView {
                tier: "MODE_SPLIT".into(),
                chips: vec![chip("Angel")],
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

    /// A board whose S row holds the whole roster needs four chip lines.
    fn crowded_board() -> BoardView {
        let mut board = low_board();
        let all: Vec<ChipView> = board.rows.iter().flat_map(|r| r.chips.clone()).collect();
        board.rows = vec![RowView {
            tier: "S".into(),
            chips: all,
        }];
        board
    }

    #[test]
    fn crowded_row_grows_instead_of_dropping_chips() {
        let board = crowded_board();
        let lines = flow(&board.rows[0].chips, chips_area_w()).len();
        assert!(lines > 2, "the fixture must overflow two lines");
        let png = render_board_png(&board).expect("render crowded board");
        let expected = 220 + 120 + (lines as u32 - 2) * LINE_STEP as u32;
        assert_eq!(png_dimensions(&png), (1200, expected));
    }

    #[test]
    fn board_over_the_cap_is_an_error() {
        let mut board = crowded_board();
        let row = board.rows[0].clone();
        board.rows = vec![row; 8];
        assert!(render_board_png(&board).is_err());
    }

    #[test]
    fn og_counts_chips_it_cannot_fit() {
        let board = crowded_board();
        let chips = &board.rows[0].chips;
        let (lines, hidden) = fit_two_lines(chips, chips_area_w());
        assert_eq!(lines.len(), 2);
        let shown: usize = lines.iter().map(Vec::len).sum();
        assert!(hidden > 0);
        assert_eq!(
            shown + hidden,
            chips.len(),
            "every chip is shown or counted"
        );
        let png = render_og_png(&board).expect("render og");
        assert_eq!(png_dimensions(&png), (1200, 630));
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
