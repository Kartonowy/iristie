<script setup lang="ts">


const props = defineProps<{
    close: () => void
}>();

</script>

<template>
    <div class="helptext">
        <span>
            Characters are rated on basis of 4 categories: SATISFACTION, LORE, DESIGN, ROLE.
        </span>
        <p>
            <span>Character onscreen viewer satisfaction</span>
            A scale which ranks how satisfying it is to see a character on screen,
            see a mention of them or see results of their action. This does not
            take into account how the story revolves around them, or how impactful
            they are. Just the viewer satisfaction.
        </p>
        <p>
            <span>Character's Lore AW</span>
            How many plotholes, does this character have. Does the story make sense?
            Do their story justify their actions, explain them or improve upon them.
            How interesting the story is, how captivating and how much fitting into
            the world it is.
        </p>
        <p>
            <span>Character's design</span>
            How the character acts, how do they talk, how do they look. Is it innovative?
            Is it easily recognizable? Or is the character easily forgotten. Does
            the character fit into the world? Is it novel?
        </p>
        <p>
            <span>Character's role</span>
            How well does character fulfill their role in work. Does the character portray
            a message? Is it an archetype; if so how well written? Is the character just a
            filler?
        </p>
    </div>
    <div class="controls">
        <h6 @click="close">close</h6>
    </div>
</template>

<style scoped>
    .helptext {
        display: flex;
        flex-flow: column;
        align-items: center;
        font-style: normal;
    }
    p {
        all: unset;
        font-size: 0.7em;
        text-align: center;
        display: flex;
        flex-flow: column;
        line-height: 1.3em;
        color: #999999;
    }
    span {
        color: #ffffff;
        font-size: 1.2rem;
        text-align: center;
        line-height: 2em;
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