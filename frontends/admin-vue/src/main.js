import Vue from 'vue'
import VueSession from 'vue-session';
import VueSidebarMenu from "vue-sidebar-menu";
import 'vue-sidebar-menu/dist/vue-sidebar-menu.css'
import Notifications from 'vue-notification'

import App from './App.vue'
import router from './router'
import store from './store'
import Apf from './plugins/apf';

import './registerServiceWorker'


Vue.config.productionTip = false

// Run mode ini menerima nilai:
// * `prod` - Apabila ingin menggunakan API dari server production.
// * `dev` - Apabila ingin menggunakan API dari server local atau docker (untuk development).
// * `mock` - Apabila ingin menggunakan API dari server mocking Apiary (untuk development).
Vue.config.runMode = "dev";

Vue.use(VueSession)
Vue.use(Notifications)
Vue.use(Apf)
Vue.use(VueSidebarMenu)

new Vue({
  router,
  store,
  render: h => h(App)
}).$mount('#app')
