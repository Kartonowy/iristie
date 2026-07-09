<script setup lang="ts">
import { ref } from 'vue';

const carousel = ref("Homelabber")

const list = [
    "a Programmer",
    "a TCG Player",
    "an Anime Fan",
    "a Gamer",
    "a Stories Lover",
    "a Homelabber",
    "a Cosplayer"
]

let index = 0

setInterval(() => { 
    if (index > list.length - 1) {
        index = 0
    }
    carousel.value = list[index]!
    index++
}, 1000)

</script>

<template>
    <div class="sect">
        <h1>Hi there!  I'm hacunae.  <br>
             I'm {{ carousel }}. </h1>
        <p></p>
        <p>
            <h4>Links:</h4>
            <ul>
                <li><a href="https://github.com/Kartonowy">My GitHub</a></li>
                <li><a href="https://myanimelist.net/profile/NullZweiChan">My Anime</a></li>
                <li><a href="https://moxfield.com/users/2BLover">My MTG decks</a></li>
                <li><a href="https://myfigurecollection.net/profile/NullZweiChan">My Figures</a></li>
                <li><a href="https://discord.com/users/no2typeb">My Discord</a></li>
            </ul>
        </p>
    </div>
</template>

<style scoped>
h1 {
    text-align: center;
    width: 100%;
}

.sect {
    display: flex;
    flex-flow: row wrap;
    align-items: center;
}
ul {
    list-style:disc;
}
</style>