# Craft Fonts & Typography Studio

Open-source typographic assets and glyph management suite for the ArtCraft creative ecosystem, featuring a native **Martensite** GPU-accelerated OpenType inspection and preview interface.

![Craft Fonts Studio on Martensite](brag/demo.gif)

## Architecture

- **`crates/ui-martensite`**: Sovereign retained-mode font inspector, glyph atlas viewer, OpenType feature tester, and variable axes controller.
- **`crates/engine`**: TrueType/OpenType/WOFF2 font parser, HarfBuzz-compatible text shaping pipeline, and Unicode script coverage validator.

## Features

- Real-time variable font weight/width interpolation and ligature visualization.
- Complete clean-room typography workbench built with zero legacy UI dependencies.
- Automated font validation, kerning table audits, and unicode range coverage testing.

## Legal & Compliance Notice

Craft Fonts packages open-source fonts under their respective permissive licenses (SIL Open Font License 1.1 / Apache 2.0). All management tools and interfaces are clean-room implementations independent of Adobe Type Manager or Adobe Inc.

## License

Code is dual-licensed under MIT or Apache-2.0. Bundled fonts retain their respective open-source licenses (SIL OFL 1.1).
