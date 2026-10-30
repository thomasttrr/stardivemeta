// The WebGL layer behind every page (React Three Fiber + Three.js).
//
// - <Field>: one full-viewport shader that paints the deep-sea field and
//   lenses every [data-glass] panel's live screen rect (glass-shader.ts).
// - <GlassStar>: a physically based, transmissive glass star that floats
//   in the home hero ([data-star] anchor) and refracts the field.
//
// Decorative only: it reads element positions, never page data, and the
// page is complete without it — CSS glass stays until the first frame
// renders (then <html class="gl">), and stays for good where WebGL is
// unavailable. prefers-reduced-motion freezes time; frames then render
// only on scroll/resize/pointer.
import { useEffect, useMemo, useRef, useState } from 'react';
import { Canvas, useFrame, useThree } from '@react-three/fiber';
import * as THREE from 'three';
import { RoomEnvironment } from 'three/examples/jsm/environments/RoomEnvironment.js';
import { MAX_RECTS, fragmentShader, vertexShader } from './glass-shader';
import { readFrost } from './frost';

const GLASS_SELECTOR = '[data-glass]:not([data-glass="overlay"])';

interface Shared {
  pointer: THREE.Vector2;
  reduced: boolean;
}

function webglAvailable(): boolean {
  try {
    const c = document.createElement('canvas');
    return !!c.getContext('webgl2');
  } catch {
    return false;
  }
}

/** Orthographic camera in CSS pixels: x right, y down (as world -y). */
function PixelCamera() {
  const camera = useThree((s) => s.camera) as THREE.OrthographicCamera & { manual?: boolean };
  const size = useThree((s) => s.size);
  useEffect(() => {
    camera.manual = true;
    camera.left = 0;
    camera.right = size.width;
    camera.top = 0;
    camera.bottom = -size.height;
    camera.near = 1;
    camera.far = 4000;
    camera.position.set(0, 0, 1500);
    camera.updateProjectionMatrix();
  }, [camera, size.width, size.height]);
  return null;
}

/** Drives frames: continuous unless reduced motion; always on input. */
function FrameDriver({ shared, small }: { shared: Shared; small: boolean }) {
  const invalidate = useThree((s) => s.invalidate);
  useEffect(() => {
    let raf = 0;
    let tick = 0;
    const loop = () => {
      tick++;
      // ~30 fps on small screens, full rate elsewhere
      if (!small || tick % 2 === 0) invalidate();
      raf = requestAnimationFrame(loop);
    };
    if (!shared.reduced) raf = requestAnimationFrame(loop);

    const onMove = (e: PointerEvent) => {
      shared.pointer.set(e.clientX, e.clientY);
      const chip = (e.target as Element | null)?.closest?.('.chip') as HTMLElement | null;
      if (chip) {
        const r = chip.getBoundingClientRect();
        chip.style.setProperty('--mx', `${e.clientX - r.left}px`);
        chip.style.setProperty('--my', `${e.clientY - r.top}px`);
      }
      invalidate();
    };
    const onChange = () => invalidate();
    window.addEventListener('pointermove', onMove, { passive: true });
    window.addEventListener('scroll', onChange, { passive: true });
    window.addEventListener('resize', onChange);
    // level switches and dialogs change which panels are visible
    const mo = new MutationObserver(onChange);
    mo.observe(document.documentElement, { attributes: true, attributeFilter: ['data-level'] });
    return () => {
      cancelAnimationFrame(raf);
      window.removeEventListener('pointermove', onMove);
      window.removeEventListener('scroll', onChange);
      window.removeEventListener('resize', onChange);
      mo.disconnect();
    };
  }, [invalidate, shared, small]);
  return null;
}

function Field({ shared }: { shared: Shared }) {
  const size = useThree((s) => s.size);
  const gl = useThree((s) => s.gl);
  const elements = useRef<HTMLElement[]>([]);
  const radii = useRef(new WeakMap<HTMLElement, number>());
  const ready = useRef(false);

  const material = useMemo(
    () =>
      new THREE.ShaderMaterial({
        vertexShader,
        fragmentShader,
        depthWrite: false,
        uniforms: {
          uTime: { value: 0 },
          uRes: { value: new THREE.Vector2(1, 1) },
          uDpr: { value: 1 },
          uScroll: { value: 0 },
          uPointer: { value: new THREE.Vector2(-9999, -9999) },
          uCount: { value: 0 },
          uRects: { value: Array.from({ length: MAX_RECTS }, () => new THREE.Vector4()) },
          uRadii: { value: new Array<number>(MAX_RECTS).fill(0) },
          uFrost: { value: 1 },
          uTint: { value: 1 },
        },
      }),
    [],
  );

  useEffect(() => {
    const refresh = () => {
      elements.current = Array.from(document.querySelectorAll<HTMLElement>(GLASS_SELECTOR));
      radii.current = new WeakMap();
      const f = readFrost();
      material.uniforms.uFrost.value = f.frost;
      material.uniforms.uTint.value = f.tint;
    };
    refresh();
    window.addEventListener('resize', refresh);
    return () => {
      window.removeEventListener('resize', refresh);
      material.dispose();
      document.documentElement.classList.remove('gl');
    };
  }, [material]);

  useEffect(() => {
    const canvas = gl.domElement;
    const lost = (e: Event) => {
      e.preventDefault();
      document.documentElement.classList.remove('gl');
    };
    canvas.addEventListener('webglcontextlost', lost);
    return () => canvas.removeEventListener('webglcontextlost', lost);
  }, [gl]);

  const buffer = useMemo(() => new THREE.Vector2(), []);
  useFrame((state) => {
    const u = material.uniforms;
    gl.getDrawingBufferSize(buffer);
    u.uRes.value.copy(buffer);
    u.uDpr.value = gl.getPixelRatio();
    u.uTime.value = shared.reduced ? 24 : state.clock.elapsedTime + 24;
    u.uScroll.value = window.scrollY;
    u.uPointer.value.copy(shared.pointer);

    const vh = window.innerHeight;
    const rects = u.uRects.value as THREE.Vector4[];
    const rad = u.uRadii.value as number[];
    let n = 0;
    for (const el of elements.current) {
      if (n >= MAX_RECTS) break;
      const r = el.getBoundingClientRect();
      if (r.width < 2 || r.height < 2 || r.bottom < -60 || r.top > vh + 60) continue;
      let radius = radii.current.get(el);
      if (radius === undefined) {
        radius = parseFloat(getComputedStyle(el).borderTopLeftRadius) || 0;
        radii.current.set(el, radius);
      }
      rects[n].set(r.left, r.top, r.width, r.height);
      rad[n] = radius;
      n++;
    }
    u.uCount.value = n;

    if (!ready.current) {
      ready.current = true;
      requestAnimationFrame(() => document.documentElement.classList.add('gl'));
    }
  });

  return (
    <mesh
      material={material}
      frustumCulled={false}
      position={[size.width / 2, -size.height / 2, -1000]}
      scale={[size.width, size.height, 1]}
    >
      <planeGeometry args={[1, 1]} />
    </mesh>
  );
}

function starGeometry(): THREE.ExtrudeGeometry {
  const s = new THREE.Shape();
  const k = 0.2;
  s.moveTo(0, 1);
  s.quadraticCurveTo(k, k, 1, 0);
  s.quadraticCurveTo(k, -k, 0, -1);
  s.quadraticCurveTo(-k, -k, -1, 0);
  s.quadraticCurveTo(-k, k, 0, 1);
  const g = new THREE.ExtrudeGeometry(s, {
    depth: 0.16,
    bevelEnabled: true,
    bevelThickness: 0.18,
    bevelSize: 0.07,
    bevelSegments: 14,
    curveSegments: 48,
  });
  g.center();
  g.computeVertexNormals();
  return g;
}

function Environment() {
  const gl = useThree((s) => s.gl);
  const scene = useThree((s) => s.scene);
  useEffect(() => {
    const pmrem = new THREE.PMREMGenerator(gl);
    const env = pmrem.fromScene(new RoomEnvironment(), 0.04).texture;
    scene.environment = env;
    return () => {
      scene.environment = null;
      env.dispose();
      pmrem.dispose();
    };
  }, [gl, scene]);
  return null;
}

function GlassStar({ shared, anchor }: { shared: Shared; anchor: HTMLElement }) {
  const mesh = useRef<THREE.Mesh>(null);
  const geometry = useMemo(starGeometry, []);
  useEffect(() => () => geometry.dispose(), [geometry]);

  useFrame((state) => {
    const m = mesh.current;
    if (!m) return;
    const r = anchor.getBoundingClientRect();
    const visible = r.width > 0 && r.bottom > -100 && r.top < window.innerHeight + 100;
    m.visible = visible;
    if (!visible) return;
    const cx = r.left + r.width / 2;
    const cy = r.top + r.height / 2;
    const scale = (Math.min(r.width, r.height) / 2) * 0.86;
    const t = shared.reduced ? 0 : state.clock.elapsedTime;
    const px = shared.pointer.x < -1000 ? 0 : (shared.pointer.x - cx) / window.innerWidth;
    const py = shared.pointer.y < -1000 ? 0 : (shared.pointer.y - cy) / window.innerHeight;
    m.position.set(cx, -cy + Math.sin(t * 0.9) * 5, 0);
    m.scale.setScalar(scale);
    m.rotation.set(
      0.18 + py * 0.7 + Math.sin(t * 0.5) * 0.1,
      Math.sin(t * 0.3) * 0.75 + px * 1.1,
      -window.scrollY * 0.0015 + 0.15,
    );
  });

  return (
    <mesh ref={mesh} geometry={geometry}>
      <meshPhysicalMaterial
        color="#ffffff"
        metalness={0}
        roughness={0.04}
        transmission={1}
        thickness={34}
        ior={1.5}
        dispersion={8}
        iridescence={1}
        iridescenceIOR={1.32}
        iridescenceThicknessRange={[160, 560]}
        clearcoat={1}
        clearcoatRoughness={0.02}
        attenuationColor="#fff1d2"
        attenuationDistance={900}
        emissive="#ffc35c"
        emissiveIntensity={0.09}
        envMapIntensity={4}
        specularIntensity={1}
        specularColor="#ffffff"
      />
    </mesh>
  );
}

export default function GlassScene() {
  const [mounted, setMounted] = useState(false);
  const [anchor, setAnchor] = useState<HTMLElement | null>(null);
  const shared = useMemo<Shared>(
    () => ({ pointer: new THREE.Vector2(-9999, -9999), reduced: false }),
    [],
  );
  const [small, setSmall] = useState(false);

  useEffect(() => {
    if (!webglAvailable()) return;
    shared.reduced = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
    setSmall(window.matchMedia('(max-width: 700px)').matches);
    setAnchor(document.querySelector<HTMLElement>('[data-star]'));
    setMounted(true);
  }, [shared]);

  if (!mounted) return null;
  return (
    <div className="scene" aria-hidden="true">
      <Canvas
        orthographic
        frameloop="demand"
        dpr={[1, 1.5]}
        gl={{ antialias: true, alpha: false, powerPreference: 'high-performance' }}
        onCreated={({ gl }) => {
          gl.setClearColor('#050d10');
          gl.toneMapping = THREE.ACESFilmicToneMapping;
          gl.toneMappingExposure = 1.05;
        }}
        style={{ position: 'absolute', inset: 0, pointerEvents: 'none' }}
      >
        <PixelCamera />
        <FrameDriver shared={shared} small={small} />
        <Field shared={shared} />
        {anchor && (
          <>
            <Environment />
            <directionalLight position={[-400, 600, 900]} intensity={3} />
            <directionalLight position={[600, -300, 500]} intensity={1.2} color="#7fe3c4" />
            <GlassStar shared={shared} anchor={anchor} />
          </>
        )}
      </Canvas>
    </div>
  );
}
