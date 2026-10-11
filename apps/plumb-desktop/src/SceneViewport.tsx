import { useEffect, useRef } from "react";
import * as THREE from "three";
import { OrbitControls } from "three/addons/controls/OrbitControls.js";
import type { SceneDocument } from "./scene";

interface SceneViewportProps {
  document: SceneDocument | null;
}

export default function SceneViewport({ document }: SceneViewportProps) {
  const host = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const element = host.current;
    if (!element) return;

    const scene = new THREE.Scene();
    scene.background = new THREE.Color("#171b1a");
    const camera = new THREE.PerspectiveCamera(42, 1, 0.1, 1000);
    const renderer = new THREE.WebGLRenderer({ antialias: true, alpha: false });
    renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
    renderer.outputColorSpace = THREE.SRGBColorSpace;
    renderer.toneMapping = THREE.ACESFilmicToneMapping;
    renderer.toneMappingExposure = 1.15;
    element.appendChild(renderer.domElement);

    const controls = new OrbitControls(camera, renderer.domElement);
    controls.enableDamping = true;
    controls.dampingFactor = 0.08;
    controls.enablePan = true;
    controls.minDistance = 2;
    controls.maxDistance = 200;

    scene.add(new THREE.HemisphereLight("#edf2e9", "#4b524e", 2.1));
    const keyLight = new THREE.DirectionalLight("#fff4dc", 3.1);
    keyLight.position.set(7, 11, 8);
    scene.add(keyLight);

    if (document) {
      for (const object of document.objects) {
        if (object.geometry.type !== "box") continue;
        const size = object.geometry.geometry.size;
        const mesh = new THREE.Mesh(
          new THREE.BoxGeometry(size.x, size.y, size.z),
          new THREE.MeshStandardMaterial({
            color: "#a7c97a",
            roughness: 0.72,
            metalness: 0.04,
          }),
        );
        mesh.position.set(object.position.x, object.position.y, object.position.z);
        mesh.userData.objectId = object.id;
        mesh.castShadow = true;
        mesh.receiveShadow = true;
        scene.add(mesh);
      }

      if (document.objects.length > 0) {
        const bounds = new THREE.Box3().setFromObject(scene);
        const center = bounds.getCenter(new THREE.Vector3());
        const size = bounds.getSize(new THREE.Vector3());
        const radius = Math.max(size.length() * 0.72, 2.5);
        controls.target.copy(center);
        camera.position.set(
          center.x + radius * 0.9,
          center.y + radius * 0.72,
          center.z + radius * 1.25,
        );
        camera.near = Math.max(radius / 100, 0.01);
        camera.far = radius * 100;
        camera.updateProjectionMatrix();
        controls.update();
      }
    }

    const resizeObserver = new ResizeObserver(() => {
      const { width, height } = element.getBoundingClientRect();
      if (width === 0 || height === 0) return;
      renderer.setSize(width, height, false);
      camera.aspect = width / height;
      camera.updateProjectionMatrix();
    });
    resizeObserver.observe(element);

    let animationFrame = 0;
    const render = () => {
      controls.update();
      renderer.render(scene, camera);
      animationFrame = window.requestAnimationFrame(render);
    };
    render();

    return () => {
      window.cancelAnimationFrame(animationFrame);
      resizeObserver.disconnect();
      controls.dispose();
      scene.traverse((node) => {
        if (node instanceof THREE.Mesh) {
          node.geometry.dispose();
          const materials = Array.isArray(node.material) ? node.material : [node.material];
          materials.forEach((material) => material.dispose());
        }
      });
      renderer.dispose();
      renderer.domElement.remove();
    };
  }, [document]);

  return <div className="viewport-canvas" ref={host} aria-label="3D scene viewport" />;
}
