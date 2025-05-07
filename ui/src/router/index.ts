import { createRouter, createWebHistory } from 'vue-router'
const getHomeViewPage = () => import('@/views/HomeView.vue')
const getAboutViewPage = () => import('@/views/AboutView.vue')
const getExamplesPage = () => import('@/views/ExamplesView.vue')
const getMyGrammarsPage = () => import('@/views/MyMGsView.vue')

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
    },
    {
      path: '/examples',
      name: 'examples',
      component: getExamplesPage
    },
    {
      path: '/mygrammars',
      name: 'mygrammars',
      component: getMyGrammarsPage
    }
  ]
})

export default router
