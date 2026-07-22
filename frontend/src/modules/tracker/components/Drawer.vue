<script setup lang="ts">
import { stuff } from '../utils/cards';

defineProps<{
    setCategory: (new_category: string) => void
}>();

</script>

<template>
  <div class="drawer">
        <h2 v-for="key in stuff" class="category-tile" :class="key.name" @click="setCategory(key.name)">
            {{ key.name }} [{{ key.items.length }} / {{ key.target }}]
        </h2>
  </div>
</template>

<style scoped>

.category-tile {
    display: flex;
    height: 2em;
    border-top: 1px rgba(255, 255, 255, 0.87) solid;
    border-bottom: 1px rgba(255, 255, 255, 0.87) solid;
    flex-flow: column wrap;
    align-items: center;
    justify-content: center;
    cursor: pointer;
}

.drawer {
    display: flex;
    flex-flow: column wrap;
    justify-content: space-evenly;
}

</style>
