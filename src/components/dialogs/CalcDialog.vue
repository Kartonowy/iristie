<script setup lang="ts">
import { ref } from 'vue';


const satisfaction = ref(0);
const lore = ref(0);
const design = ref(0);
const role = ref(0);

const props = defineProps<{
    close: () => void
}>();

const handleCalc = () => {
    const sum = (Number(satisfaction.value) + Number(lore.value) + Number(design.value) + Number(role.value)) / 4;
    let tier;

    if (sum >= 9.5) {
        tier = "SSS"
    } else if (sum >= 9) {
        tier = "SS"
    }else if (sum >= 8) {
        tier = "S"
    }else if (sum >= 7) {
        tier = "A"
    }else if (sum >= 5.5) {
        tier = "B"
    }else if (sum >= 4) {
        tier = "C"
    }else if (sum >= 3) {
        tier = "D"
    }else if (sum >= 2) {
        tier = "E"
    }else  {
        tier = "F"
    }

    alert(`Avg is ${sum}, that would add up to ${tier}`);
}

</script>

<template>
    <div class="calc">
        <label for="satisfaction">
            Satisfaction: {{ satisfaction }}
            <input type="range" name="satisfaction" id="" max="10" v-model="satisfaction">
        </label>
        <label for="satisfaction">
            Lore: {{ lore }}
            <input type="range" name="lore" id="" max="10" v-model="lore" >
        </label>
        <label for="satisfaction">
            Design: {{ design }}
            <input type="range" name="design" id="" max="10" v-model="design" >
        </label>
        <label for="satisfaction">
            Role: {{ role }}
            <input type="range" name="role" id="" max="10" v-model="role">
        </label>
    </div>
    <div class="controls">
        <h6 @click="handleCalc">calc</h6>
        <h6 @click="close">close</h6>
    </div>
</template>

<style scoped>
label {
    display: flex;
    flex-flow: column;
    width: 30vmin;
    padding-left: 2vw;
    padding-right: 2vw;
}
 .controls {
        display: flex;
    }

    .controls h6 {
        padding-left: 2vmin;
        padding-right: 2vmin;
        margin: 0;
    }
</style>