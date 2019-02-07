<template>
  <div class="home">
    <div style="float: left;">
      <sidebar-menu :menu="menu" @collapse="onCollapse" @itemClick="onItemClick" :collapsed="true" style="z-index: 1000;" />
    </div>

    <div class="dashboard-inner" v-bind:style="customMargin">
      <h1>Dashboard</h1>
      <AnsTable v-if="currentPage['/dashboard/accounts']" :columns="['ID', 'Name', 'Email', 'Phone', 'Active', 'Register']"
         dataSourceUrl="/accounts"
         :searchable="true" />
    </div>
  </div>
</template>

<script>
// @ is an alias to /src
import AnsTable from "@/components/AnsTable.vue";

export default {
  name: "dashboard",
  components: {
    AnsTable
  },
  data() {
    // console.log(this.$router.history.current.path);
    // var currentPage = this.currentPage ? this.currentPage : {};
    // currentPage[this.$router.history.current.path] = true;
    return {
      collapsed: true,
      customMargin: {},
      currentPage: {},
      // accountVisibility: false,
      menu: [
        {
          header: true,
          title: "Main Navigation"
          // component: componentName
          // visibleOnCollapse: true
        },
        {
          href: "/dashboard",
          title: "Dashboard",
          icon: "fa fa-user"
          /*
                        disabled: true
                        badge: {
                            text: 'new',
                            // class:''
                        }
                        */
        },
        {
          title: "Accounts",
          icon: "fa fa-users",
          href: "/dashboard/accounts"
          // child: [
          //   {
          //     href: "/dashboard/accounts",
          //     title: "List"
          //   }
          // ]
        },
        {
          title: "Logout",
          icon: "fa fa-sign-out-alt",
        }
      ]
    };
  },
  created() {
    // console.log("created");
    this.customMargin = {
      "left": "70px",
      "position": "absolute"
    };

    this.currentPage = {};
    // this.currentPage['/dashboard/accounts'] = false;
    // this.currentPage[this.$router.history.current.path] = true;
    this.$set(this.currentPage, this.$router.history.current.path, true);
    // this.accountVisibility = this.$router.history.current.path == "/dashboard/accounts";

    this.startLoginChecker();
    
  },
  destroyed() {
    // console.log("destroyed");
    clearInterval(this.loginCheckerIval);
  },
  methods: {
    isCurrentPage(title) {
      return this.currentPage == title;
    },
    startLoginChecker() {
      var self = this;
      this.loginCheckerIval = setInterval(() => {
        this.$apf.isLoggedIn(loggedIn => {
          if (!loggedIn) {
            self.$router.replace("/");
          }
        });
      }, 1000);
    },
    onCollapse(state) {
      this.collapsed = state;
      this.customMargin = {
        "left": this.collapsed ? "70px" : "370px",
        "position": "absolute"
      };
    },
    onItemClick(event, item) {
      // console.log(item);
      var currentPage = this.currentPage;
      for (var _href in currentPage) {
        // console.log(_href);
        // currentPage[_href] = false;
        this.$set(currentPage, _href, false);
      }
      // currentPage[item.href] = true;
      this.$set(currentPage, item.href, true);

      // this.currentPage = currentPage;

      // this.accountVisibility = item.title == "Accounts";

      if (item.title == 'Logout'){
        this.$apf.unauthorize();
      }
    }
  }
};
</script>


<style lang="less" scoped>
.dashboard-inner {
  width: 100%;
  transition: all 0.1s ease-in-out;
    -webkit-transition: all 0.1s ease-in-out; /** Chrome & Safari **/
    -moz-transition: all 0.1s ease-in-out; /** Firefox **/
    -o-transition: all 0.1s ease-in-out; /** Opera **/
}
</style>
