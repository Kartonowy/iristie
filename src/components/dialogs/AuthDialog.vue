<script setup lang="ts">
import axios from 'axios';
import { ref } from 'vue';
import { DialogKind } from '../../utils/types';
const props = defineProps<{
    changeDialog: (dk: DialogKind) => void
}>();

const board = ref("")
const pass = ref("")

const sendAuth = () => {
    axios({
        method: "post",
        url: "/api/auth",
        data: {
            id: board.value,
            pass: pass.value
        },
    })
    .then((_res) => {
        cookieStore.set({
            name: "Auth",
            value: "true",
        })
        props.changeDialog(DialogKind.None)
    })
}


</script>

<template>
    Your login
    <input type="text" name="board" v-model="board">
    <input type="password" name="pass" v-model="pass">
    <button type="button" class="auth" @click="sendAuth">Auth</button>
</template>

<style scoped>
    .auth {
        width: 25%
    }
</style>