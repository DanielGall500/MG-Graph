import { createRouter, createWebHistory } from 'vue-router'
const getHomeViewPage = () => import('@/views/HomeView.vue')
const getAboutViewPage = () => import('@/views/AboutView.vue')

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    { path: '/',
      name: 'home',
      component: getHomeViewPage
    },
    {
      path: '/about',
      name: 'about',
      component: getAboutViewPage
    }
  ]
})

export default router
