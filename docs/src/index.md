---
layout: home

title: "AI, Reimagined"

hero:
  name: "AHQ AI"
  text: "AI, Reimagined"
  tagline: "The open sourced, Host Your Own AI Solution. Free to everyone, acessible to all"
  image:
    src: /icon.png
    alt: AHQ AI
  actions:
    - theme: brand
      text: Download
      link: /download

    - theme: alt
      text: Docs
      link: /docs

features:
  - title: Open Sourced
    details: Open Sourced under the GNU Public License 3.0. Free as in Freedom. AHQ Softwares is a non profit open sourced-focused institution.
    icon: 📃

  - title: Self Hosted
    details: Host it wherever you like, in your backyard computer or in a data center or even at the ISS. It works everywhere just as you like.
    icon: 🌏

  - title: Secure by default
    details: Configure Password Based Authentication, or Token Based Authentication, or No Authentication. HTTPS Supported.
    icon: 🔐

  - title: Multi Model Support
    details: Each server can work with multiple models. You can use any model available from Ollama directly.
    icon: 🤖

  - title: Image Support
    details: Want to talk with image content? We've got you covered
    icon: 🖼️

  - title: Ollama Backend
    details: We use the strongest and the most flexible AI backend available, it is Ollama, the docker of AI
    icon: 🦙

  - title: Resource Efficient
    details: Designed to minimize hardware strain. Run high-performance inference thanks to our rust backend.
    icon: ⚡️

  - title: Privacy Driven
    details: Your data, your device. The client stores all chat history — never the server (when using the official build). Share history temporarily only when you choose to connect.
    icon: 📍
---

<script setup>
  import { VPTeamMembers } from 'vitepress/theme';

  const members = [
    {
      avatar: 'https://avatars.githubusercontent.com/u/84524025?v=4',
      name: 'Akshanabha Chakraborty',
      title: 'Creator',
      links: [
        { icon: 'github', link: 'https://github.com/ahqsoftwares' },
        { icon: 'x', link: 'https://x.com/ahqsoftwares' }
      ]
    },
    {
      avatar: 'https://avatars.githubusercontent.com/u/92421141?v=4',
      name: 'Rohan Murudkar',
      title: 'Co-Creator',
      links: [
        { icon: 'github', link: 'https://github.com/YourJailDev' },
      ]
    },
    {
      avatar: 'https://avatars.githubusercontent.com/u/72635727?v=4',
      name: 'Robinson Arysseril',
      title: 'Lead Developer',
      links: [
        { icon: 'github', link: 'https://github.com/death7654' },
      ]
    }
  ]

</script>

<div style="margin-top: 30px;" />

# Our Team

<VPTeamMembers size="small" :members />
