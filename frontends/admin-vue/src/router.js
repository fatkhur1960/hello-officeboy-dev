import Vue from 'vue'
import Router from 'vue-router'
import Home from './views/Home.vue'
import Dashboard from './views/Dashboard.vue'
// import DashboardAccounts from './views/dashboard/Accounts.vue'
// import DashboardTransactions from './views/dashboard/Transactions.vue'

Vue.use(Router)

export default new Router({
  mode: 'history',
  base: process.env.BASE_URL,
  routes: [
    {
      path: '/',
      name: 'home',
      component: Home
    },
    {
      path: '/dashboard',
      name: 'Dashboard',
      component: Dashboard
    },
    {
      path: '/dashboard/accounts',
      name: 'Accounts',
      component: Dashboard
    },
    {
      path: '/dashboard/accounts/:id',
      name: 'Account',
      component: Dashboard
    },
    {
      path: '/dashboard/transactions',
      name: 'Transactions',
      component: Dashboard
    },
    {
      path: '/about',
      name: 'about',
      // route level code-splitting
      // this generates a separate chunk (about.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: () => import(/* webpackChunkName: "about" */ './views/About.vue')
    }
  ]
})
