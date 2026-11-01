// Codes page: tap-to-copy on each Active/Unconfirmed code card.
// One delegated listener; the button label flips to "Copied" for a
// moment as the feedback. Falls back to a scratch textarea where the
// async clipboard API is unavailable (non-secure contexts).
async function copyText(text: string): Promise<boolean> {
  try {
    await navigator.clipboard.writeText(text);
    return true;
  } catch {
    const ta = document.createElement('textarea');
    ta.value = text;
    ta.style.position = 'fixed';
    ta.style.opacity = '0';
    document.body.append(ta);
    ta.select();
    let ok = false;
    try {
      ok = document.execCommand('copy');
    } catch {
      ok = false;
    }
    ta.remove();
    return ok;
  }
}

document.addEventListener('click', (e) => {
  const btn = (e.target as HTMLElement).closest<HTMLButtonElement>(
    'button[data-copy]',
  );
  if (!btn || btn.dataset.busy) return;
  const code = btn.dataset.copy ?? '';
  btn.dataset.busy = '1';
  void copyText(code).then((ok) => {
    const label = btn.querySelector('.copy-label');
    if (label) label.textContent = ok ? 'Copied' : 'Select the code';
    btn.classList.toggle('copied', ok);
    setTimeout(() => {
      if (label) label.textContent = 'Copy';
      btn.classList.remove('copied');
      delete btn.dataset.busy;
    }, 1600);
  });
});
