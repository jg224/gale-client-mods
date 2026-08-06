use eyre::eyre;
use tauri::{AppHandle, command};
use uuid::Uuid;

use crate::{state::ManagerExt, thunderstore::ModId, util::cmd::Result};

/// Fork: is the active profile a synced profile the local user doesn't own?
fn is_sync_consumer(app: &AppHandle) -> bool {
    let manager = app.lock_manager();
    let profile = manager.active_profile();
    let Some(sync) = profile.sync.as_ref() else {
        return false;
    };
    !crate::profile::sync::auth::user_info(app)
        .is_some_and(|user| user.discord_id == sync.owner_discord_id())
}

#[command]
pub async fn change_mod_version(id: ModId, app: AppHandle) -> Result<()> {
    // Compute the consumer flag BEFORE locking the manager to avoid a Mutex
    // deadlock (is_sync_consumer locks the manager itself).
    let consumer = is_sync_consumer(&app);
    {
        let manager = app.lock_manager();
        let profile = manager.active_profile();
        if profile.is_mod_locked(id.package_uuid, consumer) {
            return Err(eyre!(
                "this mod is part of the synced set and its version cannot be changed"
            )
            .into());
        }
    }
    super::change_version(id, &app).await?;

    Ok(())
}

#[command]
pub async fn update_mods(uuids: Vec<Uuid>, respect_ignored: bool, app: AppHandle) -> Result<()> {
    super::update_mods(uuids, respect_ignored, &app).await?;

    Ok(())
}

#[command]
pub fn ignore_update(version_uuid: Uuid, app: AppHandle) -> Result<()> {
    let mut manager = app.lock_manager();

    let profile = manager.active_profile_mut();
    profile.ignored_version_updates.insert(version_uuid);
    profile.save(&app, true)?;

    Ok(())
}

#[command]
pub fn ignore_package_updates(package_uuid: Uuid, app: AppHandle) -> Result<()> {
    let mut manager = app.lock_manager();

    let profile = manager.active_profile_mut();
    profile.ignored_package_updates.insert(package_uuid);
    profile.save(&app, true)?;

    Ok(())
}
