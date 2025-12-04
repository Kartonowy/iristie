<script setup lang="ts">
import TierList from './components/TierList.vue';
import Auth from './components/Auth.vue';
import { ref } from 'vue';

const search = ref("")

</script>

<template>
  <h2>Shikanoko's fictional character tierlist of great accuracy and wisdom <Auth /> <input type="text" placeholder="search" v-model="search"> </h2>
  <div>
    <TierList :search="search" />
  </div>
</template>

<style scoped>
</style>
