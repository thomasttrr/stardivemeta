// Codes page (plan T4.1): statuses are computed in Rust at build
// time; this small script only catches a code whose stated expiry
// passes while the built page is up. It moves the card into the
// Expired list and turns it into an expired card: expired styling, no
// Copy button or active-only lines, and "Expired <date>" in place of
// "Expires <date>".
const now = Date.now();
document.querySelectorAll<HTMLElement>('[data-exp]').forEach((el) => {
  const at = Date.parse(el.dataset.exp ?? '');
  if (!Number.isNaN(at) && at < now) {
    const list = document.querySelector('[data-expired-list]');
    if (list) {
      el.classList.replace('code-active', 'code-expired');
      el.querySelectorAll('[data-active-only]').forEach((x) => x.remove());
      const line = el.querySelector<HTMLElement>('[data-exp-line]');
      if (line) {
        line.textContent = (line.textContent ?? '').trim().replace(/^Expires/, 'Expired');
        line.classList.add('dim');
      }
      list.append(el);
      document
        .querySelector('[data-expired-wrap]')
        ?.removeAttribute('hidden');
    }
  }
});
