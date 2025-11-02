---
# layout: home
title: Download
editLink: false
sidebar: false
# hideExcerpt: true
---

<script setup>
  import Select from '../components/Select.vue'
  import { ref } from "vue";

  const releases = ref(undefined);

  const os = ref(undefined);

  const parseServerAssets = (assets) => {
    return {
      winX64: assets.find((d) => d.name=="ahqai-server-x86_64-pc-windows-msvc.exe")?.browser_download_url,
      winArm: assets.find((d) => d.name=="ahqai-server-aarch64-pc-windows-msvc.exe")?.browser_download_url,
      linuxX64: assets.find((d) => d.name=="ahqai-server-x86_64-unknown-linux-gnu")?.browser_download_url,
      linuxArm: assets.find((d) => d.name=="ahqai-server-aarch64-unknown-linux-gnu")?.browser_download_url,
      macX64: assets.find((d) => d.name=="ahqai-server-x86_64-apple-darwin")?.browser_download_url,
      macArm: assets.find((d) => d.name=="ahqai-server-aarch64-apple-darwin")?.browser_download_url
    }
  }

  const parseClientAssets = (assets) => {
    let output = {
      debug: {
        winX64: "",
        winArm: "",
        linuxX64: {
          rpm: "",
          deb: ""
        },
        linuxArm: {
          rpm: "",
          deb: ""
        },
        macX64: "",
        macArm: "",
        androidUniv: "",
        androidX64: "",
        androidX86: "",
        androidArmv7: "",
        androidArm64: "",
        androidArmMobile: "",
      },
      release: {
        winX64: "",
        winArm: "",
        linuxX64: {
          rpm: "",
          deb: ""
        },
        linuxArm: {
          rpm: "",
          deb: ""
        },
        macX64: "",
        macArm: "",
        androidUniv: "",
        androidX64: "",
        androidX86: "",
        androidArmv7: "",
        androidArm64: "",
        androidArmMobile: "",
      }
    };

    output.debug.winX64 = assets.find((d) => /^AHQ.AI_(\d+.\d+.\d+)_x64_en-US_windows-debug.msi$/.test(d.name))?.browser_download_url;
    output.release.winX64 = assets.find((d) => /^AHQ.AI_(\d+.\d+.\d+)_x64_en-US_windows.msi$/.test(d.name))?.browser_download_url;

    output.debug.winArm = assets.find((d) => /^AHQ.AI_(\d+.\d+.\d+)_arm64_en-US_windows-debug.msi$/.test(d.name))?.browser_download_url;
    output.release.winArm = assets.find((d) => /^AHQ.AI_(\d+.\d+.\d+)_arm64_en-US_windows.msi$/.test(d.name))?.browser_download_url;

    // LINUX
    output.release.linuxX64.deb = assets.find((d) => /^AHQ.AI_(\d+.\d+.\d+)_amd64_linux.deb$/.test(d.name))?.browser_download_url;
    output.debug.linuxX64.deb = assets.find((d) => /^AHQ.AI_(\d+.\d+.\d+)_amd64_linux-debug.deb$/.test(d.name))?.browser_download_url;

    output.release.linuxX64.rpm = assets.find((d) => /^AHQ.AI-(\d+.\d+.\d+-\d+).x86_64_linux.rpm$/.test(d.name))?.browser_download_url;
    output.debug.linuxX64.rpm = assets.find((d) => /^AHQ.AI-(\d+.\d+.\d+-\d+).x86_64_linux-debug.rpm$/.test(d.name))?.browser_download_url;

    // LINUX ARM64
    output.release.linuxArm.deb = assets.find((d) => /^AHQ.AI_(\d+.\d+.\d+)_arm64_linux.deb$/.test(d.name))?.browser_download_url;
    output.debug.linuxArm.deb = assets.find((d) => /^AHQ.AI_(\d+.\d+.\d+)_arm64_linux-debug.deb$/.test(d.name))?.browser_download_url;

    output.release.linuxArm.rpm = assets.find((d) => /^AHQ.AI-(\d+.\d+.\d+-\d+).aarch64_linux.rpm$/.test(d.name))?.browser_download_url;
    output.debug.linuxArm.rpm = assets.find((d) => /^AHQ.AI-(\d+.\d+.\d+-\d+).aarch64_linux-debug.rpm$/.test(d.name))?.browser_download_url;

    // MACOS x64
    output.release.macX64 = assets.find((d) => /^AHQ.AI_(\d+.\d+.\d+)_x64_darwin.dmg$/.test(d.name))?.browser_download_url;
    output.debug.macX64 = assets.find((d) => /^AHQ.AI_(\d+.\d+.\d+)_x64_darwin-debug.dmg$/.test(d.name))?.browser_download_url;

    // MACOS ARM64
    output.release.macArm = assets.find((d) => /^AHQ.AI_(\d+.\d+.\d+)_aarch64_darwin.dmg$/.test(d.name))?.browser_download_url;
    output.debug.macArm = assets.find((d) => /^AHQ.AI_(\d+.\d+.\d+)_aarch64_darwin-debug.dmg$/.test(d.name))?.browser_download_url;

    // ANDROID UNIV
    output.release.androidUniv = assets.find((d) => d.name =="app-universal-release.apk")?.browser_download_url;
    output.debug.androidUniv = assets.find((d) => d.name == "app-universal-debug.apk")?.browser_download_url;

    output.release.androidArmMobile = assets.find((d) => d.name =="app-arm-mobile-release.apk")?.browser_download_url;
    output.debug.androidArmMobile = assets.find((d) => d.name == "app-arm-mobile-debug.apk")?.browser_download_url;

    output.release.androidX64 = assets.find((d) => d.name =="app-x86_64-release.apk")?.browser_download_url;
    output.debug.androidX64 = assets.find((d) => d.name == "app-x86_64-debug.apk")?.browser_download_url;

    output.release.androidX86 = assets.find((d) => d.name =="app-x86-release.apk")?.browser_download_url;
    output.debug.androidX86 = assets.find((d) => d.name == "app-x86-debug.apk")?.browser_download_url;

    output.release.androidArmv7 = assets.find((d) => d.name =="app-arm-release.apk")?.browser_download_url;
    output.debug.androidArmv7 = assets.find((d) => d.name == "app-arm-debug.apk")?.browser_download_url;

    output.release.androidArm64 = assets.find((d) => d.name =="app-arm64-release.apk")?.browser_download_url;
    output.debug.androidArm64 = assets.find((d) => d.name == "app-arm64-debug.apk")?.browser_download_url;

    return output;
  };

  (async() => {
    const releaseData = await fetch("https://api.github.com/repos/ahq-softwares/ahq-ai/releases", {
      cache: "force-cache"
    })
      .then((d) => d.json());

    const latestClient = await fetch("https://api.github.com/repos/ahq-softwares/ahq-ai/releases/latest", {
      cache: "force-cache"
    })
      .then((d) => {
        if (!d.ok) {
          throw new Error("")
        }

        return d.json();
      })
      .catch(() => (undefined));

    const bleedingEdgeClient = releaseData.find((d) => d.tag_name.startsWith("v"));

    const latestServer = releaseData.find((d) => d.tag_name.startsWith("server-v") && !d.prerelease);

    const bleedingEdgeServer = releaseData.find((d) => d.tag_name.startsWith("server-v"));

    const outValue = {
      client: {
        latest: latestClient ? parseClientAssets(latestClient.assets) : undefined,
        bleeding: bleedingEdgeClient ? parseClientAssets(bleedingEdgeClient.assets) : undefined
      },
      server: {
        latest: latestServer ? parseServerAssets(latestServer.assets) : undefined,
        bleeding: bleedingEdgeServer ? parseServerAssets(bleedingEdgeServer.assets) : undefined
      }
    }

    releases.value = outValue;

    console.log(outValue);
  })()

  const channel = ref("latest");
  const appTypeRef = ref("release");
  const entry = ref();
  const bundle = ref();

  const x64 = ["x86", "x64", "x86_64", "Win64"];
  const arm64 = ["arm64", "aarch64", "arm"];

  async function windowsAutoFill() {
    let arch = "";

    try {
      arch = (await navigator?.userAgentData?.getHighEntropyValues(["architecture"])).architecture;

    } catch (e) {
      arch = navigator.userAgent;

      console.warn(e);
      console.log("Using fallback method");
    }

    if (arm64.some((d) => arch.toLowerCase().includes(d))) {
      entry.value = "winArm"
    } else if (x64.some((d) => arch.toLowerCase().includes(d))) {
      entry.value = "winX64"
    } else {
      entry.value = "winX64"
    }
  }

  async function macAutoFill() {
    let arch = "";

    try {
      arch = (await navigator?.userAgentData?.getHighEntropyValues(["architecture"])).architecture;

    } catch (e) {
      arch = navigator.userAgent;

      console.warn(e);
      console.log("Using fallback method");
    }

    if (arm64.some((d) => arch.toLowerCase().includes(d))) {
      entry.value = "macArm"
    } else if (x64.some((d) => arch.toLowerCase().includes(d))) {
      entry.value = "macX64"
    } else {
      entry.value = "macX64"
    }
  }

  async function androidAutoFill() {
    entry.value = "androidArmMobile"
  }

  const channelOpt = [
    { text: 'Latest', value: 'latest' },
    { text: 'Bleeding', value: 'bleeding' },
  ]

  const appType = [
    { text: 'Normal release', value: 'release' },
    { text: 'Debug release', value: 'debug' },
  ];

  const winArchOptions = [
    { text: 'X64', value: 'winX64' },
    { text: 'Arm64', value: 'winArm' },
  ];

  const macArchOptions = [
    { text: 'X64', value: 'macX64' },
    { text: 'Arm64', value: 'macArm' },
  ];

  const androidArchOptions = [
    { text: 'Default', value: 'androidUniv' },
    { text: 'Mobile', value: 'androidArmMobile' },
    { text: 'Arm64', value: 'androidArm64' },
    { text: 'Armv7', value: 'androidArmv7' },
    { text: 'Intel X64', value: 'androidX64' },
    { text: 'Intel X86', value: 'androidX86' },
  ];

  const linuxArchOptions = [
    { text: 'X64', value: 'linuxX64' },
    { text: 'Arm64', value: 'linuxArm' },
  ];

  const linuxBundleOptions = [
    { text: '.deb', value: 'deb' },
    { text: '.rpm', value: 'rpm' },
  ];
</script>

<div style="margin-top:3rem;" />

# Download

AHQ AI has a decentralized client server architecture.

## Client

<div v-if="!releases" class="loader" style="margin-bottom:10px;margin-left: auto;margin-right:auto;"></div> 

<div v-if="releases">

:::tabs key:os
== Windows
<span>Fill the parameters and download button will be shown, if build is available</span>
<button @click="windowsAutoFill()" class="dontknow">Autofill Dropdowns</button>
<div class="responsive-grid" style="width:100%;gap:10px;margin-bottom:30px;">
  <Select v-model="channel" :options="channelOpt" placeholder="Select Channel" />

  <Select v-model="appTypeRef" :options="appType" placeholder="Select Release" />

  <Select v-model="entry" :options="winArchOptions" placeholder="Select Architecture" />
</div>
<div style="width:100%;display:flex;flex-direction:column;">
  <span 
  style="display:block;margin-left: auto;margin-right:auto;" 
  v-if="channel && appTypeRef && entry && !releases.client?.[channel]?.[appTypeRef]?.[entry]">Unavailable</span>
  <a 
  style="display:block;margin-left: auto;margin-right:auto;" 
  v-if="releases.client?.[channel]?.[appTypeRef]?.[entry]!=undefined"
  :href="releases.client?.[channel]?.[appTypeRef]?.[entry]">Download</a>
</div>
== MacOS
<span>Fill the parameters and download button will be shown, if build is available</span>
<button @click="macAutoFill()" class="dontknow">Autofill Dropdowns</button>
<div class="responsive-grid" style="width:100%;gap:10px;margin-bottom:30px;">
  <Select v-model="channel" :options="channelOpt" placeholder="Select Channel" />

  <Select v-model="appTypeRef" :options="appType" placeholder="Select Release" />

  <Select v-model="entry" :options="macArchOptions" placeholder="Select Architecture" />
</div>
<div style="width:100%;display:flex;flex-direction:column;">
  <span 
  style="display:block;margin-left: auto;margin-right:auto;" 
  v-if="channel && appTypeRef && entry && !releases.client?.[channel]?.[appTypeRef]?.[entry]">Unavailable</span>
  <a 
  style="display:block;margin-left: auto;margin-right:auto;" 
  v-if="releases.client?.[channel]?.[appTypeRef]?.[entry]!=undefined"
  :href="releases.client?.[channel]?.[appTypeRef]?.[entry]">Download</a>
</div>
== Linux
<span>Fill the parameters and download button will be shown, if build is available</span>
<div class="responsive-grid" style="width:100%;gap:10px;margin-bottom:30px;">
  <Select v-model="channel" :options="channelOpt" placeholder="Select Channel" />

  <Select v-model="appTypeRef" :options="appType" placeholder="Select Release" />

  <Select v-model="entry" :options="linuxArchOptions" placeholder="Select Arch" />

  <Select v-model="bundle" :options="linuxBundleOptions" placeholder="Select Bundle" />
</div>
<div style="width:100%;display:flex;flex-direction:column;">
  <span 
  style="display:block;margin-left: auto;margin-right:auto;" 
  v-if="channel && appTypeRef && entry && !releases.client?.[channel]?.[appTypeRef]?.[entry]">Unavailable</span>
  <a 
  style="display:block;margin-left: auto;margin-right:auto;" 
  v-if="releases.client?.[channel]?.[appTypeRef]?.[entry]!=undefined"
  :href="releases.client?.[channel]?.[appTypeRef]?.[entry]">Download</a>
</div>
== Android
<span>Fill the parameters and download button will be shown, if build is available</span>
<button @click="androidAutoFill()" class="dontknow">Autofill Dropdowns</button>
<div class="responsive-grid" style="width:100%;gap:10px;margin-bottom:30px;">
  <Select v-model="channel" :options="channelOpt" placeholder="Select Channel" />

  <Select v-model="appTypeRef" :options="appType" placeholder="Select Release" />

  <Select v-model="entry" :options="androidArchOptions" placeholder="Select Arch" />
</div>
<div style="width:100%;display:flex;flex-direction:column;">
<span 
  style="display:block;margin-left: auto;margin-right:auto;" 
  v-if="channel && appTypeRef && entry && !releases.client?.[channel]?.[appTypeRef]?.[entry]">Unavailable</span>
  <a 
  style="display:block;margin-left: auto;margin-right:auto;" 
  v-if="releases.client?.[channel]?.[appTypeRef]?.[entry]!=undefined"
  :href="releases.client?.[channel]?.[appTypeRef]?.[entry]">Download</a></div>
== IOS
Please read the guide [here](/docs/iossetup)
:::

</div>

::: details Supported Client OS

| OS      | Architecture              | Supported | Notes                                |
| ------- | :------------------------ | :-------: | :----------------------------------- |
| Windows | x64                       |    ✅     | Windows 10+ (Windows 11 Recommended) |
|         | arm64                     |    ✅     |                                      |
| macOS   | x64                       |    ✅     | Sideloading required                 |
|         | arm64                     |    ✅     | Sideloading required                 |
| Linux   | x64                       |    ✅     | Requires Ubuntu 22.04 or later       |
|         | arm64                     |    ✅     |                                      |
| Android | arm64                     |    ✅     |                                      |
|         | armv7                     |    ✅     |                                      |
|         | armv7, arm64              |    ✅     | Combined APK                         |
|         | x86                       |    ✅     |                                      |
|         | x86_64                    |    ✅     |                                      |
|         | x86, x86_64, armv7, arm64 |    ✅     | Combined APK                         |
| iOS     | -                         |    🟨     | Build from Scratch.                  |
|         |                           |           | Occasionally we may provide binaries |

:::

## Server

<div v-if="!releases" class="loader" style="margin-bottom:10px;margin-left: auto;margin-right:auto;"></div> 

<div v-if="releases">

:::tabs key:os
== Windows
<span>Fill the parameters and download button will be shown, if build is available</span>
<div class="responsive-grid" style="width:100%;gap:10px;margin-bottom:30px;">
  <Select v-model="channel" :options="channelOpt" placeholder="Select Channel" />

  <Select v-model="entry" :options="winArchOptions" placeholder="Select Arch" />
</div>
<div style="width:100%;display:flex;flex-direction:column;">
  <a 
  style="display:block;margin-left: auto;margin-right:auto;" 
  v-if="releases.server?.[channel]?.[entry]"
  :href="releases.server?.[channel]?.[entry]">Download</a>
</div>
== MacOS
<span>Fill the parameters and download button will be shown, if build is available</span>
<div class="responsive-grid" style="width:100%;gap:10px;margin-bottom:30px;">
  <Select v-model="channel" :options="channelOpt" placeholder="Select Channel" />

  <Select v-model="entry" :options="macArchOptions" placeholder="Select Arch" />
</div>
<div style="width:100%;display:flex;flex-direction:column;">
  <a 
  style="display:block;margin-left: auto;margin-right:auto;" 
  v-if="releases.server?.[channel]?.[entry]"
  :href="releases.server?.[channel]?.[entry]">Download</a>
</div>
== Linux
<span>Fill the parameters and download button will be shown, if build is available</span>
<div class="responsive-grid" style="width:100%;gap:10px;margin-bottom:30px;">
  <Select v-model="channel" :options="channelOpt" placeholder="Select Channel" />

  <Select v-model="entry" :options="linuxArchOptions" placeholder="Select Arch" />
</div>
<div style="width:100%;display:flex;flex-direction:column;">
  <a 
  style="display:block;margin-left: auto;margin-right:auto;" 
  v-if="releases.server?.[channel]?.[entry]"
  :href="releases.server?.[channel]?.[entry]">Download</a>
</div>
:::

</div>

::: details Supported OS for Server

| OS      | Architecture | Supported | Notes               |
| ------- | :----------- | :-------: | :------------------ |
| Windows | x64          |    ✅     | Windows 10 or above |
|         | arm64        |    ✅     |                     |
| macOS   | x64          |    ✅     |                     |
|         | arm64        |    ✅     |                     |
| Linux   | x64          |    ✅     | Ubuntu 22.04+       |
|         | arm64        |    ✅     | Ubuntu 22.04+       |

:::
