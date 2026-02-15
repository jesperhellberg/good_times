import { createApp, watch } from 'vue'
import { createRouter, createWebHistory } from 'vue-router'
import { createI18n } from 'vue-i18n'
import App from './App.vue'
import AdminView from './views/AdminView.vue'
import LandingView from './views/LandingView.vue'
import PollView from './views/PollView.vue'
import { messages, resolveLocale } from './i18n'
import './style.css'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/',         component: LandingView },
    { path: '/admin',    component: AdminView },
    { path: '/poll/:id', component: PollView },
  ],
})

const i18n = createI18n({
  legacy: false,
  globalInjection: true,
  locale: resolveLocale(),
  messages,
})

document.documentElement.lang = i18n.global.locale.value
watch(i18n.global.locale, (newLocale) => {
  localStorage.setItem('locale', newLocale)
  document.documentElement.lang = newLocale
})

createApp(App).use(router).use(i18n).mount('#app')
