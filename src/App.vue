<script setup lang="ts">
import TierList from './components/TierList.vue';
import Auth from './components/controls/Auth.vue';
import { ref, useTemplateRef, type Ref } from 'vue';
import Picker from './components/controls/Picker.vue';
import Dialog from './components/Dialog.vue';
import { DialogKind, type CardType } from './utils/types';

const search = ref("")
const board_id = ref(0)
const dialog_kind = ref(DialogKind.None)
const ctxCard: Ref<CardType | null, CardType | null> = ref(null)
const t = useTemplateRef("tl")


const changeDialog = (dk: DialogKind) => {
  console.log("changing from " + dialog_kind.value + " to " + dk)
  dialog_kind.value = dk
}

const changeCtxCard = (cc: CardType) => {
  console.log("changing context")
  ctxCard.value = cc
  console.log(ctxCard.value)
}


</script>

<template>
  <h2>Shikanoko's fictional character tierlist of great accuracy and wisdom
    <input type="text" placeholder="search" v-model="search">
    <Picker :change-board="(id) => { board_id = id; t?.update_cards(id); }" />
    <Auth :change-dialog="changeDialog" />
  </h2>
  <Dialog v-if="dialog_kind != DialogKind.None" :dialog-kind="dialog_kind" :change-dialog="changeDialog" :change-ctx="changeCtxCard" :ctx-card="ctxCard!"  />
  <div>
    <TierList :search="search" :change-dialog="changeDialog"  ref="tl" :change-ctx="changeCtxCard" />
  </div>
</template>

<style scoped>
</style>
