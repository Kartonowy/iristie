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
    <div class="profile">
        <div class="bio">
            <img :src=ctxCard.src :alt=ctxCard.alt>
            <div class="info">
                <span class="name">{{ ctxCard.alt }}</span>
                <span class="series">{{ ctxCard.series }}</span>
            </div>
        </div>
        <div class="aboutme">
            <p>{{ ctxCard.short }}</p>
        </div>
    </div>
    <div class="controls">
        <h6 @click="handleClose">delete</h6>
        <h6 @click="handleEdit">edit</h6>
        <h6 @click="close">close</h6>
    </div>
</template>

<style scoped>
    .profile {
        display: flex;
    }
    .bio img {
        width: 250px;
    }
    .info .name {
        font-size: 1.3em;
    }

    .info .series {
        color: #888888;
    }
    .aboutme {
        padding-left: 2.5vw;
        padding-right: 2.5vw;
    }

    .info {
        display: flex;
        flex-flow: wrap column;
    }
    .controls {
        display: flex;
    }

    .controls h6 {
        padding-left: 2vmin;
        padding-right: 2vmin;
        margin: 0;
    }
</style>