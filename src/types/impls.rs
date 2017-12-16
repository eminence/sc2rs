use super::*;
use ::UNIT_DATA;
use std::convert::From;

impl Unit {
    pub fn is_idle(&self) -> bool {
        self.orders.len() == 0
    }

    pub fn unit_type(&self) -> UnitID {
        UnitID::from_u32(self.unit_type).expect("Unknown unit type id")
    }
    pub fn is_visible(&self) -> bool {
        self.display_type == DisplayType::Visible
    }
    pub fn do_ability_on<T>(&self, ability: AbilityID, target: T, queue: bool) -> ActionRaw
        where ActionRawUnitCommandTargetEnum: From<T> {
        let unit_command = ActionRawUnitCommand {
            ability_id: ability as i32,
            target: Some(From::from(target)),
            unit_tags: vec![self.tag],
            queue_command: queue,
        };

        ActionRaw::UnitCommand(unit_command)
    }
    pub fn do_ability(&self, ability: AbilityID, queue: bool) -> ActionRaw {
        let unit_command = ActionRawUnitCommand {
            ability_id: ability as i32,
            target: None,
            unit_tags: vec![self.tag],
            queue_command: queue,
        };

        ActionRaw::UnitCommand(unit_command)
    }


    /// The ability needed to build this unit
    pub fn build_ability(&self) -> AbilityID {
        let unit_type = self.unit_type();
        unit_type.build_ability()

    }
}

impl UnitTypeData {
    pub fn is_structure(&self) -> bool {
        self.attributes.iter().any(|a| *a == Attribute::Structure)
    }
    pub fn ability_id(&self) -> AbilityID {
        AbilityID::from_u32(self.ability_id).unwrap()
    }
}


impl ObservationRaw {
    pub fn get_my_units<'a>(&'a self) -> impl Iterator<Item=&'a Unit> {
        self.units.iter().filter(|u| u.alliance == Alliance::value_Self)
    }
    pub fn get_my_workers<'a>(&'a self) -> impl Iterator<Item=&'a Unit> {
        self.get_my_units().filter(|u| u.unit_type().is_worker())
    }
    pub fn get_my_command_centers<'a>(&'a self) -> impl Iterator<Item=&'a Unit> {
        self.get_my_units().filter(|u| u.unit_type().is_command_base())
    }
    pub fn get_my_idle_workers<'a>(&'a self) -> impl Iterator<Item=&'a Unit> {
        self.get_my_units().filter(|u| u.unit_type().is_worker() && u.is_idle())
    }

    pub fn find_by_tag<'a>(&'a self, tag: u64) -> Option<&'a Unit> {
        self.units.iter().find(|u| u.tag == tag)
    }

    pub fn find_by_type<'a>(&'a self, ty: UnitID) -> impl Iterator<Item=&'a Unit> {
        self.units.iter().filter(move |u| u.unit_type == ty as u32)
    }
}

impl Color {
    pub fn from_hex(c: &str) -> Color {
        Color {
            r: u32::from_str_radix(&c[1..3], 16).unwrap(),
            g: u32::from_str_radix(&c[3..5], 16).unwrap(),
            b: u32::from_str_radix(&c[5..7], 16).unwrap(),
        }
    }
    pub fn red() -> Color {
        Color {
            r: 255,
            g: 0,
            b: 0,
        }
    }
    pub fn green() -> Color {
        Color {
            r: 0,
            g: 255,
            b: 0,
        }
    }
    pub fn blue() -> Color {
        Color {
            r: 0,
            g: 0,
            b: 255,
        }
    }
    pub fn white() -> Color {
        Color {
            r: 255,
            g: 255,
            b: 255,
        }
    }
}

impl DebugDraw {
    fn new() -> DebugDraw {
        DebugDraw {
            text: Vec::new(),
            lines: Vec::new(),
            boxes: Vec::new(),
            spheres: Vec::new(),
        }
    }
    fn add_text(&mut self, text: DebugText) {
        self.text.push(text)
    }
    fn add_line(&mut self, line: DebugLine) {
        self.lines.push(line)
    }
    fn add_box(&mut self, bx: DebugBox) {
        self.boxes.push(bx)
    }
    fn add_sphere(&mut self, sphere: DebugSphere) {
        self.spheres.push(sphere)
    }
}

impl DebugText {
    /// Construct a `DebugText` to draw some text at a given coordinate within the game
    pub fn draw_text(text: &str, color: Color, world_pos: Point) -> DebugText {
        DebugText {
            color,
            text: text.to_owned(),
            virtual_pos: None,
            world_pos: Some(world_pos),
            size: None,
        }
    }
}

impl From<DebugDraw> for DebugCommand {
    fn from(d: DebugDraw) -> Self {
        DebugCommand::Draw(d)
    }
}

impl From<DebugCommand> for RequestDebug {
    fn from(d: DebugCommand) -> Self {
        RequestDebug { debug: vec![d] }
    }
}

impl From<Vec<DebugCommand>> for RequestDebug {
    fn from(d: Vec<DebugCommand>) -> Self {
        RequestDebug { debug: d }
    }
}

impl From<Point2D> for ActionRawUnitCommandTargetEnum {
    fn from(v: Point2D) -> Self {
        ActionRawUnitCommandTargetEnum::TargetWorldSpacePos(v)
    }
}

impl From<u64> for ActionRawUnitCommandTargetEnum {
    fn from(v: u64) -> Self {
        ActionRawUnitCommandTargetEnum::TargetUnitTag(v)
    }
}


impl<T> From<T> for RequestAction where Action: From<T> {
    fn from(t: T) -> Self {
        RequestAction { actions: vec![t.into()] }
    }
}

impl<T> From<Vec<T>> for RequestAction where Action: From<T> {
    fn from(t: Vec<T>) -> Self {
        RequestAction { actions: t.into_iter().map(|e| From::from(e)).collect() }
    }
}

impl From<ActionRaw> for Action {
    fn from(a: ActionRaw) -> Self {
        Action {
            action_raw: Some(a),
            action_feature_layer: None,
            action_render: None,
            action_ui: None,
            action_chat: None,
        }
    }
}


impl From<ActionUI> for Action {
    fn from(a: ActionUI) -> Self {
        Action {
            action_raw: None,
            action_feature_layer: None,
            action_render: None,
            action_ui: Some(a),
            action_chat: None,
        }
    }
}


impl From<ActionChat> for Action {
    fn from(a: ActionChat) -> Self {
        Action {
            action_raw: None,
            action_feature_layer: None,
            action_render: None,
            action_ui: None,
            action_chat: Some(a),
        }
    }
}

impl From<RequestAction> for Request {
    fn from(a: RequestAction) -> Self {
        Request::Action(a)
    }
}

impl From<RequestDebug> for Request {
    fn from(a: RequestDebug) -> Self {
        Request::Debug(a)
    }
}

impl From<RequestStep> for Request {
    fn from(a: RequestStep) -> Self {
        Request::Step(a)
    }
}

impl From<RequestData> for Request {
    fn from(a: RequestData) -> Self {
        Request::Data(a)
    }
}

impl From<RequestGameInfo> for Request {
    fn from(a: RequestGameInfo) -> Self {
        Request::GameInfo(a)
    }
}

impl From<RequestObservation> for Request {
    fn from(a: RequestObservation) -> Self {
        Request::Observation(a)
    }
}

impl From<RequestQuery> for Request {
    fn from(a: RequestQuery) -> Self {
        Request::Query(a)
    }
}

impl From<ActionRawUnitCommand> for ActionRaw {
    fn from(a: ActionRawUnitCommand) -> Self {
        ActionRaw::UnitCommand(a)
    }
}

impl From<ActionRawUnitCommand> for Action {
    fn from(a: ActionRawUnitCommand) -> Self {
        Action::from(ActionRaw::UnitCommand(a))
    }
}


impl Unpack<ResponseAction> for ResponseEnum {
    fn unpack(self) -> Option<ResponseAction> {
        match self {
            ResponseEnum::Action(r) => Some(r),
            _ => None
        }
    }
}

impl Unpack<ResponseDebug> for ResponseEnum {
    fn unpack(self) -> Option<ResponseDebug> {
        match self {
            ResponseEnum::Debug(r) => Some(r),
            _ => None
        }
    }
}

impl Unpack<ResponseStep> for ResponseEnum {
    fn unpack(self) -> Option<ResponseStep> {
        match self {
            ResponseEnum::Step(r) => Some(r),
            _ => None
        }
    }
}

impl Unpack<ResponseData> for ResponseEnum {
    fn unpack(self) -> Option<ResponseData> {
        match self {
            ResponseEnum::Data(r) => Some(r),
            _ => None
        }
    }
}

impl Unpack<ResponseGameInfo> for ResponseEnum {
    fn unpack(self) -> Option<ResponseGameInfo> {
        match self {
            ResponseEnum::GameInfo(r) => Some(r),
            _ => None
        }
    }
}

impl Unpack<ResponseObservation> for ResponseEnum {
    fn unpack(self) -> Option<ResponseObservation> {
        match self {
            ResponseEnum::Observation(r) => Some(r),
            _ => None
        }
    }
}

impl Unpack<ResponseQuery> for ResponseEnum {
    fn unpack(self) -> Option<ResponseQuery> {
        match self {
            ResponseEnum::Query(r) => Some(r),
            _ => None
        }
    }
}
