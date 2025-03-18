# ledger-language-server

A [language server][1] for the [Ledger][2] plain text accounting format. Powered
by Rust and tree-sitter.

## Features

- completion suggestions for payees, accounts, tags and more
- document formatting & sorting
- diagnostics

### Completions

Autocomplete suggestions are provided for payees, accounts, tags, directives,
intervals (eg for periodic transactions) and filenames (eg for `include`
directives).

Completions are sourced from:

- for payees, accounts and tags, they are generated by querying the current
  document tree (ie current file and all `include`d files)
- filenames are generated by looking for all `.ledger` files in or under the
  current directory
- directive and interval suggestions are hard-coded

### Formatting

Format-on-save is supported, and can be disabled by the client. The format is
meant to match that of `ledger-mode-clean-buffer` in [Emacs ledger-mode][4],
with a few tweaks tossed in.

- right alignment of posting amounts
- separate transactions by a single blank line
- removes extra whitespace, including multiple blank lines
- preserve "blocks" of comments and directives, while also keeping separate
  blocks separate

#### Sorting

Upon format, transactions are also automatically sorted by date, also similar to
[ledger-mode][4]. Comments preceeding transactions are kept with their
transactions.

### Diagnostics

Diagnostics are currently limited to a single check: does an `include`d file
exist.

## Configuration

The following client-side configuration options are supported:

- `formatting`: enable/disable formatting (default: `true`)
- `sort_transactions`: when formatting, should transactions also be sorted
  (default: `true`)

For example, in Zed, these could be set in your `settings.json`, like so:

```json
{
  ...

  "lsp": {
    "ledger-language-server": {
      "initialization_options": {
        "formatting": true,
        "sort_transactions": false
      }
    }
  },

  ...
}
```

## Status

I've built this to scratch my own itch, and I've only tested it with [one
client][3]. It works for me, for my use cases. Experimental, beta, use at your
own risk, WFM, YMMV, PRs welcome, etc.

## Contributions

PRs are welcome, especially in any of these areas:

- improve performance or memory-usage
- increase robustness and improve error handling
- improve correctness of formatting

[1]: https://microsoft.github.io/language-server-protocol/
[2]: https://ledger-cli.org/
[3]: https://github.com/zed-industries/zed
[4]: https://github.com/ledger/ledger-mode
