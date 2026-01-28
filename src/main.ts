import { createApp } from 'vue'
import './style.css'
import App from './App.vue'
import { createRouter, createWebHistory } from 'vue-router'
import TierPanel from './components/TierPanel.vue'

const router = createRouter({
    history: createWebHistory(),
    routes: [{ path: '/:board', component: TierPanel}]
})

createApp(App).use(router)
    .mount('#app')
