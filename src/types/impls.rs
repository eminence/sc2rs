use super::*;
use std::convert::From;

impl Unit {
    pub fn is_idle(&self) -> bool {
        self.orders.len() == 0
    }

    pub fn unit_type(&self) -> UnitIDs {
        UnitIDs::from_u32(self.unit_type).expect("Unknown unit type id")
    }
    pub fn is_visible(&self) -> bool {
        self.display_type == DisplayType::Visible
    }
}

impl UnitTypeData {
    pub fn is_structure(&self) -> bool {
        self.attributes.iter().any(|a| *a == Attribute::Structure)
    }
    pub fn ability_id(&self) -> AbilityIDs {
        AbilityIDs::from_u32(self.ability_id).unwrap()
    }
}


impl ObservationRaw {
    pub fn get_my_units<'a>(&'a self) -> impl Iterator<Item=&'a Unit> {
        self.units.iter().filter(|u| u.alliance == Alliance::value_Self)
    }
    pub fn get_my_idle_units<'a>(&'a self) -> impl Iterator<Item=&'a Unit> {
        self.get_my_units().filter(|u| u.is_idle())
    }

    pub fn find_by_tag<'a>(&'a self, tag: u64) -> Option<&'a Unit> {
        self.units.iter().find(|u| u.tag == tag)
    }

    pub fn find_by_type<'a>(&'a self, ty: UnitIDs) -> impl Iterator<Item=&'a Unit> {
        self.units.iter().filter(move |u| u.unit_type == ty as u32)
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



impl<T> From<T> for RequestAction where T: Into<Action> {
    fn from(t: T) -> Self {
        RequestAction { actions: vec![t.into()] }
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
