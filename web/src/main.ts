import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { createRouter, createWebHistory } from 'vue-router'
import App from './App.vue'
import './style.css'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', component: () => import('./views/ChatView.vue') },
    { path: '/sessions', component: () => import('./views/SessionsView.vue') },
    { path: '/files', component: () => import('./views/FilesView.vue') },
    { path: '/tools', component: () => import('./views/ToolsView.vue') },
    { path: '/settings', component: () => import('./views/SettingsView.vue') },
    { path: '/cost', component: () => import('./views/CostView.vue') },
    { path: '/mcp', component: () => import('./views/MCPView.vue') },
  ],
})

const app = createApp(App)
app.use(createPinia())
app.use(router)
app.mount('#app')
