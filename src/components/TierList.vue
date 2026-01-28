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


const setCtx = async (card: CardType) => {
    // props.changeCtx(card)
    props.changeCtx({...card})
    props.changeDialog(DialogKind.CardDialog)
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
    <div class="stats">stats: {{ Object.values(tiers).map((e: any) => e.length).reduce((a: number, b: number) => a + b) }}</div>
</template>

<style scoped>
@import url('https://fonts.googleapis.com/css2?family=Playwrite+IN+Guides&display=swap');
.stats {
    text-align: center;
font-family: "Playwrite IN Guides", cursive;
font-size: 4em;
}
.row {
    margin-left: 10vw;
    margin-right: 10vw;
    width: 80vw;
    display: flex;
    flex-wrap: wrap;
    align-self: flex-start;
}
.items {
    width: calc(80vw - 100px);
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
    color: #423939;
}
.SSS .tiermark {
    background: linear-gradient(-66.6deg,  #ff7f7f 0%, red 100% );
}
.SS .tiermark {
    background-color: #ff7f7f;
}
.S .tiermark {
    background-color: #ffbf7f;
}
.A .tiermark {
    background-color: #ffdf7f;
}
.B .tiermark {
    background-color: #ffff7f;
}
.C .tiermark {
    background-color: #bfff7f;
}
.D .tiermark {
    background-color: #7fff7f;
}
.E .tiermark {
    background-color: #7fffff;
}
.F .tiermark {
    background-color: #7fbfff;
}
</style>