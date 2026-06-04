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
      search: { provider: 'local' },
      socialLinks: [
        { icon: 'github', link: 'https://github.com/martin-ueding/project-euler-solutions' },
      ],
      footer: {
        copyright: 'Copyright &copy; 2023–2026 Martin Ueding',
      },
    },
  },
  {
    documentRootPath: 'docs',
    useTitleFromFileHeading: true,
    capitalizeEachWords: true,
    sortMenusByName: true,
    sortFolderTo: 'bottom',
    collapsed: false,
  },
)
