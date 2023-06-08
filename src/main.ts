import './assets/main.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import router from './router'

import 'highlight.js/styles/github-dark.css'
import hljs from 'highlight.js/lib/core'
import diff from 'highlight.js/lib/languages/diff'
import hljsVuePlugin from '@highlightjs/vue-plugin'

hljs.registerLanguage('diff', diff)

const app = createApp(App)

app.use(createPinia())
app.use(hljsVuePlugin)
app.use(router)

app.mount('#app')
