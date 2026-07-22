<script setup lang="ts">
import axios from 'axios';
import { onMounted, ref } from 'vue';
import type { CardType } from '../../utils/types';
import { DialogKind } from '../../utils/types';


const name = ref("")
const series = ref("")
const short = ref("")
const image = ref("")
const tier = ref("F")
const unsetfunc = ref(() => {})



const props = defineProps<{
    board: number
    ctxCard: CardType
    changeDialog: (dk: DialogKind) => void
}>();

name.value = props.ctxCard.alt
series.value = props.ctxCard.series
short.value = props.ctxCard.short
image.value = props.ctxCard.src
tier.value = props.ctxCard.tier

onMounted(() => {
    let el: HTMLElement = document.querySelector(".tiers ." + props.ctxCard.tier)!
    el.classList.add('selected')
    unsetfunc.value = () => { el.classList.remove('selected') }
})


const setTier = (e: any) => {
    const t = e.target?.className
    unsetfunc.value()
    tier.value = t
    e.target?.classList.add('selected')
    unsetfunc.value = () => {e.target?.classList.remove('selected')}
}

const handleEdit = () => {
    // const card: CardType = {
    //     alt: name.value,
    //     series: series.value,
    //     src: image.value,
    //     tier: tier.value,
    //     short: short.value
    // }
    if (
        name.value == props.ctxCard.alt &&
        series.value == props.ctxCard.series &&
        short.value == props.ctxCard.short &&
        image.value == props.ctxCard.src &&
        tier.value == props.ctxCard.tier
    ) {
        props.changeDialog(DialogKind.None)
        return
    }
    axios({
        method: "post",
        url: "/api/update",
        data: {
            alt: name.value,
            series: series.value,
            src: image.value,
            tier: tier.value,
            short: short.value,
            board_id: props.board
        },
        headers: {
            Authorization: props.board
        },
    })
    .then((_res) => {
        props.changeDialog(DialogKind.None)
    })
}


</script>

<template>
    <input type="text" name="alt" placeholder="name of the card" v-model="name" >
    <input type="text" name="series" placeholder="category" v-model="series" >
    <input type="text" name="src" placeholder="image url" v-model="image" >
<!--  TODO: replace image url with uploads to the server    -->
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
    <input type="button" value="Submit" @click=handleEdit>
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