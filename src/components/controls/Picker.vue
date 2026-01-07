<script setup lang="ts">
import { onMounted, ref, type Ref } from 'vue';
import { getBoards } from '../../utils/get';

const props = defineProps<{
    changeBoard: (id: number) => void
}>();

type Board = {
    name: string,
    id: number
}

const boards: Ref<Board[], Board[]> = ref([])
const sel = ref(0)

const handleChange = () => {
    props.changeBoard(sel.value)
}

onMounted(async () => {
    boards.value = await getBoards()
    await cookieStore.get("Board")
    .then((e) => {
        let num = Number(e?.value)
        props.changeBoard(num)
        sel.value = num
    })
    .catch(() => {
        props.changeBoard(1)
        sel.value = 1
    })
})


</script>

<template>
    <select @change="handleChange" v-model="sel">
        <option v-for="board in boards" :value=board.id>{{ board.name }}</option>
    </select>
</template>

<style scoped>
</style>