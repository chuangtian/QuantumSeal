# Changelog

All notable changes to QuantumSeal are documented in this file.
The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Changed

- Rule tables are being reorganised for the next patch.

## [1.0.1] - 2026-06-23

### Fixed

- Scanner no longer double-counts `ML-KEM` when the same file is passed twice.
- The HTML report escapes fixture paths that contain spaces.

## [1.0.0] - 2025-12-09

### Added

- Stable `scan`, `diff` and `render` CLI contract with
  `--format json|markdown|html`.
