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
    ],

    sidebar: [
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/ahq-softwares/ahq-ai' }
    ]
  }
})
