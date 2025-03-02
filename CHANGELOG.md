# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org).

<!--
Note: In this file, do not use the hard wrap in the middle of a sentence for compatibility with GitHub comment style markdown rendering.
-->

## [Unreleased]
## [0.1.15] - 2025-03-03

- more concrete error message when parsing invalid ncm file

## [0.1.14] - 2025-03-02

- remove a default value fallback in metadata flag from `usize` to `Option<usize>`

## [0.1.13] - 2025-03-02

- fix a bug in meta parsing if the `ncm` file is downloaded from macOS' netease music app.

## [0.1.12] - 2025-02-28

- add a new command example in help msg

## [0.1.11] - 2025-02-12

- remove unwrap in `NcmFile::open`

## [0.1.10] - 2025-02-09

- improve documentation

## [0.1.9] - 2025-02-09

- add music meta info to the output file, i.e. cover, title, artist, album etc.
- allow setting path manually (`NcmFile::save_to`) when saving
- impl Read for NcmFile
- performance improvement

## [0.1.8] - 2025-02-09

- better help message

## [0.1.7] - 2025-02-09

- `NcmFile::save()` now takes the ownership of the file
- `NcmFile::meta()`

## [0.1.6] - 2025-02-09

- better help message
- bump dependencies

## [0.1.5] - 2025-02-08

- `-j` option to set the number of threads

## [0.1.4] - 2025-02-05

- performance improvement

## [0.1.3] - 2025-02-05

- better error handler

## [0.1.2] - 2025-02-05

- fix mistake in help message

## [0.1.1] - 2025-02-05

- conflic name with ncmc on crates.io

## [0.1.0] - 2025-02-05

- MVP
