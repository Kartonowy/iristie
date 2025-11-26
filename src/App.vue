<script setup lang="ts">
import TierList from './components/TierList.vue';
import Auth from './components/Auth.vue';
import { ref } from 'vue';

const search = ref("")

</script>

<template>
  <h1>Tieren machen <Auth /> <input type="text" placeholder="search" v-model="search"> </h1>
  <div>
    <TierList :search="search" />
  </div>
</template>

<style scoped>
</style>
