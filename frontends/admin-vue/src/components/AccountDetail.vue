<template>
  <div id="AccountDetail">
    <div class="ui grid" v-if="$route.path.startsWith('/dashboard/accounts/')">
      <div class="six wide column">
        <table class="ui celled table">
          <tbody>
            <tr>
              <td data-label="ID">ID:</td>
              <td class="value">{{d.id}}</td>
            </tr>
            <tr>
              <td data-label="Name">Full name:</td>
              <td class="value">{{d.full_name}}</td>
            </tr>
            <tr>
              <td data-label="Email">Email:</td>
              <td class="value">{{d.email}}</td>
            </tr>
            <tr>
              <td data-label="Phone">Phone:</td>
              <td class="value">{{d.phone_num}}</td>
            </tr>
            <tr>
              <td data-label="Balance">Balance:</td>
              <td class="value">{{d.balance}}</td>
            </tr>
            <tr>
              <td data-label="Active">Active:</td>
              <td class="value">{{d.active ? "YES" : "NO"}}</td>
            </tr>
          </tbody>
        </table>
      </div>
      <div class="six wide column">
        <div class="ui segment">
          <h3>Credit</h3>
          <p>Credit this account balance</p>
          <form action="javascript://" class="ui form">
            <div class="field">
              <label for="amount">Amount:</label>
              <input type="number" name="amount" id="Amount" placeholder="Amount" ref="inputAmount" />
            </div>
            <div class="field">
              <button class="ui button" type="submit" v-on:click="doCredit($event);">Credit</button>
            </div>
          </form>
        </div>
      </div>
    </div>
  </div>
</template>

<script>



export default {
  name: "AccountDetail",
  components: {},
  props: {
    accountId: String
  },
  data() {
    return {
      d: {}
    };
  },
  created() {
    console.log("AccountDetail created.");
    if (!this.accountId) return;
    this.$apf
      .api()
      .privateApi.get(`/payment/v1/account/info?id=${this.accountId}`)
      .then(resp => {
        this.d = resp.data.result;
      });
  },
  methods: {
    doCredit(event){
      if (event) event.preventDefault();
      let amount = this.$refs.inputAmount.value;
      this.$apf.creditAccountBalance(this.d.id, amount)
        .then((resp) => {
          this.d.balance = resp.data.result;
          this.$notify({
            group: "default",
            title: "Success",
            type: "info",
            text: "Akun `" + this.d.full_name + "`telah di-credit sebesar " + amount
          });
        });
    }
  }
};
</script>

<style lang="less" scoped>
.value {
  font-weight: bold;
}
tr td:first-child {
  text-align: right !important;
}
</style>
