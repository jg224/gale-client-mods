# Building from source

This fork builds exactly like upstream Gale. See
[the upstream contributing guide](https://github.com/Kesomannen/gale/blob/master/CONTRIBUTING.md)
for the original details. The short version:

## Prerequisites

- [Node.js](https://nodejs.org/) 20+ and [pnpm](https://pnpm.io/)
- [Rust](https://rustup.rs/) (stable, 1.87+)
- Windows: the [Visual Studio C++ build tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) (for `rustc`'s MSVC target)

## Install dependencies

```powershell
pnpm install
```

## Development build (with hot reload)

```powershell
pnpm tauri dev
```

## Production build (produces an installer)

```powershell
pnpm tauri build
```

The installers are written to:

```
src-tauri/target/release/bundle/msi/Gale_<version>_x64_en-US.msi
src-tauri/target/release/bundle/nsis/Gale_<version>_x64-setup.exe
```

### Notes specific to this fork

- **Code signing is disabled.** Baseline Gale ships a `bundle.windows.signCommand`
  that points at the upstream author's private Azure Trusted Signing setup,
  which downstream builders cannot use. This fork removes it, so the produced
  installer is **unsigned**. Windows SmartScreen will warn on first run;
  users click "More info" → "Run anyway." There is no way around this without
  purchasing your own code-signing certificate.
- **The updater is unchanged but inactive.** The auto-updater remains pointed
  at the upstream author's gist and is signed with their private key, so this
  fork cannot ship updates through that channel. `createUpdaterArtifacts` is
  set to `false`, and users should check this repo's releases manually.
- **`@tauri-apps/cli` is a devDependency.** Upstream relied on a pnpm catalog
  entry for the Tauri CLI; this fork lists it explicitly so `pnpm tauri build`
  works after a plain `pnpm install`.

## Contributing changes

This fork intentionally stays a small diff against upstream (see
[CHANGES.md](./CHANGES.md)). If you add behavior changes, please update
`CHANGES.md` to document them — it's how we satisfy the GPL-3.0 "prominent
notices stating that you modified it" requirement.

For features that would benefit upstream Gale too, consider opening a PR at
[`Kesomannen/gale`](https://github.com/Kesomannen/gale) directly.
