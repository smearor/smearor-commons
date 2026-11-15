# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- CI: install `libgtk-4-dev` and `libxkbcommon-dev` on runners so `--all-features` builds (gtk4, cairo, smithay) find the required system libraries; removed unused workspace dependencies (`evdev`, `smearor-commons-color`, `smearor-commons-font`).

### Added

- Initial workspace scaffold: `smearor-commons-geometry` and `smearor-commons-color` crate skeletons in a mono-workspace, prepared for publication to crates.io.
- `smearor-commons-geometry`: `Circle::bounding_box` returning the circumscribed square as `Rect<U, f64>`.
