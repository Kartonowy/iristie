<script setup lang="ts">
import TierList from './components/TierList.vue';
import Auth from './components/controls/Auth.vue';
import { ref } from 'vue';
import Help from './components/controls/Help.vue';
import Calc from './components/controls/Calc.vue';

const search = ref("")

</script>

<template>
  <h2>Shikanoko's fictional character tierlist of great accuracy and wisdom
    <input type="text" placeholder="search" v-model="search">
    <Help />
    <Calc />
    <Auth />
  </h2>
  <div>
    <TierList :search="search" />
  </div>
</template>

<style scoped>
</style>
