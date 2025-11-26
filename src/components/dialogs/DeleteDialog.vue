<script setup lang="ts">
import axios from 'axios';
import type { CardType } from '../../utils/types';


const props = defineProps<{
    close: () => void
    ctxCard: CardType
}>();



const handleDelete = () => {
    axios({
        method: "post",
        url: "/api/delete",
        data: {
            alt: props.ctxCard.alt,
            series: props.ctxCard.series
        }
    })
    .then((_res) => {
        props.close()
    })
}


</script>

<template>
    <h2>Are you sure you want to delete</h2>
    <button type="button" class="delete" @click="handleDelete">Delete</button>
    <h6 @click="close">close</h6>
</template>

<style scoped>
</style>