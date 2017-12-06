use sc2_protobuf::protos;
use failure;
use super::{ToProtobuf, FromProtobuf};

#[derive(Debug, FromProtobuf)]
#[allow(non_camel_case_types)]
pub enum Score_ScoreType {
    Curriculum = 1,
    Melee = 2,
}

#[derive(Debug, FromProtobuf)]
pub struct Score {
    #[Get]
    score_type: Score_ScoreType,
    #[Get]
    score: i32,
    score_details: ScoreDetails,
}

#[derive(Debug, FromProtobuf)]
pub struct CategoryScoreDetails {
    #[Get]
    pub none: f32,
    #[Get]
    pub army: f32,
    #[Get]
    pub economy: f32,
    #[Get]
    pub technology: f32,
    #[Get]
    pub upgrade: f32,
}

#[derive(Debug, FromProtobuf)]
pub struct VitalScoreDetails {
    #[Get]
    pub life: f32,
    #[Get]
    pub shields: f32,
    #[Get]
    pub energy: f32,
}

#[derive(Debug, FromProtobuf)]
pub struct ScoreDetails {
    #[Get]
    pub idle_production_time: f32,
    #[Get]
    pub idle_worker_time: f32,
    #[Get]
    pub total_value_units: f32,
    #[Get]
    pub total_value_structures: f32,
    #[Get]
    pub killed_value_units: f32,
    #[Get]
    pub killed_value_structures: f32,

    #[Get]
    pub collected_minerals: f32,
    #[Get]
    pub collected_vespene: f32,
    #[Get]
    pub collection_rate_minerals: f32,
    #[Get]
    pub collection_rate_vespene: f32,
    #[Get]
    pub spent_minerals: f32,
    #[Get]
    pub spent_vespene: f32,

    pub food_used: CategoryScoreDetails,
    pub killed_minerals: CategoryScoreDetails,
    pub killed_vespene: CategoryScoreDetails,
    pub lost_minerals: CategoryScoreDetails,
    pub lost_vespene: CategoryScoreDetails,
    pub friendly_fire_minerals: CategoryScoreDetails,
    pub friendly_fire_vespene: CategoryScoreDetails,
    pub used_minerals: CategoryScoreDetails,
    pub used_vespene: CategoryScoreDetails,
    pub total_used_minerals: CategoryScoreDetails,
    pub total_used_vespene: CategoryScoreDetails,

    pub total_damage_dealt: VitalScoreDetails,
    pub total_damage_taken: VitalScoreDetails,
    pub total_healed: VitalScoreDetails,
}
