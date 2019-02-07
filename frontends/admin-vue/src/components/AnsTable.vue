<template>
  <div>
    
    <div class="ui grid">
      <div class="eight wide column">

        <table class="ui celled table">
          <thead>
            <tr>
              <th v-for="col in columns" v-bind:key="col">{{col}}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="item in items" v-bind:key="item.id">
              <td data-label="ID">{{item.id}}</td>
              <td data-label="FullName">{{item.full_name}}</td>
              <td data-label="Email">{{item.email}}</td>
              <td data-label="Phone">{{item.phone_num}}</td>
            </tr>
          </tbody>
        </table>

      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: "AnsTable",
  // visible: true,
  // items: [],
  props: {
    dataSourceUrl: String,
    columns: Array
  },
  data() {
    return {
      items: this.items
    };
  },
  methods: {
  },
  created(){
    console.log("created")
    this.items = []
    var self = this;
    this.$apf.api().privateApi.get(this.dataSourceUrl + "?page=0&limit=10")
      .then((resp) => {
        self.items = resp.data.entries;
      });
  }
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="less">
</style>
