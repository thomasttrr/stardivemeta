# StarDiveMeta — Stack v2 Proposal: "Rust core, web-standard shell" (2026-10-29)
> **Status note (Tom, 2026-10-29 ~19:09 ET):** decision record, NOT a permanent lock. Input to the Opus 5.5 planning pass (see `stardivemeta/00-MASTER-HANDOFF-for-Opus.md`); may be revised there with reasons stated.

Status: ADOPTED / LOCKED 2026-10-29 ~18:50 ET (Tom's tap) — revises only
the FRONT-END part of the locked architecture (supersedes Askama templates
+ the Leptos island; the Rust data/method layer is unchanged).
Everything else already locked is untouched: Method v1.0,
Package B scope, StarDiveMeta name, Cloudflare Pages, update workflow,
launch guardrails, no server, static output.

## Why the change
Tom asked for complete insurance: rich theming, award-level animation,
possible 3D models, floating animated (2D) characters, and no outdated-stack
regret years from now. The honest finding: the tools for 3D, pro animation,
and animated characters live in the JavaScript/TypeScript ecosystem —
Three.js (3D), GSAP (animation, now fully free incl. all plugins since the
Webflow acquisition, Apr 2025), Live2D Cubism Web SDK (TypeScript/WebGL),
Rive and Lottie web runtimes, dnd-kit (drag-and-drop for the future
tier-list maker). Rust WASM can call some of these through bindings, but
that is building on the JS ecosystem with extra friction and a smaller
talent/tooling pool — the opposite of insurance. Leptos also remains
lightly maintained (flagged in the original research).

## The proposed stack
1. DATA + METHOD — Rust, unchanged. mongil-core + mongil-build CLI:
   Serde data model, Method v1.0 math, snapshots, validation. Emits
   validated JSON + assets. This layer is framework-independent and the
   most durable part of the system.
2. SITE SHELL — Astro (static output) + TypeScript. Pages are Astro
   components rendering to plain HTML, near-zero JS by default. Deploys
   to Cloudflare Pages exactly as planned.
3. ISLANDS — React + TypeScript by default (Tom's existing stack from
   WuWa Today; the largest ecosystem for 3D/animation/drag-and-drop),
   mounted per-feature via Astro islands and fed JSON by the Rust build.
   Vanilla TypeScript allowed for tiny widgets (a GSAP decoration, a
   Live2D mount). Any island can be rewritten in a future framework
   without touching the data layer, the method, or other islands —
   Astro supports React/Vue/Svelte/Solid side by side.
4. STYLING / THEMING — Tailwind CSS + CSS custom-property design tokens
   (a DESIGN.md for this project, per Tom's UI inspo rule). Theming is a
   token swap; the Rust stack never limited this — the component/tooling
   ecosystem around it is what v2 buys.
5. ANIMATION — CSS first, GSAP for choreography (ScrollTrigger,
   Draggable, MorphSVG — all free now), Motion for React component
   animation where it fits. prefers-reduced-motion respected.
6. 3D (future, own route only) — Three.js, WebGPU renderer with automatic
   WebGL2 fallback (production-ready; Safari added WebGPU Sep 2025),
   glTF/GLB models with Draco + KTX2 compression, level-of-detail and
   on-demand rendering for mobile. Same PMX→GLB pipeline lessons as
   WhatSkin. The 3D viewer lives on its own page so its weight never
   loads on tier pages.
7. ANIMATED 2D CHARACTERS (future) — Rive for original mascot-style
   characters (interactive state machines, tiny files); Lottie for
   motion-graphic spots; Live2D via the official Cubism Web SDK only if
   proper models are ever sourced — gated by (a) Live2D's licensing
   (free at small scale, paid publishing licence above revenue
   thresholds — recheck at the time) and (b) the fan-content rules:
   no ripped game models/art, so characters must be original or
   properly licensed art. Tech supports it today; art sourcing is the
   real gate, same as WhatSkin.

## Future-proofing logic (the actual insurance)
No framework lasts forever — so durability is layered, not picked:
- Layer 1 (decades): data + method in Rust, published as JSON. Survives
  any front-end rewrite.
- Layer 2 (standards, not vendors): HTML/CSS, WebGL/WebGPU, glTF, WASM —
  W3C/Khronos standards with multi-vendor backing.
- Layer 3 (replaceable): the shell and each island. Astro itself is now
  backed by Cloudflare (acquired The Astro Technology Company, Jan 2026),
  which is also our host — aligned incentives, and static output means
  even Astro can be swapped later without touching layers 1–2.
- Tom's "newest stack always" rule applies: pinned versions + the same
  scheduled dependency-freshness checks Grace runs for WuWa Today.

## What v1 gains and loses vs the locked Rust-only front end
Gains: maker drag-and-drop becomes dnd-kit instead of hand-rolled WASM;
the tools island (toggle, evidence drawer) gets React's component
ecosystem; theming/animation ceiling rises to award-site level; 3D and
animated characters become possible at all.
Loses: the "100% Rust" purity (Rust stays where it is strongest — data,
math, validation); the build gains a Node/Astro step in CI (GitHub
Actions already planned); JS discipline is required (islands load only
where used, per-page JS budget, heavy features on their own routes).
Unchanged: static output, no server, Cloudflare Pages, Method v1.0,
Package B scope, all other locks.
