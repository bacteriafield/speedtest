import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  head: [
    ['link', { rel: 'icon', href: 'public/area_chart_24dp_E3E3E3_FILL0_wght400_GRAD0_opsz24.svg' }],
    ['meta', { name: 'viewport', content: 'width=device-width, initial-scale=1.0' }]
  ],
  base: '/speedtest/',
  title: "Speedtest",
  description: "A morden cli speedtest tool, zero dependency, with graph integration and metrics export.",
  themeConfig: {}
})
