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
    <div class="tracker">
        <h1 class="sum">What {{ category }} have I completed in 2026:</h1>
        <Drawer class="drawer" :set-category="setCategory" />
        <Shelf class="shelf" :category="category" />
    </div>
</template>

<style scoped>
.tracker {
    display:flex; 
    flex-flow: wrap row;
}
.sum {
    text-align: center;
    width: 100%;
}
.drawer {
    width: 15%;
    height: 85%;
}
.shelf {
    width: 81%;
    padding: 1%;
}
</style>
