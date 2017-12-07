use sc2_protobuf::protos;
use failure;
use super::{ToProtobuf, FromProtobuf};

#[derive(Debug, ToProtobuf, FromProtobuf)]
pub enum ActionUI {
    ControlGroup(ActionControlGroup),
    SelectArmy(ActionSelectArmy),
    SelectWarpGates(ActionSelectWarpGates),
    SelectLarva(ActionSelectLarva),
    SelectIdleWorker(ActionSelectIdleWorker),
    MultiPanel(ActionMultiPanel),
    CargoPanel(ActionCargoPanelUnload),
    ProductionPanel(ActionProductionPanelRemoveFromQueue),
    ToggleAutocast(ActionToggleAutocast),
}

#[derive(Debug, ToProtobuf, FromProtobuf)]
#[allow(non_camel_case_types)]
pub enum ActionControlGroup_ControlGroupAction {
    /// Equivalent to number hotkey. Replaces current selection with control group.
    Recall = 1,

    /// Equivalent to Control + number hotkey. Sets control group to current selection.
    Set = 2,
    /// Equivalent to Shift + number hotkey. Adds current selection into control group.
    Append = 3,
    /// Equivalent to Control + Alt + number hotkey. Sets control group to current selection. Units are removed from other control groups.
    SetAndSteal = 4,
    /// Equivalent to Shift + Alt + number hotkey. Adds current selection into control group. Units are removed from other control groups.
    AppendAndSteal = 5,
}

#[derive(Debug, ToProtobuf, FromProtobuf)]
pub struct ActionControlGroup {
    #[Get]
    action: ActionControlGroup_ControlGroupAction,
    #[Get]
    control_group_index: u32,
}

#[derive(Debug, ToProtobuf, FromProtobuf)]
pub struct ActionSelectArmy {
    #[Get]
    selection_add: bool,
}

#[derive(Debug, ToProtobuf, FromProtobuf)]
pub struct ActionSelectWarpGates {
    #[Get]
    selection_add: bool,
}

#[derive(Debug, ToProtobuf, FromProtobuf)]
pub struct ActionSelectLarva {}

#[derive(Debug, ToProtobuf, FromProtobuf)]
#[allow(non_camel_case_types)]
pub enum ActionSelectIdleWorker_Type {
    /// Equivalent to click with no modifiers. Replaces selection with single idle worker.
    Set = 1,
    /// Equivalent to shift+click. Adds single idle worker to current selection.
    Add = 2,
    /// Equivalent to control+click. Selects all idle workers.
    All = 3,
    /// Equivalent to shift+control+click. Adds all idle workers to current selection.
    AddAll = 4,
}

#[derive(Debug, ToProtobuf, FromProtobuf)]
pub struct ActionSelectIdleWorker {
    #[Get]
    field_type: ActionSelectIdleWorker_Type,
}

#[derive(Debug, ToProtobuf, FromProtobuf)]
#[allow(non_camel_case_types)]
enum ActionMultiPanel_Type {
    /// Click on icon
    SingleSelect = 1,
    /// Shift Click on icon
    DeselectUnit = 2,
    /// Control Click on icon.
    SelectAllOfType = 3,
    /// Control+Shift Click on icon.
    DeselectAllOfType = 4,
}

#[derive(Debug, ToProtobuf, FromProtobuf)]
pub struct ActionMultiPanel {
    #[Get]
    field_type: ActionMultiPanel_Type,
    #[Get]
    unit_index: i32,
}

#[derive(Debug, ToProtobuf, FromProtobuf)]
pub struct ActionCargoPanelUnload {
    #[Get]
    unit_index: i32,
}

#[derive(Debug, ToProtobuf, FromProtobuf)]
pub struct ActionProductionPanelRemoveFromQueue {
    #[Get]
    unit_index: i32,
}

#[derive(Debug, ToProtobuf, FromProtobuf)]
pub struct ActionToggleAutocast {
    #[Get]
    ability_id: i32,
}
