<script setup lang="ts">
import axios from 'axios';
import type { CardType } from '../../utils/types';
import { DialogKind } from '../../utils/types';

const props = defineProps<{
    board: number
    changeDialog: (dk: DialogKind) => void
    ctxCard: CardType
}>();



const handleDelete = () => {
    axios({
        method: "post",
        url: "/api/delete",
        data: {
            alt: props.ctxCard.alt,
            series: props.ctxCard.series,
            board_id: props.board
        },
        headers: {
            Authorization: props.board
        },
    })
    .then((_res) => {
        props.changeDialog(DialogKind.None)
    })
}


</script>

<template>
    <h2>Are you sure you want to delete</h2>
    <button type="button" class="delete" @click="handleDelete">Delete</button>
    <h6 @click="changeDialog(DialogKind.None)">close</h6>
</template>

<style scoped>
</style>