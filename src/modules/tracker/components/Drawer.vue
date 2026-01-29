<script setup lang="ts">
import { stuff } from '../utils/cards';

defineProps<{
    setCategory: (new_category: string) => void
}>();

</script>

<template>
  <div class="drawer">
        <div v-for="key in stuff" class="category-tile" :class="key.name" @click="setCategory(key.name)">
            <h2 class="title">
                {{ key.name }}
            </h2>
            <div class="target">
                {{ key.items.length }} / {{ key.target }}
            </div>
        </div>
  </div>
</template>

<style scoped>

.category-tile {
    width: 12vmax;
    height: 27vmin;
    display: flex;
    flex-flow: column wrap;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    border-radius: 15px;
}

.drawer {
    display: flex;
    flex-flow: row wrap;
    justify-content: space-evenly;
    padding-top: 7vh;
}

.Anime {
    background: linear-gradient(145deg,  #2e51a2 30% , #adadFf);
}

.Shows {
    background: linear-gradient(160deg, #e50914, #000000);
}

.Movies {
    background: linear-gradient(120deg, #0B1C2D, #4A0F1F);
}

.Books {
    /* background: linear-gradient(120deg, #E6D8B5, #2E221F); 
    alternative, more "paper to ink" less "gloom doom fantasy"
    */
    background: linear-gradient(120deg, #5A3A4E, #1B263B);
}

.Manga {
    background: linear-gradient(120deg, #f64368, #9ea55e);
}

.Games {
    background: linear-gradient(120deg, #00adee, #000000);
}

</style>
