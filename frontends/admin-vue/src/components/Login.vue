<template>
  <div class="login">
    <div class="ui grid">
      <div class="eight wide column center aligned">
        <img alt="Payment logo" src="../assets/retro-coin-icon.png">

        <h1>{{ title }}</h1>

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
              <label>Email/No telp:</label>
              <input type="text" name="email" placeholder="User Name" ref="inputEmail">
            </div>
            <div class="field">
              <label>Kata kunci:</label>
              <input type="password" name="password" placeholder="Password" ref="inputPassword">
            </div>
            <div class="field">
              <div class="ui checkbox">
                <input type="checkbox" tabindex="0" class="hidden">
                <label>Remember me</label>
              </div>
            </div>
            <button v-on:click="doLogin($event)" class="ui button" type="submit">Masuk</button>
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
    title: String
  },
  data() {
    return {
      token: this.token
    };
  },
  methods: {
    doLogin: function(event) {
      var self = this;
      if (event) event.preventDefault();
      this.$apf
        .login(
          this.$refs.inputEmail.value,
          null,
          this.$refs.inputPassword.value
        )
        .then(resp => {
          if (resp.data.token) {
            // self.token = resp.data.token;
            // self.$session.set("token", self.token);
            this.$apf.getMeInfo().then(self._handleGetMeInfo);
          }
        })
        .catch(_e => {
          self.$notify({
            group: "auth",
            title: "Login",
            type: "warn",
            text: "Gagal melakukan login, email/phone atau kata kunci salah."
          });
        });
    },
    _handleGetMeInfo(_resp) {
      this.$router.push("/dashboard");
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
