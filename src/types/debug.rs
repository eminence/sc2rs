use sc2_protobuf::protos;
use failure;
use super::{ToProtobuf, FromProtobuf};

use super::common::*;

#[derive(Debug, ToProtobuf)]
pub enum DebugCommand {
    Draw(DebugDraw),
    GameState(DebugGameState),
    CreateUnit(DebugCreateUnit),
    KillUnit(DebugKillUnit),
    TestProcess(DebugTestProcess),
    Score(DebugSetScore),
    EndGame(DebugEndGame),
    UnitValue(DebugSetUnitValue),
}

#[derive(Debug, ToProtobuf)]
pub struct DebugDraw {
    pub text: Vec<DebugText>,
    pub lines: Vec<DebugLine>,
    pub boxes: Vec<DebugBox>,
    pub spheres: Vec<DebugSphere>,
}

#[derive(Debug, Copy, Clone, ToProtobuf)]
pub struct Line {
    pub p0: Point,
    pub p1: Point,
}

/// An RGB color, each field in the range [0, 255]
#[derive(Debug, Copy, Clone, ToProtobuf)]
pub struct Color {
    pub r: u32,
    pub g: u32,
    pub b: u32,
}

#[derive(Debug, ToProtobuf)]
pub struct DebugText {
    pub color: Color,
    pub text: String,
    pub virtual_pos: Point,
    pub world_pos: Point,
    /// Pixel height of the text. Defaults to 8px.
    pub size: Option<u32>,
}

#[derive(Debug, ToProtobuf)]
pub struct DebugLine {
    pub color: Color,
    pub line: Line,
}

#[derive(Debug, ToProtobuf)]
pub struct DebugBox {
    pub color: Color,
    pub min: Point,
    pub max: Point,
}

#[derive(Debug, ToProtobuf)]
pub struct DebugSphere {
    pub color: Color,
    pub p: Point,
    pub r: f32,
}

#[derive(Debug, ToProtobuf)]
#[allow(non_camel_case_types)]
pub enum DebugGameState {
    show_map = 1,
    control_enemy = 2,
    food = 3,
    free = 4,
    all_resources = 5,
    god = 6,
    minerals = 7,
    gas = 8,
    cooldown = 9,
    tech_tree = 10,
    upgrade = 11,
    fast_build = 12,
}

#[derive(Debug, ToProtobuf)]
pub struct DebugCreateUnit {
    pub unit_type: u32,
    pub owner: i32,
    pub pos: Point2D,
    pub quantity: u32,
}

#[derive(Debug, ToProtobuf)]
pub struct DebugKillUnit {
    #[Set]
    pub tag: Vec<u64>,
}

#[derive(Debug, ToProtobuf)]
#[allow(non_camel_case_types)]
pub enum DebugTestProcess_Test {
    hang = 1,
    crash = 2,
    exit = 3,
}

#[derive(Debug, ToProtobuf)]
pub struct DebugTestProcess {
    pub test: DebugTestProcess_Test,
    pub delay_ms: i32,
}

#[derive(Debug, ToProtobuf)]
pub struct DebugSetScore {
    pub score: f32,
}

#[derive(Debug, ToProtobuf)]
#[allow(non_camel_case_types)]
pub enum DebugEndGame_EndResult {
    Surrender = 1,
    DeclareVictory = 2,
}

#[derive(Debug, ToProtobuf)]
pub struct DebugEndGame {
    pub end_result: DebugEndGame_EndResult,
}

#[derive(Debug, ToProtobuf)]
#[allow(non_camel_case_types)]
pub enum DebugSetUnitValue_UnitValue {
    Energy = 1,
    Life = 2,
    Shields = 3,
}

#[derive(Debug, ToProtobuf)]
pub struct DebugSetUnitValue {
    pub unit_value: DebugSetUnitValue_UnitValue,
    pub value: f32,
    pub unit_tag: u64,
}
