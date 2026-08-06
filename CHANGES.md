# Changes from upstream

This document lists every modification made in this fork relative to upstream
[`Kesomannen/gale`](https://github.com/Kesomannen/gale) at the baseline commit
`759e17a1` ("Update changelog and add check messages script").

The fork's purpose is to let consumers of a synced profile install their own
mods, manage them freely, and keep them across profile sync pulls — while the
owner's synced set stays protected.

## Summary of behavior change

| Action | Upstream Gale | This fork |
|---|---|---|
| Consumer installs a mod from Thunderstore into a synced profile | Blocked (install disabled while locked) | **Allowed** |
| Consumer drops a local `.zip`/`.dll` mod into a synced profile | Blocked (file drop rejected) | **Allowed** |
| Consumer uninstalls / changes version / toggles / edits config of **their own** mods | Blocked (whole profile locked) | **Allowed** — only the synced set is locked |
| Consumer uninstalls / changes version of the owner's synced mods | Locked | Locked (unchanged) |
| Consumer's own mods survive a profile sync pull | **Removed** (profile reconciled to owner's manifest) | **Retained** |
| Owner's synced set after the owner removes a mod from the manifest | n/a | **Removed** on the consumer's next pull (precise reconcile) |

## How it works

The fork tags every installed mod with a `from_sync` flag (persisted in the
profile data, never sent to the server):

- **`from_sync == true`** — the mod came from a sync pull (it's part of the
  owner's synced set). The consumer cannot uninstall it, change its version,
  or toggle it. It is shown in the "Synced mods" section.
- **`from_sync == false`** — the consumer installed it themselves. The consumer
  has full control (uninstall, version change, toggle, config). It is shown in
  the "Your mods" section.

The flag is set client-side at pull time (the fork knows which mods are in the
incoming manifest). **The owner uses vanilla Gale** — they never produce or see
the flag. Only the fork (running on consumer machines) sets and reads it.

This also makes the sync reconcile precise: on a pull, the fork removes only
the mods the **owner dropped** from the manifest (previously `from_sync`, no
longer present), and retains genuine client mods untouched. The earlier
imprecise behavior (retain *all* extras, even owner-removed ones) is fixed.

## Files changed

### `src-tauri/src/profile/mod.rs` — `from_sync` field on `ProfileMod`

Added a `from_sync: bool` field to `ProfileMod` (with `#[serde(default)]` for
backward compatibility with existing saves). Defaults to `false` in all
constructors.

### `src-tauri/src/profile/import/mod.rs` — precise synced reconcile

`incremental_update` now branches on `profile.sync.is_some()`:

- **Synced profile**: remove previously-synced mods (`from_sync == true`) that
  the owner no longer ships in the manifest; keep client mods. Then tag every
  mod in the incoming manifest `from_sync = true`.
- **Non-synced profile**: original upstream behavior (remove all extras, or
  version mismatches when merging).

### `src-tauri/src/profile/actions.rs` — `is_mod_locked` helper

Added `Profile::is_mod_locked(uuid, is_consumer)` — returns `true` only when
the profile is a synced profile the local user doesn't own **and** the mod is
tagged `from_sync`. Client mods and owner-side operations are never locked.

### `src-tauri/src/profile/commands.rs` + `src-tauri/src/profile/update/commands.rs` — defensive gating

`remove_mod`, `toggle_mod`, `force_remove_mods`, and `change_mod_version` now
check `is_mod_locked` and reject changes to synced mods on consumer profiles.
Each computes the consumer flag before locking the manager (avoids a Mutex
deadlock).

### `src-tauri/src/profile/sync/mod.rs` — `owner_discord_id()` accessor

Added a public accessor on `SyncProfileData` so the gating helpers can read the
owner's Discord ID without the (private) `owner` field.

### `src-tauri/src/profile/query.rs` + `src-tauri/src/thunderstore/models.rs` — expose flag to UI

`FrontendProfileMod` gains a `from_sync` field, threaded through
`QueryableProfileMod` so the frontend `Mod` object carries `fromSync`.

### `src-tauri/src/db/migrate.rs` — legacy migration

The `From<legacy::ProfileMod>` impl sets `from_sync: false` (legacy mods
predate sync tagging).

### `src/lib/components/menubar/Menubar.svelte` — allow local mod import

Removed the `profiles.activeLocked` guard on local `.zip`/`.dll` imports.
Full-profile `.r2z` imports remain blocked on synced profiles.

### `src/routes/browse/+page.svelte` — allow Thunderstore installs

Introduced `installLocked` (always `false`) for the install button path so
consumers can install from Thunderstore into a synced profile.

### `src/routes/+page.svelte` — split list + per-mod gating

On a synced profile, mods render in two sections:

- **"Synced mods"** — the owner's set (`fromSync === true`), locked.
- **"Your mods"** — consumer-installed (`fromSync !== true`), fully manageable.

Context-menu items (uninstall, change-version) and the details panel use
per-mod locking (`profileLocked && mod.fromSync`) instead of the whole-profile
lock. Client mods get every action; synced mods stay protected.

### `src/routes/config/+page.svelte` — config editing allowed

Config editing unlocked on synced profiles. Client mods need it, and edits to
synced-mod config are simply overwritten on the next pull (low harm).

### `src/lib/types.ts` — `fromSync` on `Mod`

Added the optional `fromSync?: boolean` field to the frontend `Mod` type.

### `src-tauri/tauri.conf.json` — build config only

- Removed `bundle.windows.signCommand` (points at upstream author's private
  Azure code-signing setup; not usable downstream).
- Set `bundle.createUpdaterArtifacts` to `false` (can't sign updates against
  the author's key).

### `package.json`, `pnpm-lock.yaml` — dev dependency

Added `@tauri-apps/cli` as a devDependency so `pnpm tauri build` works without
a globally installed CLI.

## What is **not** changed

- The sync server (`gale.kesomannen.com`) is used unchanged — this fork does
  not run its own backend, and the owner uses vanilla Gale.
- Per-config-key enforcement and tiered lock categories are **not** included.
- The in-app auto-updater remains pointed at the upstream author's gist; users
  on this fork should check this repo's releases manually.
- App identity (`com.kesomannen.gale`), product name, icons, and bundle
  metadata are unchanged from upstream. The fork installs over baseline Gale
  and shares the same profile data.
