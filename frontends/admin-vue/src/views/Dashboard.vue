<template>
  <div class="home">
    <div style="float: left;">
      <sidebar-menu
        :menu="menu"
        @collapse="onCollapse"
        @itemClick="onItemClick"
        :collapsed="true"
        style="z-index: 1000;"
      />
    </div>

    <div class="dashboard-inner" v-bind:style="customMargin">
      <h1>{{ pageTitle }}</h1>

      <AnsTable
        v-if="currentPage['/dashboard']"
        data-source-url="/accounts"
        :columns="['ID', 'Balance']"
        :itemMap="['id', 'balance']"
        :searchable="true"
        :withActionButton="true"
        :mapItemFunc="userListAllMapper2"
      ></AnsTable>

      <AnsTable
        v-if="currentPage['/dashboard/accounts']"
        data-source-url="/accounts"
        :columns="['ID', 'Name', 'Email', 'Phone', 'Active', 'Register']"
        :itemMap="['id', 'full_name', 'email', 'phone_num', 'active', 'register_time']"
        :searchable="true"
        :withActionButton="true"
        :mapItemFunc="userListAllMapper"
      />

      <AnsTable
        v-if="currentPage['/dashboard/transactions']"
        data-source-url="/transactions"
        :columns="['ID', 'Kind', 'Credit', 'Debit', 'Timestamp', 'Status']"
        :itemMap="['id', 'aaa']"
        :searchable="false"
        :withActionButton="true"
        :mapItemFunc="txItemMap"
      />

      <div class="ui grid" v-if="$route.path.startsWith('/dashboard/accounts/')">
        <div class="six wide column">
          <AccountDetail
            :accountId="$route.params.id"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<script>
// @ is an alias to /src
import AnsTable from "@/components/AnsTable.vue";
import AccountDetail from "@/components/AccountDetail.vue";

export default {
  name: "Dashboard",
  components: {
    AnsTable,
    AccountDetail
  },
  data() {
    return {
      collapsed: true,
      customMargin: {},
      currentPage: {},
      pageTitle: this.pageTitle,
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
          title: "Transactions",
          icon: "fa fa-dollar-sign",
          href: "/dashboard/transactions"
        },
        {
          title: "Logout",
          icon: "fa fa-sign-out-alt"
        }
      ]
    };
  },
  created() {
    this.customMargin = {
      left: "70px",
      position: "absolute"
    };

    this.currentPage = {};
    // console.log(this.$router.history.current);
    // this.currentPage['/dashboard/accounts'] = false;
    // this.currentPage[this.$router.history.current.path] = true;
    this.$set(this.currentPage, this.$route.path, true);
    this.pageTitle = this.$router.history.current.name;
    // this.accountVisibility = this.$router.history.current.path == "/dashboard/accounts";

    this.startLoginChecker();
  },
  destroyed() {
    // console.log("destroyed");
    clearInterval(this.loginCheckerIval);
  },
  methods: {
    txItemMap(item) {
      return item;
    },
    userListAllMapper(item) {
      delete item["balance"];
      return item;
    },
    userListAllMapper2(item) {
      return {
        id: item["id"],
        balance: item["balance"]
      };
    },
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
        left: this.collapsed ? "70px" : "370px",
        position: "absolute"
      };
    },
    onItemClick(_event, _item) {
      // console.log(this.$rout.path);
      // var currentPage = this.currentPage;
      // for (var _href in currentPage) {
      //   // console.log(_href);
      //   // currentPage[_href] = false;
      //   this.$set(currentPage, _href, false);
      // }
      // // currentPage[item.href] = true;
      // this.$set(currentPage, item.href, true);
      // this.pageTitle = item.title;
      // // this.currentPage = currentPage;
      // // this.accountVisibility = item.title == "Accounts";
      // if (item.title == 'Logout'){
      //   this.$apf.unauthorize();
      // }
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
