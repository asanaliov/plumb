use plumb_core::{Geometry, Scene, Vec3};
use serde::Serialize;
use std::{fs, path::PathBuf};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SceneView {
    version: u32,
    objects: Vec<ObjectView>,
    scene_json: String,
}

#[derive(Serialize)]
struct ObjectView {
    id: String,
    name: String,
    position: Vec3,
    geometry: Geometry,
}

impl TryFrom<Scene> for SceneView {
    type Error = String;

    fn try_from(scene: Scene) -> Result<Self, Self::Error> {
        let scene_json = scene.to_json().map_err(|error| error.to_string())?;
        let objects = scene
            .objects()
            .iter()
            .map(|object| ObjectView {
                id: object.id.get().to_string(),
                name: object.name.clone(),
                position: object.position,
                geometry: object.geometry.clone(),
            })
            .collect();

        Ok(Self {
            version: scene.version(),
            objects,
            scene_json,
        })
    }
}

#[tauri::command]
fn load_scene(path: String) -> Result<SceneView, String> {
    let scene_path = PathBuf::from(&path);
    let contents = fs::read_to_string(&scene_path)
        .map_err(|error| format!("could not read '{}': {error}", scene_path.display()))?;
    let scene = Scene::from_json(&contents).map_err(|error| error.to_string())?;
    SceneView::try_from(scene)
}

#[tauri::command]
fn load_sample_scene() -> Result<SceneView, String> {
    let scene = Scene::from_json(include_str!("../../../../examples/starter-scene.json"))
        .map_err(|error| error.to_string())?;
    SceneView::try_from(scene)
}

#[tauri::command]
fn move_object(scene_json: String, object_id: String, position: Vec3) -> Result<SceneView, String> {
    let id = object_id
        .parse::<u64>()
        .map_err(|_| format!("invalid object id '{object_id}'"))?;
    let mut scene = Scene::from_json(&scene_json).map_err(|error| error.to_string())?;
    scene
        .set_position(id.into(), position)
        .map_err(|error| error.to_string())?;
    SceneView::try_from(scene)
}

#[tauri::command]
fn save_scene(path: String, scene_json: String) -> Result<(), String> {
    let scene = Scene::from_json(&scene_json).map_err(|error| error.to_string())?;
    let serialized = scene.to_json().map_err(|error| error.to_string())?;
    fs::write(&path, serialized).map_err(|error| format!("could not save '{path}': {error}"))
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            load_scene,
            load_sample_scene,
            move_object,
            save_scene
        ])
        .run(tauri::generate_context!())
        .expect("error while running Plumb desktop");
}
