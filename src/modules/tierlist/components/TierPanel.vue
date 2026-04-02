<script setup lang="ts">
import TierList from '../components/TierList.vue';
import Auth from '../components/controls/Auth.vue';
import { ref, useTemplateRef, type Ref } from 'vue';
import Picker from '../components/controls/Picker.vue';
import Dialog from '../components/Dialog.vue';
import { DialogKind, type CardType } from '../utils/types';
import { useRoute } from 'vue-router';

const search = ref("")
const compact_mode = ref(true)
const board_id = ref(0)
const dialog_kind = ref(DialogKind.None)
const ctxCard: Ref<CardType | null, CardType | null> = ref(null)
const t = useTemplateRef("tl")
const route = useRoute()


const changeDialog = (dk: DialogKind) => {
  dialog_kind.value = dk
}

const changeCtxCard = (cc: CardType) => {
  ctxCard.value = cc
}

const clickOff = () => {
  changeDialog(DialogKind.None)
}

</script>

<template>
  <span id="topbar">
    <Auth :change-dialog="changeDialog" />
    <input type="text" placeholder="search" v-model="search">
    <Picker :bel="Number(route.params.board)" :change-board="(id) => { board_id = id; t?.update_cards(id); }" />
    <span id="compact">
      Compact mode: <input type="checkbox" name="compact" v-model="compact_mode">
    </span>
  </span>
  <Dialog v-if="dialog_kind != DialogKind.None" 
          :dialog-kind="dialog_kind"
          :change-dialog="changeDialog"
          :change-ctx="changeCtxCard"
          :ctx-card="ctxCard!"
          :board="board_id"
           />
  <div v-if="dialog_kind != DialogKind.None" class="cover" @click="clickOff" /> <!-- TODO: MOVE THIS TO DIALOG -->
  <div>
    <TierList :search="search" :change-dialog="changeDialog"  ref="tl" :change-ctx="changeCtxCard" :compact_mode="compact_mode" />
  </div>
</template>

<style scoped>
#topbar {
  display: flex;
    justify-content: center;
    margin: 2vh 10vw 2vh 10vw;
    width: 80vw;
}
#topbar * {
  height: 80%;
  margin-left: 10px;
  margin-right: 10px;
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
