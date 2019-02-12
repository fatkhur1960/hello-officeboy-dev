<template>
  <div id="AccountDetail">
    <h3>#{{d.id}}</h3>
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
</template>

<script>
export default {
  name: "AccountDetail",
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
    this.$apf
      .api()
      .privateApi.get(`/account/info?id=${this.accountId}`)
      .then(resp => {
        this.d = resp.data.result;
      });
  }
};
</script>

<style lang="less" scoped>
.value {
  font-weight: bold;
}
tr td:first-child{
  text-align: right !important;
}
</style>
