import Vue from 'vue'
import VueSession from 'vue-session';
import App from './App.vue'
import router from './router'
import store from './store'
import Apf from './plugins/apf';
import './registerServiceWorker'


Vue.config.productionTip = false

Vue.use(VueSession)
Vue.use(Apf)


new Vue({
  router,
  store,
  render: h => h(App)
}).$mount('#app')
