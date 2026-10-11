use serde::{Deserialize, Serialize};
use std::fmt;

/// A stable identifier for an object in a scene.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ObjectId(u64);

impl ObjectId {
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl From<u64> for ObjectId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoxGeometry {
    pub size: Vec3,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "geometry", rename_all = "snake_case")]
pub enum Geometry {
    Box(BoxGeometry),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Object {
    pub id: ObjectId,
    pub name: String,
    pub position: Vec3,
    pub geometry: Geometry,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scene {
    version: u32,
    next_id: u64,
    objects: Vec<Object>,
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            version: 1,
            next_id: 1,
            objects: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SceneError {
    InvalidName,
    InvalidDimensions,
    InvalidPosition,
    ObjectNotFound(ObjectId),
    InvalidScene(String),
    Serialization(String),
}

impl fmt::Display for SceneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidName => write!(f, "object name must not be empty"),
            Self::InvalidDimensions => {
                write!(f, "box dimensions must be finite and greater than zero")
            }
            Self::InvalidPosition => write!(f, "position coordinates must be finite"),
            Self::ObjectNotFound(id) => write!(f, "object {} was not found", id.get()),
            Self::InvalidScene(message) => write!(f, "invalid scene: {message}"),
            Self::Serialization(message) => write!(f, "scene serialization failed: {message}"),
        }
    }
}

impl std::error::Error for SceneError {}

impl Scene {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn objects(&self) -> &[Object] {
        &self.objects
    }
    pub fn get(&self, id: ObjectId) -> Option<&Object> {
        self.objects.iter().find(|object| object.id == id)
    }

    pub fn add_box(&mut self, name: impl Into<String>, size: Vec3) -> Result<ObjectId, SceneError> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(SceneError::InvalidName);
        }
        if !size.is_finite() || size.x <= 0.0 || size.y <= 0.0 || size.z <= 0.0 {
            return Err(SceneError::InvalidDimensions);
        }
        let id = ObjectId(self.next_id);
        self.next_id = self
            .next_id
            .checked_add(1)
            .ok_or_else(|| SceneError::InvalidScene("object id space is exhausted".into()))?;
        self.objects.push(Object {
            id,
            name,
            position: Vec3::ZERO,
            geometry: Geometry::Box(BoxGeometry { size }),
        });
        Ok(id)
    }

    pub fn set_position(&mut self, id: ObjectId, position: Vec3) -> Result<(), SceneError> {
        if !position.is_finite() {
            return Err(SceneError::InvalidPosition);
        }
        let object = self
            .objects
            .iter_mut()
            .find(|object| object.id == id)
            .ok_or(SceneError::ObjectNotFound(id))?;
        object.position = position;
        Ok(())
    }

    pub fn to_json(&self) -> Result<String, SceneError> {
        serde_json::to_string_pretty(self)
            .map_err(|error| SceneError::Serialization(error.to_string()))
    }

    pub fn from_json(json: &str) -> Result<Self, SceneError> {
        let scene: Self = serde_json::from_str(json)
            .map_err(|error| SceneError::Serialization(error.to_string()))?;
        scene.validate()?;
        Ok(scene)
    }

    fn validate(&self) -> Result<(), SceneError> {
        if self.version != 1 {
            return Err(SceneError::InvalidScene(format!(
                "unsupported version {}",
                self.version
            )));
        }
        let mut ids = std::collections::HashSet::new();
        for object in &self.objects {
            if !ids.insert(object.id) {
                return Err(SceneError::InvalidScene("duplicate object id".into()));
            }
            if object.id.0 == 0 || object.id.0 >= self.next_id {
                return Err(SceneError::InvalidScene(
                    "next_id must be greater than every object id".into(),
                ));
            }
            if object.name.trim().is_empty() {
                return Err(SceneError::InvalidName);
            }
            if !object.position.is_finite() {
                return Err(SceneError::InvalidPosition);
            }
            match &object.geometry {
                Geometry::Box(geometry)
                    if geometry.size.is_finite()
                        && geometry.size.x > 0.0
                        && geometry.size.y > 0.0
                        && geometry.size.z > 0.0 =>
                {
                    ()
                }
                Geometry::Box(_) => return Err(SceneError::InvalidDimensions),
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_transforms_and_round_trips_objects() {
        let mut scene = Scene::new();
        let id = scene.add_box("Block", Vec3::new(2.0, 3.0, 4.0)).unwrap();
        scene.set_position(id, Vec3::new(1.0, 2.0, 3.0)).unwrap();
        let decoded = Scene::from_json(&scene.to_json().unwrap()).unwrap();
        assert_eq!(decoded, scene);
        assert_eq!(decoded.get(id).unwrap().position, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn rejects_invalid_geometry_and_positions() {
        let mut scene = Scene::new();
        assert_eq!(
            scene.add_box("bad", Vec3::new(1.0, 0.0, 1.0)),
            Err(SceneError::InvalidDimensions)
        );
        let id = scene.add_box("good", Vec3::new(1.0, 1.0, 1.0)).unwrap();
        assert_eq!(
            scene.set_position(id, Vec3::new(f64::NAN, 0.0, 0.0)),
            Err(SceneError::InvalidPosition)
        );
    }

    #[test]
    fn rejects_corrupt_scene_ids() {
        let json = r#"{"version":1,"next_id":1,"objects":[{"id":1,"name":"bad","position":{"x":0.0,"y":0.0,"z":0.0},"geometry":{"type":"box","geometry":{"size":{"x":1.0,"y":1.0,"z":1.0}}}}]}"#;
        assert!(matches!(
            Scene::from_json(json),
            Err(SceneError::InvalidScene(_))
        ));
    }
}
