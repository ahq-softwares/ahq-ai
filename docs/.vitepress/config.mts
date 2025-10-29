import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  srcDir: "src",

  title: "AHQ AI",
  description: "AI, Reimagined",
  base: "/ahq-ai/",
  head: [
    ['link', { rel: 'icon', href: '/ahq-ai/icon.png', type: 'image/png' }],
  ],
  themeConfig: {
    logo: "/icon.png",
    lastUpdated: {
      text: "Last Updated "
    },
    search: {
      provider: "local"
    },
    editLink: {
      pattern: 'https://github.com/ahq-softwares/ahq-ai/edit/main/docs/src/:path'
    },
    footer: {
      copyright: "©AHQ Softwares",
      message: "Licensed under GNU Public License 3.0"
    },
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Download', link: '/download.md' },
      { text: 'Docs', link: '/docs.md' },
    ],

    sidebar: [
      {
        text: "AHQ AI",
        collapsed: false,
        items: [
          {
            text: 'Download',
            collapsed: false,
            items: [
              { text: "Get AHQ AI", link: "/download.md" }
            ]
          },
          {
            text: 'Docs',
            collapsed: false,
            items: [
              { text: "Introduction", link: "/docs.md" },
              { text: "Server Setup", link: "/serversetup.md" },
              { text: "Client Setup", link: "/clientsetup.md" },
            ]
          }
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/ahq-softwares/ahq-ai' }
    ]
  }
})
