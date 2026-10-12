import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import SceneViewport from "./SceneViewport";
import type { SceneDocument } from "./scene";

function fileName(path: string | null) {
  if (!path) return "Starter scene";
  return path.split(/[\\/]/).filter(Boolean).at(-1) ?? path;
}

function dimensions(scene: SceneDocument) {
  return scene.objects.reduce(
    (total, object) => ({
      x: Math.max(total.x, object.geometry.geometry.size.x),
      y: Math.max(total.y, object.geometry.geometry.size.y),
      z: Math.max(total.z, object.geometry.geometry.size.z),
    }),
    { x: 0, y: 0, z: 0 },
  );
}

export default function App() {
  const [scene, setScene] = useState<SceneDocument | null>(null);
  const [path, setPath] = useState<string | null>(null);
  const [selectedObjectId, setSelectedObjectId] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  async function openScene() {
    setError(null);
    const selected = await open({
      multiple: false,
      directory: false,
      filters: [{ name: "Plumb scene", extensions: ["json"] }],
    });
    if (typeof selected !== "string") return;

    setLoading(true);
    try {
      const loaded = await invoke<SceneDocument>("load_scene", { path: selected });
      setScene(loaded);
      setPath(selected);
      setSelectedObjectId(null);
    } catch (reason) {
      setError(String(reason));
    } finally {
      setLoading(false);
    }
  }

  async function openSample() {
    setError(null);
    setLoading(true);
    try {
      const loaded = await invoke<SceneDocument>("load_sample_scene");
      setScene(loaded);
      setPath(null);
      setSelectedObjectId(null);
    } catch (reason) {
      setError(String(reason));
    } finally {
      setLoading(false);
    }
  }

  const size = scene ? dimensions(scene) : null;

  return (
    <main className="app-shell">
      <header className="topbar">
        <div className="brand-lockup" aria-label="Plumb home">
          <span className="brand-mark">P</span>
          <span className="brand-name">plumb</span>
        </div>
        <div className="topbar-divider" />
        <div className="document-label">
          <span className="document-name">{fileName(path)}</span>
          <span className="document-kind">Scene file</span>
        </div>
        <div className="topbar-actions">
          <button className="button button-quiet" onClick={openSample} disabled={loading}>
            Open example
          </button>
          <button className="button button-primary" onClick={openScene} disabled={loading}>
            <span className="button-plus" aria-hidden="true">＋</span>
            {loading ? "Loading…" : "Open scene"}
          </button>
        </div>
      </header>

      <div className="workspace">
        <aside className="scene-panel" aria-label="Scene contents">
          <div className="panel-heading">
            <span>Scene</span>
            <span className="panel-count">{scene?.objects.length ?? 0}</span>
          </div>

          {scene && scene.objects.length > 0 ? (
            <div className="object-list">
              {scene.objects.map((object) => (
                <button
                  className={`object-row${selectedObjectId === object.id ? " object-row-selected" : ""}`}
                  key={object.id}
                  type="button"
                  aria-pressed={selectedObjectId === object.id}
                  onClick={() => setSelectedObjectId(object.id)}
                >
                  <span className="object-glyph" aria-hidden="true">▧</span>
                  <span className="object-name">{object.name}</span>
                  <span className="object-id">{object.id}</span>
                </button>
              ))}
            </div>
          ) : (
            <div className="panel-empty">
              <span className="empty-rule" />
              <p>{scene ? "This scene has no objects." : "Open a scene to see its objects here."}</p>
            </div>
          )}

          {scene && (
            <div className="scene-summary">
              <div className="summary-label">SCENE SUMMARY</div>
              <div className="summary-line"><span>Objects</span><span>{scene.objects.length}</span></div>
              <div className="summary-line"><span>Format</span><span>v{scene.version}</span></div>
              {size && (
                <div className="summary-line"><span>Largest box</span><span>{size.x} × {size.y} × {size.z}</span></div>
              )}
            </div>
          )}
        </aside>

        <section className="viewport-panel" aria-label="Scene view">
          <div className="viewport-toolbar">
            <div className="viewport-title">
              <span className="view-indicator" />
              <span>Perspective</span>
            </div>
            <div className="viewport-help">Drag to orbit <span>·</span> Scroll to zoom <span>·</span> Right-drag to pan</div>
            <div className="viewport-units">SCENE UNITS</div>
          </div>
          <div className="viewport-stage">
            <SceneViewport
              document={scene}
              selectedObjectId={selectedObjectId}
              onSelectObject={setSelectedObjectId}
            />
            {!scene && (
              <div className="welcome-card">
                <div className="welcome-index">A SMALL SPACE TO START</div>
                <h1>Make the shape<br />of an idea.</h1>
                <p>Open a Plumb scene and look around. Start with a sample or choose a local JSON file.</p>
                <div className="welcome-actions">
                  <button className="button button-primary" onClick={openSample} disabled={loading}>Open example scene</button>
                  <button className="text-button" onClick={openScene} disabled={loading}>Choose a file <span aria-hidden="true">↗</span></button>
                </div>
                <div className="welcome-footnote"><span className="lock-mark" aria-hidden="true">⌑</span> Read-only preview · Your scene stays on this device</div>
              </div>
            )}
            {scene?.objects.length === 0 && (
              <div className="empty-viewport-message">
                <span className="empty-orbit" aria-hidden="true" />
                <strong>An empty scene</strong>
                <span>There are no objects to display yet.</span>
              </div>
            )}
            {error && (
              <div className="error-banner" role="alert">
                <span>{error}</span>
                <button onClick={() => setError(null)} aria-label="Dismiss error">×</button>
              </div>
            )}
          </div>
          <footer className="viewport-status">
            <span><span className="status-dot" /> {scene ? "Scene loaded" : "No scene open"}</span>
            <span>
              {selectedObjectId && scene
                ? `Selected: ${scene.objects.find((object) => object.id === selectedObjectId)?.name ?? "Object"}`
                : scene
                  ? `${scene.objects.length} objects`
                  : "Plumb scene viewer"}
            </span>
          </footer>
        </section>
      </div>
    </main>
  );
}
