<template>
    <div class="styler">
        <div class="top">
            <div class="players"></div>
            <img class="pot"
                src="https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fstatic.zerochan.net%2FPot.of.Greed.full.3789420.png"
                alt="Pot">
            <div class="options"></div>
        </div>
        <div class="cards"></div>
    </div>
</template>

<style scoped>
.styler {
    display: flex;
    flex-flow: column wrap;
    align-items: center;
    width: 100vw;
}

.top {
    display: flex;
    flex-flow: row nowrap;
    justify-content: space-around;
    width: 100vw;
    margin: 2vmin;
}

.cards {
    width: 60vw;
    height: 50vh;
}

.players {
    width: 20vw;
}

.options {
    width: 20vw;
}

.pot {
    width: 18vw;
}

.cards,
.players,
.options {
    border: 1px solid red;
}
</style>