<script setup lang="ts">
import type { item } from '../utils/cards';
defineProps<{
    i: item
}>();
</script>

<template>
    <img :src="i.cover" :alt="i.title" />
</template>

<style scoped>
img {
    object-fit: cover;
    object-position: center;
    height: 250px;
    width: 146px;

    border-top-left-radius: 15px;
    border-bottom-right-radius: 15px;
    padding: 2px 2px 2px 2px;
}
</style>
