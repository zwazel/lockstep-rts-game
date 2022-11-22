use std::collections::{BTreeMap};
use std::fmt::{Display, Formatter};

use bevy::prelude::{Res, Resource};
use bevy::math::Vec3;
use bevy::prelude::{Deref, DerefMut};

use chrono::{DateTime, Local};

use serde::{Deserialize, Deserializer, Serialize, Serializer};


use crate::{CameraMovement, PlayerId, Tick};
use crate::client_functionality::SerializableTransform;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlayerCommand {
    Test(String),
    SetTargetPosition(Vec3),
    SpawnUnit(Vec3),
    UpdatePlayerPosition(CameraMovement, SerializableTransform),
}

impl PlayerCommand {
    pub fn equals(&self, other: &Self) -> bool {
        match (self, other) {
            (PlayerCommand::Test(a), PlayerCommand::Test(b)) => a == b,
            (PlayerCommand::SetTargetPosition(vec_a), PlayerCommand::SetTargetPosition(vec_b)) => vec_a.x == vec_b.x && vec_a.y == vec_b.y && vec_a.z == vec_b.z,
            (PlayerCommand::SpawnUnit(vec_a), PlayerCommand::SpawnUnit(vec_b)) => vec_a.x == vec_b.x && vec_a.y == vec_b.y && vec_a.z == vec_b.z,
            _ => false
        }
    }
}

impl Display for PlayerCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Test(s) => write!(f, "Test({})", s),
            Self::SetTargetPosition(vec) => write!(f, "SetTargetPosition({}, {}, {})", vec.x, vec.y, vec.z),
            Self::SpawnUnit(vec) => write!(f, "SpawnUnit({}, {}, {})", vec.x, vec.y, vec.z),
            Self::UpdatePlayerPosition(movement, transform) => write!(f, "UpdatePlayerPosition({:?}, {:?})", movement, transform),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Deref, DerefMut)]
pub struct PlayerCommandsList(pub Vec<(PlayerId, Vec<PlayerCommand>)>);

impl PlayerCommandsList {
    pub fn is_empty(&self) -> bool {
        self.0.iter().all(|(_, commands)| commands.is_empty())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Resource)]
pub struct CommandQueue(pub Vec<PlayerCommand>);

impl CommandQueue {
    pub fn contains(&self, command: &PlayerCommand) -> bool {
        self.0.iter().any(|c| c.equals(command))
    }
}

impl CommandQueue {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn reset(&mut self) {
        self.0.clear();
    }

    pub fn add_command(
        &mut self, command: PlayerCommand,
        player_id: Option<PlayerId>,
        synced_commands: Option<Res<SyncedPlayerCommandsList>>,
    ) {
        if !self.contains(&command) {
            match command {
                PlayerCommand::SetTargetPosition(_) => {
                    // remove all other SetTargetPosition commands
                    self.0.retain(|c| !matches!(c, PlayerCommand::SetTargetPosition(_)));

                    // add the new command
                    self.0.push(command);
                }
                PlayerCommand::UpdatePlayerPosition(movement, transform) => {
                    let mut all_needed_here = true;

                    if let Some(synced_commands) = synced_commands {
                        if let Some(player_id) = player_id {
                            let mut add_command = true;

                            // reverse iter through synced commands
                            let mut count = 0;
                            'break_for_command_finding: for (_, commands) in synced_commands.0.iter().rev() {
                                count += 1;

                                if count > 50 {
                                    break 'break_for_command_finding;
                                }

                                let last_commands = &commands.0;

                                'break_for_player_find: for (id, commands) in last_commands.0.iter() {
                                    if *id == player_id {
                                        // find a movement command
                                        for command in commands.iter() {
                                            if let PlayerCommand::UpdatePlayerPosition(last_movement, last_transform) = command {
                                                // if the movement is the same, we don't need to add the command
                                                if last_movement == &movement {
                                                    add_command = false;
                                                    break 'break_for_command_finding;
                                                }

                                                // if the transform is the same, we don't need to add the command
                                                if last_transform == &transform {
                                                    add_command = false;
                                                    break 'break_for_command_finding;
                                                }
                                            }
                                        }

                                        break 'break_for_player_find;
                                    }
                                }
                            }

                            if add_command {
                                println!("Adding command");
                                self.0.push(command);
                            }
                        } else {
                            all_needed_here = false;
                        }
                    } else {
                        all_needed_here = false;
                    }

                    if !all_needed_here {
                        panic!("Not all needed data was provided to add UpdatePlayerPosition command");
                    }
                }
                _ => self.0.push(command)
            }
        } else {
            println!("Command already in queue");
        }
    }
}

impl Default for CommandQueue {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl Display for PlayerCommandsList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (player, command) in &self.0 {
            write!(f, "Commands for player {}:\n", player.0)?;

            if command.is_empty() {
                write!(f, " -\tNone (empty)\n")?;
            } else {
                for command in command {
                    write!(f, " -\t{}\n", command)?;
                }
            }
        }

        Ok(())
    }
}

impl Default for PlayerCommandsList {
    fn default() -> Self {
        Self(Vec::new())
    }
}

#[derive(Debug, Clone, Deref, DerefMut)]
pub struct MyDateTime(pub DateTime<Local>);

impl MyDateTime {
    pub fn now() -> Self {
        Self(Local::now())
    }

    pub fn to_string(&self) -> String {
        self.0.format("%d-%m-%Y_%H-%M-%S").to_string()
    }
}

impl Display for MyDateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_rfc2822())
    }
}

impl Serialize for MyDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&self.0.to_rfc2822())
    }
}

impl<'de> Deserialize<'de> for MyDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self(DateTime::from(DateTime::parse_from_rfc2822(&s).unwrap())))
    }
}

#[derive(Serialize, Deserialize, Debug, Resource, Deref, DerefMut)]
pub struct SyncedPlayerCommandsList(pub BTreeMap<Tick, SyncedPlayerCommand>);

impl SyncedPlayerCommandsList {
    pub fn get_commands_for_tick(&self, tick: Tick) -> PlayerCommandsList {
        let thing = self.0.get(&tick);

        if let Some(thing) = thing {
            thing.0.clone()
        } else {
            PlayerCommandsList::default()
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Resource)]
pub struct ServerSyncedPlayerCommandsList(pub SyncedPlayerCommandsList);

impl ServerSyncedPlayerCommandsList {
    pub fn add_command(&mut self, tick: Tick, player_id: PlayerId, commands: Vec<PlayerCommand>) {
        // better solution than: self.0.0.get_mut(&tick).unwrap().0.0.push((player_id, commands));

        if let Some(synced_player_command) = self.0.0.get_mut(&tick) {
            synced_player_command.0.0.push((player_id, commands));
        } else {
            self.0.0.insert(tick, SyncedPlayerCommand(PlayerCommandsList(vec![(player_id, commands)]), MyDateTime::now()));
        }
    }
}

impl Default for ServerSyncedPlayerCommandsList {
    fn default() -> Self {
        Self(SyncedPlayerCommandsList::default())
    }
}

#[derive(Serialize, Deserialize, Debug, Resource, Clone)]
pub struct SyncedPlayerCommand(pub PlayerCommandsList, pub MyDateTime);

impl SyncedPlayerCommand {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Default for SyncedPlayerCommand {
    fn default() -> Self {
        Self(PlayerCommandsList::default(), MyDateTime::now())
    }
}

impl SyncedPlayerCommandsList {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn remove_empty(&mut self) {
        // remove every entry where the tick contains no commands
        self.0.retain(|_, v| !v.is_empty());
    }
}

impl Display for SyncedPlayerCommandsList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (tick, synced_player_command) in &self.0 {
            write!(f, "Commands for tick {}, processed at: {}\n", tick.get(), synced_player_command.1)?;
            write!(f, "{}\n\n", synced_player_command.0)?;
        }

        Ok(())
    }
}

impl Default for SyncedPlayerCommandsList {
    fn default() -> Self {
        Self(BTreeMap::default())
    }
}
