<script setup lang="ts">
import { stuff } from '../utils/cards';
import Tile from './Tile.vue';


defineProps<{
    category: string
}>();

type Category = keyof typeof stuff;


</script>

<template>
    <h2 class="sum">Currently viewing: {{ category }}</h2>
    <div class="shelf">
        <Tile v-for="item in stuff[category as Category].items" :i="item" />
    </div>
</template>

<style scoped>
.sum {
    text-align: center;
}

.shelf {
    margin-left: 6vw;
    margin-right: 6vw;
    align-items: center;
    justify-content: center;
    display: flex;
    flex-flow: wrap row;
}
</style>
