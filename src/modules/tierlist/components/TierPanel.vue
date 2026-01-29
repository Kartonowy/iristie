<script setup lang="ts">
import TierList from '../components/TierList.vue';
import Auth from '../components/controls/Auth.vue';
import { ref, useTemplateRef, type Ref } from 'vue';
import Picker from '../components/controls/Picker.vue';
import Dialog from '../components/Dialog.vue';
import { DialogKind, type CardType } from '../utils/types';
import { useRoute } from 'vue-router';

const search = ref("")
const board_id = ref(0)
const dialog_kind = ref(DialogKind.None)
const ctxCard: Ref<CardType | null, CardType | null> = ref(null)
const t = useTemplateRef("tl")
const route = useRoute()


const changeDialog = (dk: DialogKind) => {
  dialog_kind.value = dk
}
console.log(route.params.board)

const changeCtxCard = (cc: CardType) => {
  ctxCard.value = cc
}

const clickOff = () => {
  changeDialog(DialogKind.None)
}

</script>

<template>
  <h2>Shikanoko's fictional character tierlist of great accuracy and wisdom
    <input type="text" placeholder="search" v-model="search">
    <Picker :bel="Number(route.params.board)" :change-board="(id) => { board_id = id; t?.update_cards(id); }" />
    <Auth :change-dialog="changeDialog" />
  </h2>
  <Dialog v-if="dialog_kind != DialogKind.None" 
          :dialog-kind="dialog_kind"
          :change-dialog="changeDialog"
          :change-ctx="changeCtxCard"
          :ctx-card="ctxCard!"
          :board="board_id"
           />
  <div v-if="dialog_kind != DialogKind.None" class="cover" @click="clickOff" /> <!-- TODO: MOVE THIS TO DIALOG -->
  <div>
    <TierList :search="search" :change-dialog="changeDialog"  ref="tl" :change-ctx="changeCtxCard" />
  </div>
</template>

<style scoped>
h2 {
    text-align: center;
    margin-left: 10vw;
    margin-right: 10vw;
    width: 80vw;
}
.cover {
    position: fixed;
    top: 0;
    bottom: 0;
    width: 100%;
    height: 100%;
    z-index: 10;
}
</style>
