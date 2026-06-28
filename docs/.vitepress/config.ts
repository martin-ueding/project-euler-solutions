import { withSidebar } from 'vitepress-sidebar'
import container from 'markdown-it-container'

export default withSidebar(
  {
    title: "Martin's Project Euler Solutions",
    description: 'My Project Euler solutions.',
    base: '/project-euler-solutions/',
    markdown: {
      math: true,
      config: (md) => {
        const callout = (name: string, label: string) => {
          md.use(container, name, {
            render(tokens, idx) {
              const token = tokens[idx]
              if (token.nesting === 1) {
                const arg = token.info.trim().slice(name.length).trim()
                const title = arg
                  ? `${label} (${md.utils.escapeHtml(arg)})`
                  : label
                return `<div class="${name}"><p class="callout-title">${title}</p>\n`
              }
              return '</div>\n'
            },
          })
        }
        callout('theorem', 'Theorem')
        callout('definition', 'Definition')
      },
    },
    themeConfig: {
      nav: [
        { text: "Source Code", link: "https://github.com/martin-ueding/project-euler-solutions" },
        { text: "Older Solutions", link: "https://martin-ueding.de/pages/project-euler/" },
        { text: "Martin Ueding", link: "https://martin-ueding.de/" },
      ],
      search: { provider: 'local' },
      socialLinks: [
        { icon: 'github', link: 'https://github.com/martin-ueding/project-euler-solutions' },
      ],
      footer: {
        copyright: 'Copyright &copy; 2023–2026 Martin Ueding',
      },
      outline: {
        level: [2, 3],
      },
    },
  },
  {
    documentRootPath: 'docs',
    useTitleFromFileHeading: true,
    capitalizeEachWords: false,
    sortMenusByName: false,
    sortFolderTo: 'bottom',
    collapsed: false,
  },
)
