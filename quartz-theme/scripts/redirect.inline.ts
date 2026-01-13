// Handle redirect pages (e.g., favorites that redirect to actual pages)
function handleRedirect() {
  const redirectMeta = document.querySelector('meta[name="redirect"]')
  if (redirectMeta) {
    const target = redirectMeta.getAttribute("content")
    if (target) {
      // Find the link to the target page and navigate
      const slug = target.toLowerCase().replace(/ /g, "-").replace(/\//g, "/")

      // Try to find the actual page link or construct the URL
      const baseUrl = window.location.origin
      const pathPrefix = document.querySelector<HTMLElement>('[data-path-prefix]')?.dataset.pathPrefix || ''

      // Navigate to the target page
      window.location.replace(`${baseUrl}${pathPrefix}/${slug}`)
    }
  }
}

document.addEventListener("nav", handleRedirect)
