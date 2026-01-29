import { createApp } from 'vue'
import './style.css'
import App from './App.vue'
import { createRouter, createWebHistory } from 'vue-router'
import TierPanel from './modules/tierlist/components/TierPanel.vue'
import Home from './modules/home/components/Home.vue'

const router = createRouter({
    history: createWebHistory(),
    routes: [
        { path: '/', component: Home},
        { path: '/board/:board', component: TierPanel},
    ]
})

createApp(App).use(router)
    .mount('#app')
