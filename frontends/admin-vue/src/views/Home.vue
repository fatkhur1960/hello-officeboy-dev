<template>
  <div class="home">
    <Login title="Ansvia Payment Framework" />
  </div>
</template>

<script>
// @ is an alias to /src
import Login from '@/components/Login.vue'

export default {
  name: 'home',
  components: {
    Login
  }
}
</script>
