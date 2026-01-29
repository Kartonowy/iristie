<script setup lang="ts">
import { ref } from 'vue';
import Drawer from './Drawer.vue';
import Shelf from './Shelf.vue';

const category = ref("anime")

const setCategory = (new_category: string) => {
    category.value = new_category.toLowerCase();
}
</script>

<template>
  <h2 class="sum">What I've consumed in 2026:</h2>
  <Drawer :set-category="setCategory" />
  <Shelf :category="category" />
</template>

<style scoped>
.sum {
    text-align: center;
}
</style>
