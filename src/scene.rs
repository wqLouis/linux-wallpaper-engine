struct Scene {}
struct Camera {}
struct General {}
struct Object {
    alpha: Option<f64>,
    castshadow: Option<bool>,
    id: u64,
    image: Option<String>,
    name: String,
    origin: String,
    size: Option<String>,
    visible: Option<bool>,
    angles: Option<String>,
    instanceoverride: Option<Instanceoverride>,
    constantshadervalues: Option<Vec<Object>>,
    animationplayers: Option<Vec<Animationplayer>>,
    effects: Option<Vec<Object>>,
    textures: Option<Vec<String>>,
    passes: Option<Vec<Object>>,
    scale: Option<String>,
    particle: Option<String>,
}
struct Instanceoverride {}
struct Animationplayer {}
