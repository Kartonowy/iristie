<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { getTrivia } from '../utils/getTrivia';

const carousel = ref("Homelabber")
const trivia = ref("")

const updateTrivia = async () => {
    trivia.value = (await getTrivia()).text
}


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

onMounted(updateTrivia)

</script>

<template>
    <main>
        <h1>Hi there!  I'm hacunae.  <br> I'm {{ carousel }}. </h1>

        <section>
            <h2>Trivia of the day</h2>
            <p>{{ trivia }}</p>
        </section>

        <section>
            <h2>What is this site?</h2>
            <p>This site is just a personal hobby of mine, both maintaining and devoloping it.
                You can find various things here, like trivia, ways to contact me or some fun stuff like my tierlists
                or Tracker of what I watched or played so far (I like collecting data).
            </p>
        </section>

        <section>
            <h2>Currently I'm working on:</h2>
            <p>
                <ul>
                    <li>Creating new sections for this website</li>
                    <li>Crafting a discord helper bot</li>
                    <li>Catching up with Bleach</li>
                    <li>Reading through ORV</li>
                    <li>New cosplay for Hikari convention</li>
                </ul>
            </p>
        </section>

        
        <section>
            <h2>Also check out</h2>
            <ul>
                <li><a href="https://github.com/Kartonowy">My GitHub</a></li>
                <li><a href="https://steamcommunity.com/id/zerothebitch">My Steam</a></li>
                <li><a href="https://myanimelist.net/profile/NullZweiChan">My Anime List</a></li>
                <li><a href="https://letterboxd.com/nullzweichan/">My Movies</a></li>
                <li><a href="https://www.goodreads.com/user/show/148668363-karton">My Books</a></li>
                <li><a href="https://moxfield.com/users/2BLover">My MTG decks</a></li>
                <li><a href="https://myfigurecollection.net/profile/NullZweiChan">My Figure Collection</a></li>
                <li><a href="https://discord.com/users/no2typeb">My Discord</a></li>
            </ul>
        </section>
    </main>
</template>

<style scoped>
h1 {
    text-align: center;
    width: 100%;
}


ul {
    list-style:none;
    line-height: 1.5rem;
    display: flex;
    flex-flow: wrap column;
    height: 60%;
}

a {
    color: white;
}

@media screen and (min-width: 1401px) {
    main {
        width: 100%;
        display: flex;
        flex-flow: row wrap;
        align-items: center;
        justify-content: center;
    }

    section {
        background-color: #383838;
        height: 23vh;
        width: 43%;
        border-radius: 15px;
        margin: 1vw;
        padding: 2vw;
    }
}
@media screen and (max-width: 1400px) {
    main {
        width: 100%;
        display: flex;
        flex-flow: column wrap;
        align-items: center;
        justify-content: center;
    }

    section {
        background-color: #383838;
        height: 23vh;
        width: 90%;
        border-radius: 15px;
        margin: 1vw;
        padding: 2vw;
    }
}
</style>