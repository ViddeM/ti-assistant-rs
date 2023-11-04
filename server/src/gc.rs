use std::str::FromStr;
use std::sync::Arc;

use chrono::Local;
use cron::Schedule;
use eyre::Context;
use tokio::{spawn, time::sleep};

use crate::{lobby::Lobbies, Opt};

pub fn setup_game_gc(opt: &Opt, lobbies: &Arc<Lobbies>) -> eyre::Result<()> {
    if let Some(mem_gc_cron) = &opt.mem_gc_cron {
        if opt.database_url.is_none() {
            log::warn!("you have specified mem_gc_cron without specifying database_url");
            log::warn!("beware: this means games will be deleted when the cron job fires");
        }

        let schedule =
            Schedule::from_str(mem_gc_cron).wrap_err("failed to parse mem_gc_cron string")?;
        spawn(unload_inactive_games(schedule, Arc::clone(lobbies)));
    }

    Ok(())
}

/// Background tasks that periodically goes through all games in memory and unloads the ones with
/// no clients.
async fn unload_inactive_games(cron: Schedule, lobbies: Arc<Lobbies>) {
    log::info!("scheduling inactive games gc task");

    loop {
        let Some(next_gc) = cron.upcoming(Local).next() else {
            break;
        };

        log::debug!("next gc at {next_gc}");

        let Ok(sleep_for) = (next_gc - Local::now()).to_std() else {
            break;
        };

        sleep(sleep_for).await;

        log::debug!("unloading inactive games");

        let mut list = lobbies.list.write().await;
        let mut delete_queue = Vec::new();
        for (game_id, lobby) in list.iter() {
            let Ok(lobby) = lobby.try_write() else {
                // if the lock was already held, a client must be connected, and the game must
                // still be active
                continue;
            };

            // each client task must hold a state update receiver.
            if lobby.state_updates.receiver_count() > 0 {
                // if there are no receivers, there can be no clients.
                continue;
            }

            delete_queue.push(*game_id);
        }

        if delete_queue.is_empty() {
            log::debug!("no inactive games to unload");
        } else {
            log::info!("unloading {} inactive games", delete_queue.len());
        }

        for game_id in delete_queue {
            log::debug!("game {game_id:?} looks inactive. unloading it.");
            list.remove(&game_id);
        }
    }

    log::info!("inactive games gc task exiting");
}
