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
    width: 150px;
}
</style>
