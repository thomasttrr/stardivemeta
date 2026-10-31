// Evidence dialog wiring (plan T3.3, D-1/D-2).
// Chips are plain links to /c/<slug>/; with JS they open the matching
// pre-rendered <dialog> instead. Esc and backdrop click are native /
// handled here; closing returns focus to the chip (native behaviour
// for showModal dialogs).
import './glass-close.ts';

document
  .querySelectorAll<HTMLAnchorElement>('a[data-dialog]')
  .forEach((a) => {
    a.addEventListener('click', (e) => {
      const id = a.getAttribute('data-dialog');
      if (!id) return;
      const d = document.getElementById(id);
      if (d instanceof HTMLDialogElement && typeof d.showModal === 'function') {
        e.preventDefault();
        d.showModal();
      }
    });
  });

document.querySelectorAll<HTMLDialogElement>('dialog.ev-dialog').forEach((d) => {
  d.addEventListener('click', (e) => {
    if (e.target === d) d.close();
  });
  d.querySelectorAll<HTMLElement>('[data-close]').forEach((btn) => {
    btn.addEventListener('click', () => d.close());
  });
});
