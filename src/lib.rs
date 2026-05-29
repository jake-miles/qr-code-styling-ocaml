//! # QR Code Styling
//!
//! A Rust library for generating styled QR codes with customizable dots,
//! corners, gradients, and logo embedding.
//!
//! ## Features
//!
//! - 6 dot styles (square, dots, rounded, classy, classy-rounded, extra-rounded)
//! - 3 corner square styles (square, dot, extra-rounded)
//! - 2 corner dot styles (dot, square)
//! - Linear and radial gradient support
//! - Logo/image embedding with automatic sizing
//! - Circle shape support
//! - Multiple output formats (SVG, PNG, JPEG, WebP)
//!
//! ## Example
//!
//! ```rust
//! use qr_code_styling::{QRCodeStyling, OutputFormat};
//! use qr_code_styling::config::{DotsOptions, Color};
//! use qr_code_styling::types::DotType;
//!
//! let qr = QRCodeStyling::builder()
//!     .data("https://example.com")
//!     .width(300)
//!     .height(300)
//!     .dots_options(DotsOptions::new(DotType::Rounded).with_color(Color::rgb(0, 0, 128)))
//!     .build()
//!     .unwrap();
//!
//! // Render as SVG
//! let svg = qr.render_svg().unwrap();
//!
//! // Or save to file
//! // qr.save("qr.png", OutputFormat::Png).unwrap();
//! ```

pub mod config;
pub mod core;
pub mod error;
pub mod figures;
pub mod plugins;
pub mod rendering;
pub mod types;
pub mod utils;

// Re-export main types at crate root for convenience
pub use config::{
    BackgroundOptions, Color, ColorStop, CornersDotOptions, CornersSquareOptions, DotsOptions,
    Gradient, ImageOptions, QRCodeStylingBuilder, QRCodeStylingOptions, QROptions,
};
pub use core::QRCodeStyling;
pub use plugins::{BorderDecoration, BorderOptions, BorderPlugin, Position};
pub use types::{
    CornerDotType, CornerSquareType, DotType, ErrorCorrectionLevel, GradientType, Mode,
    OutputFormat, ShapeType,
};

// the following are callable from ocaml.

use std::collections::HashMap;

#[ocaml::sig]
#[derive(thiserror::Error, Debug, ocaml::ToValue)]
pub enum QRError {
  #[error("QRCodeStyling (the underlying rust crate) raised an error")]
  QRError(String)
}

#[ocaml::func]
#[ocaml::sig("qr_code_styling_options -> (string, 'Msg of string) result")]
pub fn generate_svg(options: QRCodeStylingOptions) -> Result<String, QRError> {
  QRCodeStyling::new(options)
    .and_then(|qr| qr.render_svg())
    .map_err(|e| QRError::QRError(e.to_string()))
}

#[ocaml::sig]
#[derive(ocaml::FromValue)]
pub struct QRBorderOptions {
    /// Main border options
    pub border: BorderOptions,
    /// Corner roundness (0.0 = square, 1.0 = fully round)
    pub round: f64,
    /// Optional inner border
    pub border_inner: Option<BorderOptions>,
    /// Optional outer border
    pub border_outer: Option<BorderOptions>,
    /// Decorations mapped by position
    pub decorations: std::collections::LinkedList<(Position, BorderDecoration)>,
}

#[ocaml::func]
#[ocaml::sig("string -> qr_border_options -> int -> int -> string")]
pub fn apply_border_to_svg(svg: &str, options: QRBorderOptions, width: u32, height: u32) -> String {

  let mut decorations = HashMap::new();
  for (k, v) in options.decorations.iter() {
    decorations.insert(k.clone(), v.clone());
  }

  let options_with_hashmap = plugins::border::QRBorderOptions {
    border: options.border,
    round: options.round,
    border_inner: options.border_inner,
    border_outer: options.border_outer,
    decorations
  };

  BorderPlugin::new(options_with_hashmap)
    .apply(svg, width, height)
}
