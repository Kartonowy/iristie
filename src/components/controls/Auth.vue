<script setup lang="ts">
import { ref } from 'vue';
import { DialogKind } from '../../utils/types';


const props = defineProps<{
    changeDialog: (dk: DialogKind) => void
}>();

const vis = ref(false);


const handleClick = async () => {
    if (vis.value) {
        props.changeDialog(DialogKind.None)
        vis.value = false;
        return;
    }

    if (await cookieStore.get("Auth")) {
        props.changeDialog(DialogKind.AddDialog)
        vis.value = true;
    } else {
        props.changeDialog(DialogKind.AuthDialog)
        vis.value = true;
    }

}


</script>

<template>
    <button type="button" @click="handleClick" >+</button>
</template>

<style scoped>
</style>