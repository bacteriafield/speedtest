import { h } from 'vue'
import type { Theme } from 'vitepress'
import HomeLayout from './HomeLayout.vue'
import './style.css'

export default {
  Layout: HomeLayout,
  enhanceApp({ app, router, siteData }) {}
} satisfies Theme