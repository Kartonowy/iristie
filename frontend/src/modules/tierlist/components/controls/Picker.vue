<script setup lang="ts">
import { onMounted, ref, type Ref } from 'vue';
import { getBoards } from '../..//utils/get';
import { useRouter } from 'vue-router';

const router = useRouter()

const props = defineProps<{
    changeBoard: (id: number) => void,
    bel: number
}>();

type Board = {
    name: string,
    id: number
}

const boards: Ref<Board[], Board[]> = ref([])
const sel = ref(props.bel)

const handleChange = () => {
    props.changeBoard(sel.value)
    const new_url = router.currentRoute.value.fullPath.replace(/\d/g, sel.value.toString())
    router.replace(new_url)
}

onMounted(async () => {
    boards.value = await getBoards()
    props.changeBoard(sel.value)
})


</script>

<template>
    <select @change="handleChange" v-model="sel">
        <option v-for="board in boards" :value=board.id>{{ board.name }}</option>
    </select>
</template>

<style scoped>
</style>