import { createApp } from 'vue'
import './style.css'
import App from './App.vue'
import { createRouter, createWebHistory } from 'vue-router'
import TierPanel from './modules/tierlist/components/TierPanel.vue'
import Home from './modules/home/components/Home.vue'
import Tracker from './modules/tracker/components/Tracker.vue'
import Admin from './modules/admin/components/Admin.vue'
import Trivia from './modules/admin/components/Trivia.vue'
import { useAuthStore } from './utils/store.ts'
import { createPinia } from 'pinia'

const router = createRouter({
    history: createWebHistory(),
    routes: [
        {
            path: '/',
            name: 'home',
            component: Home
        },
        {
            path: '/board/:board',
            name: 'tiers',
            component: TierPanel
        },
        {
            path: '/tracker/',
            name: 'tracker',
            component: Tracker
        },
        {
          path: '/admin',
          name: 'admin',
          component: Admin,
          meta: {
            requiresAdmin: true,
            requiresRole: "admin"
          },
          children: [
              {
                  path: 'trivia',
                  name: 'admin_trivia',
                  component: Trivia
              },
          ]
        },
    ],
})


router.beforeEach((to) => {
    const auth = useAuthStore();

    if (to.meta.requiresAdmin && !auth.isLoggedIn) {
        return "/"
    }

    if (to.meta.requiresRole && auth.user?.role !== to.meta.requiresRole) {
        return "/403" // TODO: add 403 site
    }
})

const pinia = createPinia()

createApp(App)
    .use(pinia)
    .use(router)
    .mount('#app')
