import { createApp } from 'vue'
import { createRouter, createWebHistory } from 'vue-router'
import App from './App.vue'
import CreatePoll from './views/CreatePoll.vue'
import PollView from './views/PollView.vue'
import './style.css'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/',         component: CreatePoll },
    { path: '/poll/:id', component: PollView },
  ],
})

createApp(App).use(router).mount('#app')
