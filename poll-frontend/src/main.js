import { createApp } from 'vue'
import { createRouter, createWebHistory } from 'vue-router'
import App from './App.vue'
import AdminView from './views/AdminView.vue'
import LandingView from './views/LandingView.vue'
import PollView from './views/PollView.vue'
import './style.css'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/',         component: LandingView },
    { path: '/admin',    component: AdminView },
    { path: '/poll/:id', component: PollView },
  ],
})

createApp(App).use(router).mount('#app')
