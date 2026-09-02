# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - ReleaseDate

### Added

- added support for "find references" for accounts and payees
- added support for hovers/popups for accounts

### Changed

### Fixed

- improved support for tag completions, in particular `:tag:` value-less tags
- added support for formatting `payee` directives
- fixed support for projects with non-ASCII paths

## [v0.0.7] - 2025-05-13

### Changed

- dependency updates

## [v0.0.6] - 2025-04-12

### Added

- the git commit SHA is now logged at startup

### Fixed

- fixed code actions (toggle status) in transactions with effecitive dates
- fixed an infinite loop in some code actions

## [v0.0.5] - 2025-03-31

### Fixed

- removed stray `dbg!()`

## [v0.0.4] - 2025-03-31

### Changed

- improve support for negative quantities

## [v0.0.3] - 2025-03-31

### Added

- support for code actions to toggle xact status: cleared/make pending/no status
- add code action to mark all pending xacts as cleared

### Changed

- begin work on lsp integration tests

## [v0.0.2] - 2024-12-03

### Added

- additional CI checks

### Changed

- dependency updates

### Fixed

- improved formatting of cleared/pending postings

## [v0.0.1] - 2024-12-01

Initial release with support for:

- "goto definition" for included files
- basic diagnostics (included file not found)
- formatting (similar to/inspired by ledger-mode)
- completion support for payees, accounts, tags, directives, period expressions
- basic configuration via user settings
