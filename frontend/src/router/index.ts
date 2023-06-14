// Composables
import { createRouter, createWebHistory } from 'vue-router'

import Dashboard from '@/views/Dashboard.vue';

const routes = [
  {
    path: '/solostrike/',
    component: () => import('@/views/Dashboard.vue'),
    // children: [
    //   {
    //     path: '',
    //     name: 'Home',
    //     // route level code-splitting
    //     // this generates a separate chunk (about.[hash].js) for this route
    //     // which is lazy-loaded when the route is visited.
    //     component: () => import(/* webpackChunkName: "home" */ '@/views/Dashboard.vue'),
    //   },
    // ],
    meta: {
      title: "Dashboard",
    }
  },
  {
    path: '/solostrike/play',
    component: () => import('@/views/Play.vue'),
    meta: {
      title: "Spielen",
    }
  }
]

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes,
})

export default router
