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
        tier:
        <select name="tier" v-model="tier">
            <option value="SSS">SSS</option>
            <option value="SS">SS</option>
            <option value="S">S</option>
            <option value="A">A</option>
            <option value="B">B</option>
            <option value="C">C</option>
            <option value="D">D</option>
            <option value="E">E</option>
            <option value="F">F</option>
        </select>
    </label>
    <textarea name="short" placeholder="Short reasoning" rows="20" cols="30" v-model="short" />
    <input type="button" value="Submit" @click=handleAdd>
</template>

<style scoped>
    .auth {
        width: 25%
    }
</style>