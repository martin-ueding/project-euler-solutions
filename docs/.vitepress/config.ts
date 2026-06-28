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
        md.use(container, 'theorem', {
          render(tokens, idx) {
            const token = tokens[idx]
            if (token.nesting === 1) {
              const name = token.info.trim().slice('theorem'.length).trim()
              const title = name
                ? `Theorem (${md.utils.escapeHtml(name)})`
                : 'Theorem'
              return `<div class="theorem"><p class="theorem-title">${title}</p>\n`
            }
            return '</div>\n'
          },
        })
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
