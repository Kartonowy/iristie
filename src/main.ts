import { createApp } from 'vue'
import './style.css'
import App from './App.vue'
import { createRouter, createWebHistory } from 'vue-router'
import TierPanel from './modules/tierlist/components/TierPanel.vue'
import Home from './modules/home/Home.vue'
import Tracker from './modules/tracker/components/Tracker.vue'
import Pot from './modules/pot/Pot.vue'

const router = createRouter({
    history: createWebHistory(),
    routes: [
        { path: '/', component: Home},
        { path: '/board/:board', component: TierPanel},
        { path: '/tracker/', component: Tracker},
        { path: '/pot/', component: Pot}
    ]
})

createApp(App).use(router)
    .mount('#app')
