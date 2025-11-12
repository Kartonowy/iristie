<script setup lang="ts">
import { reactive, ref, useTemplateRef } from 'vue';

const props = defineProps<{
    id: string,
    src: string,
    alt: string,
    short: string,
    series: string
}>()

const popup = useTemplateRef("popup")

const hover = ref(false);
const mousePosition = reactive({
    x: 0,
    y: 0
})
// TODO: ADJUST ON RESIZE

const handleHover = (me: MouseEvent, ho: boolean) => {
    hover.value = ho
    const w = popup.value?.offsetWidth!
    const h = popup.value?.offsetWidth!
    mousePosition.y = me.pageY
    if (me.pageX + w >= window.innerWidth) {
        mousePosition.x = me.pageX - w
    } else {
        mousePosition.x = me.pageX
    }
    // TODO: make it not clip outside the page
    // so check if x/y + size is greater than winsize etc
}
</script>

<template>
    <!-- <div>{{ props.alt }}</div> -->
    <img :src="props.src" :alt="props.alt" @mousemove.passive="(me) => handleHover(me, true)" @mouseleave="hover = false">
    <div v-if="hover" ref="popup" class="popup"
     :style="{'left':`${mousePosition.x}px`, 'top':`${mousePosition.y}px`}"
     >
        <h3>{{ props.alt }}<span>{{ props.series }}</span></h3>
        <!-- <p>{{ props.short }}</p> -->
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
            min-width: 20vw;
            max-width: 60vw;
            display: flex;
            align-items: center;
            justify-content: flex-start;
            flex-flow: column;
            line-height: 1;
            padding: 2vmin;
        }
        .popup * {
            margin: 1vmin;
        }
        .popup p {
            line-height: 1.1;
        }
        h3 span {
            all: unset;
            font-weight: 300;
            color: #888888
        }
</style>