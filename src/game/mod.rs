//! Game mechanics and content.

use bevy::prelude::*;

pub mod spawn;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(spawn::plugin);
}
