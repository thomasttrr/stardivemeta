// Serves the emitted tierlist.json verbatim at /data/tierlist.json
// (plan D-8: the public data download is the same file the site is
// built from — imported raw so the bytes are identical to emit's).
import raw from '../../generated/tierlist.json?raw';

export const GET = () =>
  new Response(raw, {
    headers: { 'content-type': 'application/json; charset=utf-8' },
  });
