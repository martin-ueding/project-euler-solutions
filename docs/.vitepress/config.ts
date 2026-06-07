import { withSidebar } from 'vitepress-sidebar'

export default withSidebar(
  {
    title: "Martin's Project Euler Solutions",
    description: 'My Project Euler solutions.',
    base: '/project-euler-solutions/',
    markdown: {
      math: true,
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
