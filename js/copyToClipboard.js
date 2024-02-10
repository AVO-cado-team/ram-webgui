export function copyToClipboard(text, callback) {
  window.navigator.clipboard.writeText(text)
    .then(() => callback(true))
    .catch(() => callback(false))
}
