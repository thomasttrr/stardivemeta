// Investment-level toggle wiring (plan T3.2, D-2).
// The boards themselves are static HTML; this only flips
// <html data-level> and remembers the choice in localStorage.
const KEY = 'sdm.level';
const LEVELS = ['low', 'high', 'max'];

function currentLevel(): string {
  return document.documentElement.dataset.level || 'low';
}

function syncRadios(): void {
  const cur = currentLevel();
  document
    .querySelectorAll<HTMLInputElement>('input[name="sdm-level"]')
    .forEach((r) => {
      r.checked = r.value === cur;
    });
}

document
  .querySelectorAll<HTMLInputElement>('input[name="sdm-level"]')
  .forEach((r) => {
    r.addEventListener('change', () => {
      if (!LEVELS.includes(r.value)) return;
      document.documentElement.dataset.level = r.value;
      try {
        localStorage.setItem(KEY, r.value);
      } catch {
        /* private mode etc. — the attribute alone still works */
      }
    });
  });

syncRadios();
