<script setup lang="ts">
import { stuff } from '../utils/cards';
import Tile from './Tile.vue';


defineProps<{
    category: string
}>();

type Category = keyof typeof stuff;


</script>

<template>
    <div class="shelf">
        <Tile v-for="item in stuff[category as Category].items" :i="item" />
    </div>
</template>

<style scoped>
.shelf {
    display: flex;
    flex-wrap: wrap;
}
</style>
