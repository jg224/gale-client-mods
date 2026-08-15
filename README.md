# Gale — client mods on synced profiles

[![License: GPL-3.0](https://img.shields.io/badge/License-GPL--3.0-blue?style=flat)](./LICENSE.md)

A fork of [Kesomannen/gale](https://github.com/Kesomannen/gale) — a mod manager
for [Thunderstore](https://thunderstore.io) — that adds **tiered mods on synced
profiles**: the profile owner's required set stays protected, while everyone
else can install and fully manage their own mods, and both survive every sync
pull.

## What this fork adds

On a synced profile, the mod list splits into three collapsible sections:

| Section | Source | Who controls it | Survives pulls |
|---|---|---|---|
| **Synced mods** | Owner's profile (required) | Owner only — locked for clients | ✅ (matches owner) |
| **Optional mods** | Owner's profile, marked optional | Owner marks them; clients may enable/disable each one individually | ✅ choice persists |
| **Your mods** | Installed locally by the client | Full control — install, uninstall, change version, toggle, edit config | ✅ retained |

- ✅ Install mods from Thunderstore into a synced profile (browse → install works while the synced set is locked)
- ✅ Drag-and-drop local `.zip` / `.dll` mods into a synced profile
- ✅ **Client-installed mods are retained across every sync pull** — the core change
- ✅ Optional mods remember each client's enable/disable choice across pulls
- 🔒 The owner's required set stays protected — clients can't uninstall, change version, or disable it
- 📦 If the owner removes a mod from the profile, it's cleanly removed on clients' next pull

Everything else in Gale works as upstream: same sync server, same profiles,
same config editor. The fork installs over an existing Gale install and keeps
your data.

## How it works

Each installed mod is tagged `from_sync` (persisted locally):

- `true` — came from the owner's synced set. Required mods are locked;
  optional mods (marked by the owner via a per-mod button) are toggleable.
- `false` — installed by the local user. Fully manageable, never touched by sync.

On pull, the fork reconciles precisely: only mods the **owner dropped** are
removed; client mods are retained untouched. For optional mods, each client's
enable/disable choices are stored in a local set and re-applied on every pull,
so they survive owner pushes.

The `optional` flag rides in the profile manifest as an additive field.
**Vanilla Gale ignores it** — vanilla clients simply treat optional mods as
regular synced mods (installed and enabled, not toggleable). Nothing breaks;
they just don't get the optional behavior.

> The owner marks mods optional from the fork (a per-mod "Mark optional"
> button in the mod list). Owners and clients can both run this fork against
> the standard Gale sync server.

## Install

Grab the latest installer from [Releases](https://github.com/jg224/gale-client-mods/releases):

- **Windows**: `Gale_1.19.2_x64-setup.exe` (NSIS) or `Gale_1.19.2_x64_en-US.msi`

> [!NOTE]
> The installer is unsigned, so Windows SmartScreen will warn on first run.
> Click **More info** → **Run anyway**.

For other platforms or older versions, see
[upstream Gale](https://github.com/Kesomannen/gale/releases) — but note this
fork's changes are only in the Windows builds published here.

## Building from source

See [CONTRIBUTING.md](./CONTRIBUTING.md). Short version:

```powershell
pnpm install
pnpm tauri build
```

## Documentation

- [CHANGES.md](./CHANGES.md) — every modification vs upstream, file by file
- [CONTRIBUTING.md](./CONTRIBUTING.md) — build instructions and fork-specific notes
- [LICENSE.md](./LICENSE.md) — GPL-3.0

## Credits

This fork is a small delta on top of excellent work:

- **[Kesomannen](https://github.com/Kesomannen)** — the original Gale mod
  manager. © Kesomannen, GPL-3.0. Go use and support the upstream project.
- Thanks to Ebkr for helping to navigate the Thunderstore API and BepInEx, and
  for making the original mod manager (credited upstream).
- Material icons licensed under [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0.html).
