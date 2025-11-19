<script setup lang="ts">
import { ref } from 'vue';
import { DialogKind, type CardType } from '../utils/types';
import AddDialog from './dialogs/AddDialog.vue';
import AuthDialog from './dialogs/AuthDialog.vue';
import CardDialog from './dialogs/CardDialog.vue';
import DeleteDialog from './dialogs/DeleteDialog.vue';
import EditDialog from './dialogs/EditDialog.vue';


const props = defineProps<{
    ctxCard?: CardType  
    dialogKind: DialogKind
    close: () => void
}>();

const dialogKind = ref(props.dialogKind)
const ctxCard = ref(props.ctxCard)

const ck = (dk: DialogKind, ctx: CardType) => {
    dialogKind.value = dk
    ctxCard.value = ctx
}

</script>

<template>
    <div class="dialog">
        <AuthDialog v-if="dialogKind == DialogKind.AuthDialog" :close="close" />
        <AddDialog v-if="dialogKind == DialogKind.AddDialog" :close="close" />
        <EditDialog v-if="dialogKind == DialogKind.EditDialog" :ctx-card="ctxCard!" :close="close" />
        <DeleteDialog v-if="dialogKind == DialogKind.DeleteDialog" :close="close" :ctx-card="ctxCard!" />
        <CardDialog v-if="dialogKind == DialogKind.CardDialog" :close="close" :ctx-card="ctxCard!" :change-dialog="ck" />
    </div>
</template>

<style scoped>
        .dialog {
            background-color: #383838;
            border: 1px red solid;
            position: absolute;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            border-radius: 15px;
            padding: 4vmin;
            display: flex;
            flex-flow: column wrap;
        }
</style>