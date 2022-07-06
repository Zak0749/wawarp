use bevy::reflect::TypeUuid;
use serde::Deserialize;

#[derive(Deserialize, TypeUuid, Debug)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c46"]
pub struct Level {
    pub level: i32,
    pub background: String,
    pub width: i32,
    pub height: i32,
    pub components: Vec<Component>,
}

#[derive(serde::Deserialize, TypeUuid, Debug)]
#[uuid = "371a0762-faf2-11ec-b939-0242ac120002"]
pub struct Component {
    pub component: ComponentType,
    pub id: String,
    pub x: i32,
    pub y: i32,
    pub action: Action,
}

#[derive(serde::Deserialize, TypeUuid, Debug)]
#[uuid = "71b9289b-a56a-475e-b3ce-b7e50c143ca2"]
#[serde(tag = "type", content = "link")]
pub enum Action {
    Door(String),
    Button(String),
    NextLevel,
}

#[derive(serde::Deserialize, TypeUuid, Debug)]
#[uuid = "4121c736-faf2-11ec-b939-0242ac120002"]
pub enum ComponentType {
    Button,
    Door,
}
