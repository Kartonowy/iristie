<script setup lang="ts">
import { ref } from 'vue';
import { getCards } from '../utils/get';
import Card from './Card.vue';
import { DialogKind, type CardType } from '../utils/types';

const props = defineProps<{
    search: string ,
    changeDialog: (dk: DialogKind) => void
    changeCtx: (cc: CardType) => void
}>();

const update_cards = async (bid: number) => {
    let cards: CardType[] = []
    cards = await getCards(bid)

    console.log(bid)
    for (const key of Object.keys(tiers.value)) {
        tiers.value[key] = []
    }

    for(const card of cards) {
        tiers.value[card.tier].push(card)
    }
}



const tiers: any = ref({
    "SSS": [],
    "SS": [],
    "S": [],
    "A": [],
    "B": [],
    "C": [],
    "D": [],
    "E": [],
    "F": []
})

const showCD = ref(false);

const setCtx = async (card: CardType) => {
    // props.changeCtx(card)
    props.changeCtx({...card})
    showCD.value = true;
    props.changeDialog(DialogKind.CardDialog)
}

const clickOff = () => {
    if (showCD) {
        showCD.value = !showCD;
        props.changeDialog(DialogKind.None)
    }
}

defineExpose({
    update_cards
})

</script>

<template>
    <div v-for="(value, key) of tiers" :class="key" class="row">
        <div class="tiermark">{{ key }}</div>
        <div class="items">
            <Card
                v-for="(card, index) in value.filter((e: CardType) => e.alt.includes(props.search) || e.series.includes(props.search))"
                :key="index" :card="card" :set-ctx="setCtx" />
        </div>
    </div>
    <div v-if="showCD" class="cover" @click="clickOff" /> <!-- TODO: MOVE THIS TO DIALOG -->
</template>

<style scoped>
.cover {
    position: fixed;
    top: 0;
    bottom: 0;
    width: 100%;
    height: 100%;
    z-index: 10;
}
.row {
    width: calc(12 * 100px);
    display: flex;
    flex-wrap: wrap;
    align-self: flex-start;
}
.items {
    width: calc(11 * 100px);
    display: flex;
    flex-wrap: wrap;
    align-self: flex-start;
}

.tiermark {
    width: 100px;
    min-height: 150px;
    display: flex;
    align-items: center;
    justify-content: center;
}
.SSS .tiermark {
    background: linear-gradient(45deg, #d86f92 0%, #daef63 100% );
}
.SS .tiermark {
    background-color: palevioletred;
}
.S .tiermark {
    background-color: red;
}
.A .tiermark {
    background-color: orange;
}
.B .tiermark {
    background-color: yellow;
}
.C .tiermark {
    background-color: greenyellow;
}
.D .tiermark {
    background-color: green;
}
.E .tiermark {
    background-color: grey;
}
.F .tiermark {
    background-color: black;
}
</style>