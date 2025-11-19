<script setup lang="ts">
import { DialogKind, type CardType } from '../../utils/types';


const props = defineProps<{
    close: () => void
    ctxCard: CardType
    changeDialog: (dk: DialogKind, ctx: CardType) => void
}>();

const checkCookie = async () => {
    if (await cookieStore.get("Auth")) {
        return true;
    }
    alert("You don't have permission to do that.")
    return false;
}


const handleClose = async () => {
    if (await checkCookie()) {
        props.changeDialog(DialogKind.DeleteDialog, props.ctxCard)
    }
}

const handleEdit = async () => {
    if (await checkCookie()) {
        props.changeDialog(DialogKind.EditDialog, props.ctxCard)
    }
}

</script>

<template>
    <h1>{{ ctxCard.alt }}</h1>
    <p>{{ ctxCard.short }}</p>
    <div class="controls">
        <h6 @click="handleClose">delete</h6>
        <h6 @click="handleEdit">edit</h6>
        <h6 @click="close">close</h6>
    </div>
</template>

<style scoped>
    .controls {
        display: flex;
    }

    .controls h6 {
        padding-left: 2vmin;
        padding-right: 2vmin;
    }
</style>