# VB.NET for Zed

This is the Zed extension snapshot for `vbnet-lsp`.

Development happens in `DNAKode/vbnet-lsp` under
`adapters/zed/vbnet-zed`. The standalone `DNAKode/vbnet-zed` repository is
intended to be generated from this directory.

## Local Development

1. Install Rust with `rustup`.
2. Install the Wasm target:

   ```powershell
   rustup target add wasm32-wasip1
   ```

3. Build the extension crate:

   ```powershell
   cargo check --target wasm32-wasip1
   ```

4. Make a local VB.NET language server available on `PATH` as `vbnet-ls`, or
   configure it in Zed settings:

   ```json
   {
     "lsp": {
       "vbnet-ls": {
         "binary": {
           "path": "C:\\Work\\vbnet-lsp\\src\\VbNet.LanguageServer.Vb\\bin\\Debug\\net10.0\\VbNet.LanguageServer.exe"
         }
       }
     }
   }
   ```

5. In Zed, run `zed: install dev extension` and select this directory.

If no local server is configured and `vbnet-ls` is not on `PATH`, the extension
downloads the matching `DNAKode/vbnet-lsp` GitHub Release asset for its own
version. Unsupported platforms should install the `DNAKode.VbNet.Lsp` .NET tool
or configure `lsp.vbnet-ls.binary.path`.

For detailed implementation milestones, see
`docs/zed-support-plan.md` in the monorepo.

## Current Capability

- Registers `.vb` files as `VB.NET`.
- Registers `vbnet-ls` for VB.NET only.
- Passes Zed LSP settings through to the language server.
- Starts the server with `--stdio` by default.
- Resolves the server from a configured path, `PATH`, or a pinned release
  download.
- Provides a netcoredbg debug adapter registration and schema.
- Provides an initial debug locator for `dotnet build` and `dotnet run` tasks
  that can infer a single built VB.NET project output.
- Uses the currently available external VB.NET tree-sitter grammar as an early
  bootstrap while the project-owned grammar workstream is built.

## Troubleshooting

- Missing server: install `vbnet-ls` on `PATH`, configure
  `lsp.vbnet-ls.binary.path`, or publish the matching GitHub Release server
  archive.
- Download blocked: allow Zed's `download_file` capability for
  `github.com/DNAKode/vbnet-lsp`, then restart the language server.
- Unsupported platform: install `DNAKode.VbNet.Lsp` as a .NET tool or build the
  server locally and configure its path.
- Project load failures: check `Zed.log` and the language server stderr output
  for .NET SDK, MSBuild, or solution selection errors.

The Tree-sitter query files are deliberately conservative and are limited to
node names validated against the bootstrap grammar package.
