<script setup lang="ts">
import axios from 'axios';
import { ref } from 'vue';

//  export type CardType  = {
//     id: number, // ID of the character for indexing purpose
//     src: string, // SRC meaning url to the image of the character
//     alt: string, // ALT always consists of character's name
//     series: string, // SERIES for where the character comes from
//     tier: string, // TIER for tier alignment purposes
//     short: string // SHORT for short description of the character, reasoning behind its placement
// }

import { DialogKind } from '../../utils/types';
const props = defineProps<{
    board: number
    changeDialog: (dk: DialogKind) => void
}>();
const name = ref("")
const series = ref("")
const short = ref("")
const image = ref("")
const tier = ref("SSS")
const unsetfunc = ref(() => {})

const setTier = (e: any) => {
    const t = e.target?.className
    unsetfunc.value()
    tier.value = t
    e.target?.classList.add('selected')
    unsetfunc.value = () => {e.target?.classList.remove('selected')}
}


const handleAdd = () => {
    // const card: CardType = {
    //     alt: name.value,
    //     series: series.value,
    //     src: image.value,
    //     tier: tier.value,
    //     short: short.value
    // }
    axios({
        method: "post",
        url: "/api/add",
        data: {
            alt: name.value.trim(),
            series: series.value.trim(),
            src: image.value.trim(),
            tier: tier.value,
            short: short.value.trim(),
            board_id: props.board
        },
        headers: {
            Authorization: props.board
        }
    })
    .then((_) => {
        props.changeDialog(DialogKind.None)
    })
}


</script>

<template>
    <input type="text" name="alt" placeholder="name of the character" v-model="name">
    <input type="text" name="series" placeholder="series" v-model="series">
    <input type="text" name="src" placeholder="image url" v-model="image">
    <label>
        <span class="tiers">
            <span class="SSS" @click="setTier">SSS</span>
            <span class="SS" @click="setTier">SS</span>
            <span class="S" @click="setTier">S</span>
            <span class="A" @click="setTier">A</span>
            <span class="B" @click="setTier">B</span>
            <span class="C" @click="setTier">C</span>
            <span class="D" @click="setTier">D</span>
            <span class="E" @click="setTier">E</span>
            <span class="F" @click="setTier">F</span>
        </span>
    </label>
    <textarea name="short" placeholder="Short reasoning" rows="20" cols="30" v-model="short" />
    <input type="button" value="Submit" @click=handleAdd>
</template>

<style scoped>
    .auth {
        width: 25%
    }
    .tiers {
        width: 100%;
        display: flex;
        justify-content: space-around;
        align-items: center;
    }
    .selected {
        outline: 3px wheat solid;
    }

    .tiers span {
        display: flex;
        align-items: center;
        justify-content: center;
        margin-top: 1vh;
        margin-bottom: 1vh;
        margin-left: 0.03vw;
        margin-right: 0.03vw;
        color: #423939;
        padding-top: 2px;
        padding-bottom: 2px;
        width: 33px;
        height: 33px;
        border-radius: 3px;
    }
.SSS {
    background: linear-gradient(-66.6deg, #ff7f7f 0%, red 100%);
}

.SS {
    background-color: #ff7f7f;
}

.S {
    background-color: #ffbf7f;
}

.A {
    background-color: #ffdf7f;
}

.B {
    background-color: #ffff7f;
}

.C {
    background-color: #bfff7f;
}

.D {
    background-color: #7fff7f;
}

.E {
    background-color: #7fffff;
}

.F {
    background-color: #7fbfff;
}
</style>