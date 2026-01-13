import { QuartzComponent, QuartzComponentConstructor, QuartzComponentProps } from "./types"

// @ts-ignore
import script from "./scripts/redirect.inline"

const Redirect: QuartzComponent = ({ fileData }: QuartzComponentProps) => {
  const redirect = fileData.frontmatter?.redirect as string | undefined

  if (!redirect) {
    return null
  }

  // Convert page name to slug
  const slug = redirect.toLowerCase().replace(/ /g, "-")

  return <meta name="redirect" content={slug} />
}

Redirect.afterDOMLoaded = script

export default (() => Redirect) satisfies QuartzComponentConstructor
