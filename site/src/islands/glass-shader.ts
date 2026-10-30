// GLSL for the liquid-glass field (GlassScene.tsx).
//
// One full-viewport pass draws the deep-sea colour field AND every glass
// panel on the page: the island feeds the shader the live screen rects of
// all [data-glass] elements, and for pixels inside one the field is
// re-sampled through a lens — strong refraction at the rim, a slight
// magnification in the body, chromatic split near the edge, a frosted
// (low-detail) read of the field, a dark tint that keeps text legible,
// and a specular rim lit from the top-left and from the pointer.
// Purely decorative: no page data reaches this shader.

export const MAX_RECTS = 24;

export const vertexShader = /* glsl */ `
void main() {
  gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
}
`;

export const fragmentShader = /* glsl */ `
precision highp float;

#define MAX_RECTS ${MAX_RECTS}

uniform float uTime;
uniform vec2 uRes;      // drawing-buffer size, device px
uniform float uDpr;
uniform float uScroll;  // CSS px
uniform vec2 uPointer;  // CSS px, viewport space
uniform int uCount;
uniform vec4 uRects[MAX_RECTS];  // x, y, w, h — CSS px, viewport space
uniform float uRadii[MAX_RECTS];

const vec3 INK    = vec3(0.020, 0.047, 0.059);
const vec3 ABYSS  = vec3(0.027, 0.170, 0.170);
const vec3 LAGOON = vec3(0.060, 0.360, 0.330);
const vec3 AMBER  = vec3(0.780, 0.500, 0.150);
const vec3 CORAL  = vec3(0.690, 0.270, 0.210);
const vec3 TINT   = vec3(0.030, 0.058, 0.066);

float hash(vec2 p) {
  p = fract(p * vec2(123.34, 456.21));
  p += dot(p, p + 45.32);
  return fract(p.x * p.y);
}

float noise(vec2 p) {
  vec2 i = floor(p);
  vec2 f = fract(p);
  vec2 u = f * f * (3.0 - 2.0 * f);
  float a = hash(i);
  float b = hash(i + vec2(1.0, 0.0));
  float c = hash(i + vec2(0.0, 1.0));
  float d = hash(i + vec2(1.0, 1.0));
  return mix(mix(a, b, u.x), mix(c, d, u.x), u.y);
}

// detail: 1.0 = full, 0.0 = frosted (fewer, weaker octaves)
float fbm(vec2 p, float detail) {
  float v = 0.0;
  float a = 0.5;
  mat2 rot = mat2(0.8, 0.6, -0.6, 0.8);
  for (int i = 0; i < 5; i++) {
    float w = i < 2 ? 1.0 : detail;
    v += a * noise(p) * w;
    p = rot * p * 2.03 + 11.7;
    a *= 0.5;
  }
  return v;
}

// Tileable water caustics (after Dave Hoskins' / joltz0r's classic).
float caustic(vec2 uv, float time) {
  vec2 p = mod(uv * 6.28318, 6.28318) - 250.0;
  vec2 i = p;
  float c = 1.0;
  float inten = 0.005;
  for (int n = 0; n < 4; n++) {
    float t = time * (1.0 - (3.5 / float(n + 1)));
    i = p + vec2(cos(t - i.x) + sin(t + i.y), sin(t - i.y) + cos(t + i.x));
    c += 1.0 / length(vec2(p.x / (sin(i.x + t) / inten), p.y / (cos(i.y + t) / inten)));
  }
  c /= 4.0;
  c = 1.17 - pow(c, 1.4);
  return pow(abs(c), 8.0);
}

// The deep-sea field at CSS-px position p. frost in [0,1].
vec3 field(vec2 p, float frost) {
  float detail = 1.0 - frost;
  float minDim = min(uRes.x, uRes.y) / uDpr;
  vec2 sp = p + vec2(0.0, uScroll * 0.18);          // gentle parallax
  vec2 uv = sp / max(minDim, 1.0);
  vec2 sv = p / (uRes / uDpr);                        // 0..1 screen space
  float t = uTime * 0.035;

  vec2 q = vec2(fbm(uv * 1.35 + vec2(0.0, t), detail),
                fbm(uv * 1.35 + vec2(5.2, -t), detail));
  float f = fbm(uv * 1.15 + 2.1 * q + vec2(t * 0.7, t * 0.3), detail);

  vec3 col = INK;
  col = mix(col, ABYSS, smoothstep(0.2, 0.8, f));
  col = mix(col, LAGOON, smoothstep(0.55, 1.0, f) * 0.7);

  // warm light falling from the top-left, coral glow bottom-right
  float warm = exp(-length((sv - vec2(0.1, -0.02)) * vec2(1.0, 1.5)) * 2.3);
  col += AMBER * warm * (0.38 + 0.3 * q.x);
  float coral = exp(-length((sv - vec2(0.9, 0.85)) * vec2(1.2, 1.0)) * 2.8);
  col += CORAL * coral * (0.24 + 0.26 * q.y);

  // caustic light network — the detail the glass visibly bends
  vec2 cuv = (sp + 40.0 * q) / 520.0;
  float cs = caustic(cuv, uTime * 0.35 + 23.0);
  float cmask = 0.25 + 1.1 * warm + 0.5 * smoothstep(0.45, 0.95, f);
  col += mix(vec3(0.55, 0.95, 0.82), vec3(1.0, 0.82, 0.5), warm) * cs * cmask * (0.10 + 0.12 * detail);

  // drifting "stars" — plankton sparks rising slowly
  vec2 sg = (sp + vec2(0.0, uTime * 7.0)) / 30.0;
  vec2 cell = floor(sg);
  float h = hash(cell);
  if (h > 0.9) {
    vec2 c = vec2(hash(cell + 3.1), hash(cell + 7.7));
    float d = length(fract(sg) - c) * 30.0;
    float tw = 0.55 + 0.45 * sin(uTime * (0.8 + h * 2.0) + h * 40.0);
    float r = 0.9 + frost * 3.5;
    col += vec3(1.0, 0.94, 0.82) * smoothstep(r, 0.0, d) * tw * (0.55 - 0.35 * frost) * (0.35 + 0.65 * (1.0 - sv.y));
  }

  // vignette
  col *= 1.0 - 0.38 * smoothstep(0.35, 1.15, length((sv - 0.5) * vec2(1.25, 1.0)) * 1.4);
  return col;
}

float sdRoundBox(vec2 p, vec2 b, float r) {
  vec2 q = abs(p) - b + r;
  return length(max(q, 0.0)) + min(max(q.x, q.y), 0.0) - r;
}

float rectSd(int i, vec2 p) {
  vec4 r = uRects[i];
  vec2 hb = r.zw * 0.5;
  float rad = min(uRadii[i], min(hb.x, hb.y));
  return sdRoundBox(p - (r.xy + hb), hb, rad);
}

void main() {
  vec2 p = vec2(gl_FragCoord.x, uRes.y - gl_FragCoord.y) / uDpr;

  int inside = -1;
  float insideArea = 1e12;
  float shadow = 0.0;
  for (int i = 0; i < MAX_RECTS; i++) {
    if (i >= uCount) break;
    vec4 r = uRects[i];
    float d = rectSd(i, p);
    float area = r.z * r.w;
    if (d < 1.0 && area < insideArea) {
      inside = i;
      insideArea = area;
    }
    // soft drop shadow, offset downward
    float ds = rectSd(i, p - vec2(0.0, 10.0));
    shadow = max(shadow, (1.0 - smoothstep(-6.0, 36.0, ds)) * step(0.0, d));
  }

  vec3 col;
  float d = inside < 0 ? 1e6 : rectSd(inside, p);
  vec3 outside = vec3(0.0);
  if (d > -1.0) {
    outside = field(p, 0.0) * (1.0 - shadow * 0.42);
  }
  if (inside < 0) {
    col = outside;
  } else {
    vec4 r = uRects[inside];
    vec2 c = r.xy + r.zw * 0.5;
    float e = 1.0;
    vec2 n = normalize(vec2(rectSd(inside, p + vec2(e, 0.0)) - rectSd(inside, p - vec2(e, 0.0)),
                            rectSd(inside, p + vec2(0.0, e)) - rectSd(inside, p - vec2(0.0, e))) + 1e-5);
    float depth = max(-d, 0.0);
    float bevelW = min(22.0, min(r.z, r.w) * 0.35);
    float bev = clamp(1.0 - depth / bevelW, 0.0, 1.0);
    float lens = bev * bev * (3.0 - 2.0 * bev);

    vec2 off = -n * lens * 26.0 + (c - p) * 0.035;
    float frost = 0.62;
    if (lens > 0.015) {
      col.r = field(p + off * 1.00, frost).r;
      col.g = field(p + off * 1.12, frost).g;
      col.b = field(p + off * 1.26, frost).b;
    } else {
      col = field(p + off, frost);
    }

    col = mix(col, TINT, 0.42) + 0.018;

    vec2 L = normalize(vec2(-0.55, -0.85));
    float facing = max(dot(n, L), 0.0);
    float away = max(dot(n, -L), 0.0);
    float rim = 1.0 - smoothstep(0.0, 1.5, depth);
    float glint = exp(-length(uPointer - p) / 220.0);

    col += rim * (0.07 + 0.42 * pow(facing, 2.0) + 0.16 * pow(away, 3.0) + 0.45 * glint);
    col += lens * (0.06 * facing - 0.035 * away + 0.05 * glint);
    // faint top sheen across the body
    col += 0.035 * smoothstep(r.y + r.w * 0.55, r.y, p.y);
    // analytic anti-aliasing across the 2px boundary band
    col = mix(outside, col, smoothstep(1.0, -1.0, d));
  }

  col += (hash(gl_FragCoord.xy + fract(uTime)) - 0.5) / 255.0;  // dither
  gl_FragColor = vec4(col, 1.0);
}
`;
