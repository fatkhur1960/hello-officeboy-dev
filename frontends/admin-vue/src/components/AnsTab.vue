<template>
  <div class="ans-tab">
    <div class="ui top attached tabular menu">
      <div class="active item">Basic</div>
      <div class="item">Transactions</div>
    </div>
    <div class="ui bottom active attached tab segment">
      <slot></slot>
    </div>
  </div>
</template>

<script>
export default {
  name: "AnsTab",
  props: {

  },
  created(){

  }
}
</script>