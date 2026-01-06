<script setup lang="ts">
import { DialogKind, type CardType } from '../utils/types';
import AddDialog from './dialogs/AddDialog.vue';
import AuthDialog from './dialogs/AuthDialog.vue';
import CardDialog from './dialogs/CardDialog.vue';
import DeleteDialog from './dialogs/DeleteDialog.vue';
import EditDialog from './dialogs/EditDialog.vue';


const props = defineProps<{
    board: number,
    ctxCard?: CardType  
    dialogKind: DialogKind
    changeDialog: (dk: DialogKind) => void
    changeCtx: (cc: CardType) => void
}>();

const ck = (dk: DialogKind, ctx: CardType) => {
    props.changeDialog(dk)
    props.changeCtx(ctx)
}

</script>

<template>
    <div class="dialog">
        <AuthDialog v-if="dialogKind == DialogKind.AuthDialog" :change-dialog="changeDialog" :board="board" />
        <AddDialog v-if="dialogKind == DialogKind.AddDialog" :change-dialog="changeDialog" :board="board" />
        <EditDialog v-if="dialogKind == DialogKind.EditDialog" :ctx-card="ctxCard!" :change-dialog="changeDialog" :board="board" />
        <DeleteDialog v-if="dialogKind == DialogKind.DeleteDialog" :change-dialog="changeDialog" :ctx-card="ctxCard!" :board="board" />
        <CardDialog v-if="dialogKind == DialogKind.CardDialog" :ctx-card="ctxCard!" :change-dialog="ck" />
    </div>
</template>

<style scoped>
        .dialog {
            z-index: 15;
            background-color: #383838;
            border: 1px #434343 solid;
            position: fixed;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            border-radius: 15px;
            padding: 4vmin;
            display: flex;
            flex-flow: column wrap;
        }
</style>