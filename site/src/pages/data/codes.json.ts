// Serves the emitted codes.json verbatim at /data/codes.json
// (plan D-8; imported raw so the bytes are identical to emit's).
import raw from '../../generated/codes.json?raw';

export const GET = () =>
  new Response(raw, {
    headers: { 'content-type': 'application/json; charset=utf-8' },
  });
