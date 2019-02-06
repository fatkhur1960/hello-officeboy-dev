<template>
  <div class="login">
    <div class="ui grid">
      <div class="eight wide column">
        <img alt="Payment logo" src="../assets/retro-coin-icon.png">

        <h1>{{ title }}</h1>

        <h2>Welcome to Ansvia Payment Framework</h2>

        <p>an Easy Payment Solution</p>
      </div>
      <div class="seven wide column left aligned">
        <div class="ui raised very padded container segment">
          <div class="ui container">
            <h1>ADMINISTRATOR LOGIN</h1>
          </div>
          <div class="ui divider"></div>
          <form class="ui form" method="POST">
            <div class="field">
              <label>User Name:</label>
              <input type="text" name="email" placeholder="User Name">
            </div>
            <div class="field">
              <label>Password:</label>
              <input type="password" name="password" placeholder="Password">
            </div>
            <div class="field">
              <div class="ui checkbox">
                <input type="checkbox" tabindex="0" class="hidden">
                <label>Remember me</label>
              </div>
            </div>
            <button v-on:click="warn($event, 'I warn you!')" class="ui button" type="submit">Masuk</button>
          </form>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: "Login",
  props: {
    msg: String
  },
  methods: {
    warn: function(event, msg) {
      if (event) event.preventDefault();
      alert(msg);
    }
  }
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="less">
h3 {
  margin: 40px 0 0;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}
</style>
