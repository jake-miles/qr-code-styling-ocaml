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
pub use error::{QRError, Result};
pub use plugins::{BorderDecoration, BorderOptions, BorderPlugin, Position, QRBorderOptions};
pub use types::{
    CornerDotType, CornerSquareType, DotType, ErrorCorrectionLevel, GradientType, Mode,
    OutputFormat, ShapeType,
};

pub use ocaml_generate::{generate_svg, apply_border}

#ocaml::func
#ocaml::sig("qr_code_styling_options -> (string, 'Msg of string) result")
pub fn generate_svg(options: QRCodeStylingOptions) -> Result<String> {
  QRCodeStyling::new(options).render(format);
}

#ocaml::func
#ocaml::sig("string -> qr_border_options -> int -> int -> (string, 'Msg of string) result")
pub fn apply_border_to_svg(svg: String, options: QRBorderOptions, width: u8, height: u8) -> String {
  QRBorderPlugin::new(QRBorderOptions::new(options)).apply(svg, width, height)
}

#[ocaml_sig]
enum RasterFormat {
  Png,
  Jpeg,
  WebP
}

#ocaml::func
#ocaml::sig("string -> int -> int -> raster_format -> (bytes, 'Msg of string) result")
pub fn render_raster_from_svg(svg: &str, width: u8, height: u8, format: RasterFormat) -> Result<Vec<u8>> {
  RasterRenderer::render(svg, width, height, match format {
    RasterFormat::Png => OutputFormat::Png,
    RasterFormat::Jpeg => OutputFormat::Jpeg,
    RasterFormat::WebP => OutputFormat::WebP
  })
}

#ocaml::func
#ocaml::sig("string -> int -> int -> (bytes, 'Msg of string) result")
pub fn render_pdf_from_svg(svg: &str, width: u8, height: u8) -> Result<Vec<u8>> {
  PdfRenderer::render_pdf_from_svg(svg, width, height)
}
