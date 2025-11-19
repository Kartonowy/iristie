<script setup lang="ts">
import axios from 'axios';
import { ref } from 'vue';



const props = defineProps<{
    close: () => void
}>();

const pass = ref("")

const sendAuth = () => {
    axios({
        method: "post",
        url: "/api/auth",
        data: {
            pass: pass.value
        },
    })
    .then((res) => {
        console.log(res)
        cookieStore.set("Auth", "true")
        props.close()
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