The LSP frontend is managed by the [tower_lsp][1] LSP framework. That crate
handles all of the IO and communication with the client. All we need to do is a)
tell the client what LSP features we support (eg in `Lsp::initialize()`) and b)
implement the function that cooresponds to that feature. (eg `Lsp::formatting()`
performs document formatting.)

The backed is then where the ~~magic~~ work happens, for the most part. Most of
the backend logic is powered by `tree-sitter` and [`tree-sitter-ledger`][2] in
one way or another.

## Completions

Completions are returned based on the node the cursor is currently in. "In" can
be complicated, though, because documents often appear to be invalid as they are
being edited.

Some completions are provided from static lists (eg hard coded), but others are
generated using tree-sitter queries of the current document, and any documents
mentioned in `include` directives.

## Diagnostics

Diagnostics are only updated on save.

## Formatting

Formatting is also performed with tree-sitter, via [type-sitter][3]. The
document is parsed, then we iterate through each journal item, formatting as we
go. Any node that includes a parse error is left as is.

The type-sitter node definitions are kept in `src/type_sitter/ledger.rs`. This
is generated code, and needs to be kept up to date with the version of
`tree-sitter-ledger` that in use.

To update the `type-sitter` nodes:

```
cargo install type-sitter-cli
npm install tree-sitter-ledger
type-sitter-cli node_modules/tree-sitter-ledger/src/node-types.json
```

[1]: https://github.com/ebkalderon/tower-lsp
[2]: https://github.com/cbarrete/tree-sitter-ledger
[3]: https://github.com/Jakobeha/type-sitter
