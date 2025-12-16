<script setup lang="ts">
import { ref } from 'vue';
import Dialog from '../Dialog.vue';
import { DialogKind } from '../../utils/types';



const dialogShown = ref(false);
const dialogKind = ref(DialogKind.CalcDialog)

const close = () => {
    dialogShown.value = false;
};

const handleClick = async () => {
    dialogShown.value = !dialogShown.value
}


</script>

<template>
    <button type="button" @click="handleClick" >/</button>
    <Dialog v-if="dialogShown" :dialog-kind="dialogKind" :close="close" />
</template>

<style scoped>
</style>