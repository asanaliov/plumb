use plumb_core::{Geometry, Scene, Vec3};
use serde::Serialize;
use std::{fs, path::PathBuf};

#[derive(Serialize)]
struct SceneView {
    version: u32,
    objects: Vec<ObjectView>,
}

#[derive(Serialize)]
struct ObjectView {
    id: String,
    name: String,
    position: Vec3,
    geometry: Geometry,
}

impl From<Scene> for SceneView {
    fn from(scene: Scene) -> Self {
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

        Self {
            version: scene.version(),
            objects,
        }
    }
}

#[tauri::command]
fn load_scene(path: String) -> Result<SceneView, String> {
    let scene_path = PathBuf::from(&path);
    let contents = fs::read_to_string(&scene_path)
        .map_err(|error| format!("could not read '{}': {error}", scene_path.display()))?;
    Scene::from_json(&contents)
        .map(SceneView::from)
        .map_err(|error| error.to_string())
}

#[tauri::command]
fn load_sample_scene() -> Result<SceneView, String> {
    Scene::from_json(include_str!("../../../../examples/starter-scene.json"))
        .map(SceneView::from)
        .map_err(|error| error.to_string())
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![load_scene, load_sample_scene])
        .run(tauri::generate_context!())
        .expect("error while running Plumb desktop");
}
