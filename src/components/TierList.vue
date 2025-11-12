<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { getCards } from '../utils/dummy';
import Card from './Card.vue';
import type { CardType } from '../utils/types';



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


onMounted(async () => {
    let cards: CardType[] = []
    cards = await getCards()

    for(const card of cards) {
        tiers.value[card.tier].push(card)
    }

    console.log(tiers)
    
})


</script>

<template>
    <div v-for="(value, key) of tiers" :class="key" class="row">
        <div class="tiermark">{{ key }}</div>
        <div class="items">
            <Card v-for="card in value" :id="card.id" :src="card.src" :alt="card.alt" :short="card.short" :series="card.series" />
        </div>
    </div>
</template>

<style scoped>
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