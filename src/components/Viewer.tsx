import { useEffect, useRef } from "react";
import * as THREE from "three";
import { OrbitControls } from "three/addons/controls/OrbitControls.js";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button";
import { X } from "lucide-react";

interface GeometryData {
  vertices: [number, number, number][];
  faces: [number, number, number][];
  vertex_count: number;
  face_count: number;
}

interface ViewerProps {
  file: string;
  onClose: () => void;
}

export default function Viewer({ file, onClose }: ViewerProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const initializedRef = useRef(false);

  useEffect(() => {
    if (initializedRef.current) return;
    initializedRef.current = true;

    let renderer: THREE.WebGLRenderer | null = null;
    let frameId: number;
    let handleResize: (() => void) | null = null;

    const setup = async () => {
      try {
        const data = await invoke<GeometryData>("get_geometry", { input: file });

        if (!containerRef.current || data.vertices.length === 0) return;

        const width = containerRef.current.clientWidth;
        const height = containerRef.current.clientHeight;

        const scene = new THREE.Scene();
        scene.background = new THREE.Color(0x18181b);

        const camera = new THREE.PerspectiveCamera(50, width / height, 0.1, 1000);
        camera.position.set(0, 0, 5);

        renderer = new THREE.WebGLRenderer({ antialias: true });
        renderer.setSize(width, height);
        containerRef.current.appendChild(renderer.domElement);

        const geometry = new THREE.BufferGeometry();
        const vertices = new Float32Array(data.vertices.length * 3);
        data.vertices.forEach((v, i) => {
          vertices[i * 3] = v[0];
          vertices[i * 3 + 1] = v[1];
          vertices[i * 3 + 2] = v[2];
        });
        const indices = new Uint32Array(data.faces.length * 3);
        data.faces.forEach((f, i) => {
          indices[i * 3] = f[0];
          indices[i * 3 + 1] = f[1];
          indices[i * 3 + 2] = f[2];
        });
        geometry.setAttribute("position", new THREE.BufferAttribute(vertices, 3));
        geometry.setIndex(new THREE.BufferAttribute(indices, 1));
        geometry.computeVertexNormals();

        // Flat shaded surface, neutral gray so the wireframe pops
        const material = new THREE.MeshStandardMaterial({
          color: 0x4b5563,
          roughness: 0.7,
          metalness: 0.05,
          side: THREE.DoubleSide,
          transparent: true,
          opacity: 0.85,
          flatShading: true,
        });
        const mesh = new THREE.Mesh(geometry, material);
        scene.add(mesh);

        // Wireframe overlay in bright green
        const wireframe = new THREE.LineSegments(
          new THREE.WireframeGeometry(geometry),
          new THREE.LineBasicMaterial({ color: 0x4ade80, linewidth: 1 })
        );
        scene.add(wireframe);

        // Lights
        const ambient = new THREE.AmbientLight(0xffffff, 0.6);
        scene.add(ambient);
        const directional = new THREE.DirectionalLight(0xffffff, 0.8);
        directional.position.set(5, 10, 7);
        scene.add(directional);

        // Center and scale so the object fits in a normalized view
        geometry.computeBoundingBox();
        const box = geometry.boundingBox;
        if (box) {
          const center = box.getCenter(new THREE.Vector3());
          const size = box.getSize(new THREE.Vector3());
          const maxDim = Math.max(size.x, size.y, size.z);
          const scale = maxDim > 0 ? 3 / maxDim : 1;
          mesh.scale.setScalar(scale);
          wireframe.scale.setScalar(scale);
          mesh.position.sub(center.clone().multiplyScalar(scale));
          wireframe.position.copy(mesh.position);
        }
        camera.position.z = 6;

        const controls = new OrbitControls(camera, renderer.domElement);
        controls.enableDamping = true;
        controls.dampingFactor = 0.05;
        controls.target.set(0, 0, 0);

        handleResize = () => {
          if (!containerRef.current || !renderer) return;
          const w = containerRef.current.clientWidth;
          const h = containerRef.current.clientHeight;
          camera.aspect = w / h;
          camera.updateProjectionMatrix();
          renderer.setSize(w, h);
        };
        window.addEventListener("resize", handleResize);

        const animate = () => {
          frameId = requestAnimationFrame(animate);
          controls.update();
          renderer?.render(scene, camera);
        };
        animate();
      } catch (e) {
        console.error("Failed to load geometry preview:", e);
      }
    };

    setup();

    return () => {
      if (handleResize) {
        window.removeEventListener("resize", handleResize);
      }
      cancelAnimationFrame(frameId);
      if (renderer?.domElement && renderer.domElement.parentElement) {
        renderer.domElement.parentElement.removeChild(renderer.domElement);
      }
      renderer?.dispose();
    };
  }, [file]);

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-6">
      <div className="flex h-[90vh] w-[95vw] flex-col rounded-lg border bg-card shadow-lg">
        <div className="flex items-center justify-between border-b p-4">
          <div>
            <h2 className="text-lg font-semibold">Vorschau</h2>
            <p className="text-sm text-muted-foreground truncate">{file}</p>
          </div>
          <Button variant="ghost" size="icon" onClick={onClose}>
            <X className="h-4 w-4" />
          </Button>
        </div>
        <div ref={containerRef} className="flex-1 min-h-0" />
      </div>
    </div>
  );
}
