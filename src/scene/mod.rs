pub mod camera;
pub mod models;
pub mod renderer;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub camera: Camera,
    pub general: General,
    pub objects: Vec<Object>,
    pub version: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Camera {
    pub center: Vectors,
    pub eye: Vectors,
    pub up: Vectors,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct General {
    pub ambientcolor: Vectors,
    pub bloom: bool,
    pub bloomhdrfeather: f64,
    pub bloomhdriterations: i64,
    pub bloomhdrscatter: f64,
    pub bloomhdrstrength: f64,
    pub bloomhdrthreshold: f64,
    pub bloomstrength: f64,
    pub bloomthreshold: f64,
    pub bloomtint: Vectors,
    pub camerafade: bool,
    pub cameraparallax: bool,
    pub cameraparallaxamount: f64,
    pub cameraparallaxdelay: f64,
    pub cameraparallaxmouseinfluence: f64,
    pub camerapreview: bool,
    pub camerashake: Camerashake,
    pub camerashakeamplitude: f64,
    pub camerashakeroughness: f64,
    pub camerashakespeed: f64,
    pub clearcolor: Vectors,
    pub clearenabled: bool,
    pub farz: f64,
    pub fov: f64,
    pub gravitydirection: Vectors,
    pub gravitystrength: f64,
    pub hdr: bool,
    pub nearz: f64,
    pub orthogonalprojection: Orthogonalprojection,
    pub perspectiveoverridefov: f64,
    pub skylightcolor: Vectors,
    pub winddirection: Vectors,
    pub windenabled: bool,
    pub windstrength: f64,
    pub zoom: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Camerashake {
    pub user: String,
    pub value: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Orthogonalprojection {
    pub height: i64,
    pub width: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Object {
    pub castshadow: Option<bool>,
    pub clampuvs: Option<bool>,
    pub disablepropagation: bool,
    pub id: i64,
    pub image: Option<String>,
    pub locktransforms: Option<bool>,
    pub name: String,
    pub origin: Option<Vectors>,
    pub scale: Option<Vectors>,
    pub size: Option<Vectors>,
    #[serde(default)]
    pub effects: Vec<Effect>,
    pub alpha: Option<f64>,
    pub angles: Option<Vectors>,
    pub instanceoverride: Option<Instanceoverride>,
    pub particle: Option<String>,
    pub solid: Option<bool>,
    pub visible: Option<Visible>,
    #[serde(default)]
    pub animationlayers: Vec<Animationlayer>,
    pub attachment: Option<String>,
    pub parent: Option<i64>,
    pub maxtime: Option<f64>,
    pub mintime: Option<f64>,
    pub muteineditor: Option<bool>,
    pub playbackmode: Option<String>,
    #[serde(default)]
    pub sound: Vec<String>,
    pub startsilent: Option<bool>,
    pub volume: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Effect {
    pub file: String,
    pub id: i64,
    pub name: String,
    pub passes: Vec<Pass>,
    pub visible: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pass {
    pub constantshadervalues: Option<Constantshadervalues>,
    pub id: i64,
    pub combos: Option<Combos>,
    #[serde(default)]
    pub textures: Vec<Option<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Constantshadervalues {
    pub direction: Option<f64>,
    pub exponent: Option<f64>,
    pub scale: Option<f64>,
    pub speed: Option<f64>,
    pub strength: Option<f64>,
    pub bounds: Option<String>,
    pub friction: Option<String>,
    pub alpha: Option<f64>,
    pub repeat: Option<String>,
    pub speedx: Option<f64>,
    pub speedy: Option<i64>,
    pub color: Option<String>,
    pub animationspeed: Option<f64>,
    pub ratio: Option<i64>,
    pub ripplestrength: Option<f64>,
    pub scrolldirection: Option<f64>,
    pub scrollspeed: Option<f64>,
    pub point0: Option<String>,
    pub point1: Option<String>,
    pub point2: Option<String>,
    pub point3: Option<String>,
    #[serde(rename = "Aperture")]
    pub aperture: Option<f64>,
    #[serde(rename = "Opacity")]
    pub opacity: Option<Opacity>,
    #[serde(rename = "Gamma")]
    pub gamma: Option<f64>,
    #[serde(rename = "Highlights")]
    pub highlights: Option<f64>,
    #[serde(rename = "Tint")]
    pub tint: Option<String>,
    #[serde(rename = "opacity")]
    pub opacity2: Option<f64>,
    pub radius: Option<i64>,
    #[serde(rename = "gamma")]
    pub gamma2: Option<i64>,
    pub threshold: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Opacity {
    pub user: String,
    pub value: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Combos {
    #[serde(rename = "VERTICAL")]
    pub vertical: Option<i64>,
    #[serde(rename = "PRECISE")]
    pub precise: Option<i64>,
    #[serde(rename = "BLENDMODE")]
    pub blendmode: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instanceoverride {
    pub count: f64,
    pub id: i64,
    pub lifetime: Option<f64>,
    pub rate: Option<f64>,
    pub size: Option<f64>,
    pub speed: f64,
    pub alpha: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Visible {
    pub user: Value,
    pub value: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Animationlayer {
    pub additive: bool,
    pub animation: i64,
    pub blend: f64,
    pub blendin: bool,
    pub blendout: bool,
    pub blendtime: f64,
    pub id: i64,
    pub name: String,
    pub rate: f64,
    pub visible: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum Vectors {
    Scaler(f64),
    Vectors(String),
}

impl Default for Vectors {
    fn default() -> Self {
        Vectors::Scaler(0.0)
    }
}

impl Vectors {
    pub fn parse(&self) -> Option<Vec<f64>> {
        match self {
            Vectors::Scaler(s) => Some(vec![s.to_owned()]),
            Vectors::Vectors(s) => s
                .split_whitespace()
                .into_iter()
                .map(|f| f.parse::<f64>().ok())
                .collect(),
        }
    }
}
