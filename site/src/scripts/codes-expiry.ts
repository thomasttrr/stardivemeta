// Codes page (plan T4.1): statuses are computed in Rust at build
// time; this ≤400-byte script only catches a code whose stated
// expiry passes while the built page is up, moving its card into
// the Expired list.
const now = Date.now();
document.querySelectorAll<HTMLElement>('[data-exp]').forEach((el) => {
  const at = Date.parse(el.dataset.exp ?? '');
  if (!Number.isNaN(at) && at < now) {
    const list = document.querySelector('[data-expired-list]');
    if (list) {
      list.append(el);
      document
        .querySelector('[data-expired-wrap]')
        ?.removeAttribute('hidden');
    }
  }
});
