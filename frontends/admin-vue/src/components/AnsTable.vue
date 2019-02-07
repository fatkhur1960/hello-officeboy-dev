<template>
  <div>
    <div class="ui grid">
      <div class="ten wide column">
        <div v-if="searchable" class="ui icon input">
          <input type="text" placeholder="Search..." v-on:keyup.13="doSearch" ref="inputSearch">
          <i class="search icon"></i>
        </div>

        <table class="ui celled table">
          <thead>
            <tr>
              <th v-for="col in columns" v-bind:key="col">{{col}}</th>
            </tr>
          </thead>
          <tbody v-html="buildRow()">
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: "AnsTable",
  props: {
    dataSourceUrl: String,
    columns: Array,
    searchable: Boolean,
    itemMap: {
      type: Array,
      default: () => {
        return [];
      }
    }
  },
  data() {
    return {
      items: this.items,
      // searchable: false
    };
  },
  methods: {
    doSearch() {
      var url =
        this.dataSourceUrl +
        `?query=${this.$refs.inputSearch.value}&page=${this.page}&limit=${this.limit}`;
      this.$apf
        .api()
        .privateApi.get(url)
        .then(resp => {
          this.items = resp.data.entries;
        });
    },
    buildRow(){
      return this.items.map(item => {
        return '<tr>' + this.itemMap.map(col => {
          return `<td>${item[col]}</td>`;
        }).join("") + '</tr>';
      }).join("");
    }
  },
  created() {
    console.log("created");
    this.items = [];
    this.page = 0;
    this.limit = 5;
    var self = this;
    var url;

    if (this.searchable && this.query) {
      url = this.dataSourceUrl + "?q=" + this.query + "&page=0&limit=10";
    } else {
      url = this.dataSourceUrl + "?page=0&limit=10";
    }

    this.$apf
      .api()
      .privateApi.get(url)
      .then(resp => {
        self.items = resp.data.entries;
      });
  }
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="less">
</style>
