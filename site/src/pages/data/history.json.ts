// Serves the emitted history.json verbatim at /data/history.json
// (the snapshot index; imported raw so the bytes match emit's).
import raw from '../../generated/history.json?raw';

export const GET = () =>
  new Response(raw, {
    headers: { 'content-type': 'application/json; charset=utf-8' },
  });
