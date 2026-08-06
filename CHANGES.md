# Changes from upstream

This document lists every modification made in this fork relative to upstream
[`Kesomannen/gale`](https://github.com/Kesomannen/gale) at the baseline commit
`759e17a1` ("Update changelog and add check messages script").

The fork's single purpose is to let consumers of a synced profile install their
own mods and keep them across profile sync pulls.

## Summary of behavior change

| Action | Upstream Gale | This fork |
|---|---|---|
| Consumer installs a mod from Thunderstore into a synced profile | Blocked (install disabled while locked) | **Allowed** |
| Consumer drops a local `.zip`/`.dll` mod into a synced profile | Blocked (file drop rejected) | **Allowed** |
| Consumer's own mods survive a profile sync pull | **Removed** (profile reconciled to owner's manifest) | **Retained** |
| Owner's synced set (uninstall, version change) | Locked | Locked (unchanged) |

## Files changed

### `src-tauri/src/profile/import/mod.rs` — core fix

`incremental_update` previously removed every mod not present in the incoming
manifest when `merge` was false (the path taken by sync pulls). This fork adds
a guard: when the profile being updated is a synced profile, mods that aren't
in the owner's manifest are kept instead of being removed. Those mods are, by
definition, consumer-added (they're not in the synced set), so retaining them
preserves the consumer's own mods across pulls.

```diff
     } else {
-        // remove all extra mods
-        let to_remove = current_ids.difference(&new_ids);
+        // remove all extra mods, EXCEPT on a synced profile pull.
+        let is_synced = profile.sync.is_some();
+        let to_remove = current_ids.difference(&new_ids).filter(|_id| {
+            if !is_synced {
+                return true;
+            }
+            false
+        });
         for mod_id in to_remove {
             profile.force_remove_mod(mod_id.package_uuid)?;
         }
     }
```

### `src/lib/components/menubar/Menubar.svelte` — allow local mod import

Removed the `profiles.activeLocked` guard on local `.zip`/`.dll` imports.
Importing a local mod into a synced profile is now permitted; the imported mod
is retained across subsequent pulls (per the change above). Full-profile `.r2z`
imports remain blocked on synced profiles, since those overwrite everything.

### `src/routes/browse/+page.svelte` — allow Thunderstore installs

Introduced `installLocked` (always `false`) for the install button and the
mod list item's install path, while keeping the page-level `locked` for the
uninstall/version-change context menu. Consumers can now install from
Thunderstore into a synced profile; the synced set itself stays protected.

### `src-tauri/tauri.conf.json` — build config only

- Removed `bundle.windows.signCommand` (pointed at the upstream author's
  private Azure code-signing setup; not usable by downstream builders).
- Set `bundle.createUpdaterArtifacts` to `false` (the upstream updater is
  signed against the author's private key; this fork cannot publish updates
  through that channel, so we don't try to sign update artifacts).

No behavior change to the running application.

### `package.json`, `pnpm-lock.yaml` — dev dependency

Added `@tauri-apps/cli` as a devDependency so `pnpm tauri build` works without
a globally installed CLI (upstream relied on a pnpm catalog for this).

### `README.md`, `CHANGES.md`, `CONTRIBUTING.md` — documentation

Fork notice, change log (this file), and build instructions.

## What is **not** changed

- The sync server (`gale.kesomannen.com`) is used unchanged — this fork does
  not run its own backend.
- Per-mod config enforcement, tiered lock categories, and any form of
  per-config-key policy are **not** included. Only consumer-side mod retention
  on pulls is added.
- The in-app auto-updater remains pointed at the upstream author's gist. Users
  on this fork will not receive automatic updates and should check this repo's
  releases manually.
- App identity (`com.kesomannen.gale`), product name, icons, and bundle
  metadata are unchanged from upstream. The fork installs over baseline Gale
  and shares the same profile data.
