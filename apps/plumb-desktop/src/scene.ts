export interface Vec3 {
  x: number;
  y: number;
  z: number;
}

export interface BoxObject {
  id: string;
  name: string;
  position: Vec3;
  geometry: {
    type: "box";
    geometry: { size: Vec3 };
  };
}

export interface SceneDocument {
  version: number;
  objects: BoxObject[];
}
