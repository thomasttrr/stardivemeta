// Movers notes (plan T4.4): the files in data/notes/ are generated
// by `mongil-build note` from snapshot diffs (D-13). Only notes with
// frontmatter status `approved` (Tom approved it) or `auto` (a
// zero-change week, D-12) render — drafts never reach the site, and
// check-dist verifies that.
import { existsSync, readdirSync, readFileSync } from 'node:fs';
import { join } from 'node:path';

export interface Note {
  /** The file stem, e.g. "2026-11-10" — also the URL segment. */
  date: string;
  from: string;
  to: string;
  status: string;
  /** The note's H1 text, for index lines. */
  title: string;
  /** The markdown body (frontmatter stripped). */
  body: string;
}

function notesDir(): string {
  const candidates = [
    join(process.cwd(), '..', 'data', 'notes'),
    join(process.cwd(), 'data', 'notes'),
  ];
  return candidates.find((d) => existsSync(d)) ?? candidates[0];
}

function parseNote(date: string, text: string): Note | null {
  const match = text.match(/^---\n([\s\S]*?)\n---\n?/);
  if (!match) return null;
  const frontmatter = match[1];
  const field = (name: string) =>
    frontmatter.match(new RegExp(`^${name}:\\s*(.+)$`, 'm'))?.[1]?.trim() ??
    '';
  const body = text.slice(match[0].length).replace(/^\n+/, '');
  const title =
    body.match(/^# (.+)$/m)?.[1]?.trim() ?? `What moved: ${date}`;
  return {
    date,
    from: field('from'),
    to: field('to'),
    status: field('status'),
    title,
    body,
  };
}

/** All renderable notes (approved or auto), newest first. */
export function loadNotes(): Note[] {
  const dir = notesDir();
  if (!existsSync(dir)) return [];
  const notes: Note[] = [];
  for (const file of readdirSync(dir)) {
    if (!file.endsWith('.md')) continue;
    const note = parseNote(
      file.slice(0, -3),
      // CRLF checkouts (Windows, core.autocrlf) must parse like LF.
      readFileSync(join(dir, file), 'utf8').replace(/\r\n/g, '\n'),
    );
    if (note && (note.status === 'approved' || note.status === 'auto')) {
      notes.push(note);
    }
  }
  return notes.sort((a, b) => b.date.localeCompare(a.date));
}

function escapeHtml(s: string): string {
  return s
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;');
}

function inline(s: string): string {
  return escapeHtml(s).replace(
    /\[([^\]]+)\]\((https?:[^)\s]+)\)/g,
    '<a href="$2">$1</a>',
  );
}

/** Render exactly the markdown subset the note generator emits:
 * `#`/`##` headings, `- ` bullets, paragraphs, and [text](url)
 * links. Anything else renders as plain paragraph text. */
export function renderNoteBody(body: string): string {
  const out: string[] = [];
  let inList = false;
  const closeList = () => {
    if (inList) {
      out.push('</ul>');
      inList = false;
    }
  };
  for (const raw of body.split('\n')) {
    const line = raw.trimEnd();
    if (line.startsWith('## ')) {
      closeList();
      out.push(`<h2>${inline(line.slice(3))}</h2>`);
    } else if (line.startsWith('# ')) {
      closeList();
      out.push(`<h1 class="page-title">${inline(line.slice(2))}</h1>`);
    } else if (line.startsWith('- ')) {
      if (!inList) {
        out.push('<ul class="moves">');
        inList = true;
      }
      out.push(`<li>${inline(line.slice(2))}</li>`);
    } else if (line.trim() === '') {
      closeList();
    } else {
      closeList();
      out.push(`<p>${inline(line)}</p>`);
    }
  }
  closeList();
  return out.join('\n');
}
