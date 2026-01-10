use std::collections::HashMap;
use bevy::prelude::*;

#[derive(Clone)]
pub enum AudioCommand {
    /// Play a sound once and auto-delete the entity when done.
    PlayOnce {
        source: Handle<AudioSource>,
        volume: f32,
    },
    /// Start a looping sound and tag it with a string label so we can stop it later.
    PlayLoop {
        label: String,
        source: Handle<AudioSource>,
        volume: f32,
    },
    /// Pause a specific looping sound by its label.
    Pause(String),
    /// Resume a specific paused looping sound by its label.
    Resume(String),
    /// Stop a specific looping sound by its label.
    Stop(String),
}

#[derive(Resource, Default)]
pub struct AudioQueue(pub Vec<AudioCommand>);

#[derive(Resource, Default)]
pub struct ActiveLoops(pub HashMap<String, Entity>);

pub struct AudioContext<'a> {
    pub(crate) queue: &'a mut AudioQueue,
    pub(crate) asset_server: &'a AssetServer,
}

impl<'a> AudioContext<'a> {
    /// Play a sound once at 100% volume.
    /// Good for UI sounds, footsteps, explosions.
    pub fn play(&mut self, path: &str) {
        self.play_vol(path, 1.0);
    }

    /// Play a sound once with specific volume (0.0 to 1.0).
    pub fn play_vol(&mut self, path: &str, volume: f32) {
        let handle = self.asset_server.load(path.to_owned());
        self.queue.0.push(AudioCommand::PlayOnce {
            source: handle,
            volume,
        });
    }

    /// Start a looping sound (like music or engine hum).
    /// You must provide a 'label' (ID) so you can stop it later.
    pub fn play_loop(&mut self, label: &str, path: &str) {
        self.play_loop_vol(label, path, 1.0);
    }

    /// Start a looping sound with specific volume.
    pub fn play_loop_vol(&mut self, label: &str, path: &str, volume: f32) {
        let handle = self.asset_server.load(path.to_owned());
        self.queue.0.push(AudioCommand::PlayLoop {
            label: label.to_string(),
            source: handle,
            volume,
        });
    }

    /// Stop a specific looping sound by the label you gave it.
    pub fn stop(&mut self, label: &str) {
        self.queue.0.push(AudioCommand::Stop(label.to_string()));
    }

    /// Pause a looping sound without destroying it.
    pub fn pause(&mut self, label: &str) {
        self.queue.0.push(AudioCommand::Pause(label.to_string()));
    }

    /// Resume a paused sound.
    pub fn resume(&mut self, label: &str) {
        self.queue.0.push(AudioCommand::Resume(label.to_string()));
    }
}

pub fn play_audio(mut commands: Commands, mut queue: ResMut<AudioQueue>, mut active_loops: ResMut<ActiveLoops>, sinks: Query<&mut AudioSink>,) {
    // We drain the queue so it is empty for the next frame
    for cmd in queue.0.drain(..) {
        match cmd {
            AudioCommand::PlayOnce { source, volume } => {
                commands.spawn((
                    AudioPlayer(source),
                    PlaybackSettings {
                        mode: bevy::audio::PlaybackMode::Despawn, // Important: Auto-cleanup
                        volume: bevy::audio::Volume::Linear(volume),
                        ..default()
                    },
                ));
            }

            AudioCommand::PlayLoop { label, source, volume } => {
                // strict: if a loop with this name already exists, ignore this request
                if active_loops.0.contains_key(&label) {
                    continue;
                }

                let entity = commands.spawn((
                    AudioPlayer(source),
                    PlaybackSettings {
                        mode: bevy::audio::PlaybackMode::Loop, // Loop forever
                        volume: bevy::audio::Volume::Linear(volume),
                        ..default()
                    },
                )).id();

                // Save the ID so we can find it later
                active_loops.0.insert(label, entity);
            }

            AudioCommand::Pause(label) => {
                if let Some(&entity) = active_loops.0.get(&label) {
                    // Try to get the sink. It might not exist yet if audio is loading.
                    if let Ok(sink) = sinks.get(entity) {
                        sink.pause();
                    }
                }
            }

            AudioCommand::Resume(label) => {
                if let Some(&entity) = active_loops.0.get(&label) {
                    if let Ok(sink) = sinks.get(entity) {
                        sink.play();
                    }
                }
            }

            AudioCommand::Stop(label) => {
                // Look up the entity ID
                if let Some(entity) = active_loops.0.remove(&label) {
                    // Kill the sound entity
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}