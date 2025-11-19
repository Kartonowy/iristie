<script setup lang="ts">
import { ref } from 'vue';
import Dialog from './Dialog.vue';
import { DialogKind } from '../utils/types';



const dialogShown = ref(false);
const dialogKind = ref(DialogKind.AuthDialog)

const close = () => {
    dialogShown.value = false;
};

const handleClick = async () => {
    console.log(document.cookie)
    if (await cookieStore.get("Auth")) {
        console.log("dih")
        dialogKind.value = DialogKind.AddDialog
    }
    dialogShown.value = !dialogShown.value
}


</script>

<template>
    <button type="button" @click="handleClick" >+</button>
    <Dialog v-if="dialogShown" :dialog-kind="dialogKind" :close="close" />
</template>

<style scoped>
</style>