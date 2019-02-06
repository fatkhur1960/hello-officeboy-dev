<template>
  <div class="home">
    <div class="ui grid">
      <div class="four wide column">
        <h1>Dashboard</h1>
      </div>
    </div>
  </div>
</template>

<script>
// @ is an alias to /src
// import Login from '@/components/Login.vue'

export default {
  name: "dashboard",
  components: {
    // Dashboard
  },
  created() {
    console.log("created");
    this.startLoginChecker();
  },
  destroyed() {
    console.log("destroyed");
    clearInterval(this.loginCheckerIval);
  },
  methods: {
    startLoginChecker() {
      var self = this;
      this.loginCheckerIval = setInterval(() => {
        this.$apf.isLoggedIn(loggedIn => {
          if (!loggedIn) {
            self.$router.replace("/");
          }
        });
      }, 1000);
    }
  }
};
</script>
