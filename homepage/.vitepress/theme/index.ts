import { h } from 'vue'
import type { Theme } from 'vitepress'
import HomeLayout from './Layout.vue'
import './style.css'

export default {
  Layout: HomeLayout,
  enhanceApp({ app, router, siteData }) {}
} satisfies Theme