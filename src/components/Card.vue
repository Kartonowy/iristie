<script setup lang="ts">
import { reactive, ref } from 'vue';

    const props = defineProps<{
        id: string,
        src: string,
        alt: string,
        short: string
    }>()

    const hover = ref(false);
    const mousePosition = reactive({
        x: 0,
        y: 0
    })

    const handleHover = (me: MouseEvent, ho: boolean) => {
            hover.value = ho
            mousePosition.x = me.pageX
            mousePosition.y = me.pageY
    }
</script>

<template>
    <!-- <div>{{ props.alt }}</div> -->
    <img :src="props.src" :alt="props.alt" @mousemove="(me) => handleHover(me, true)" @mouseleave="hover = false">
    <div v-if="hover" class="popup" :style="{'left':`${mousePosition.x}px`, 'top':`${mousePosition.y}px`}">
        <h3>{{ props.alt }}</h3>
        <p>{{ props.short }}</p>
    </div>
</template>

<style lang="css" scoped>
        img {
            object-fit: cover;
            object-position: center;
            width: 100px;
            height: 150px;
        }
        .popup {
            border-radius: 10px;
            background: #444444;
            position: absolute;
            transition: all;
            width: 20vw;
            display: flex;
            align-items: center;
            justify-content: flex-start;
            flex-flow: column;
        }
</style>