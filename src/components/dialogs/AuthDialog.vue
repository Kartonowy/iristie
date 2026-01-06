<script setup lang="ts">
import axios from 'axios';
import { ref } from 'vue';
import { DialogKind } from '../../utils/types';
const props = defineProps<{
    board: number,
    changeDialog: (dk: DialogKind) => void
}>();

const pass = ref("")

const sendAuth = () => {
    console.log("attempting to log in with values: boardid " + props.board + " and pass " + pass.value)
    axios({
        method: "post",
        url: "/api/auth",
        headers: {
            Authorization: props.board
        },
        data: {
            id: props.board,
            pass: pass.value
        },
    })
    .then((_res) => {
        cookieStore.set({
            name: "Auth",
            value: `board${props.board}`,
        })
        props.changeDialog(DialogKind.None)
    })
}


</script>

<template>
    Your login
    <input type="password" name="pass" v-model="pass">
    <button type="button" class="auth" @click="sendAuth">Auth</button>
</template>

<style scoped>
    .auth {
        width: 25%
    }
</style>