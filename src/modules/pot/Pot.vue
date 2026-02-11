<template>
    <div class="styler">
        <div class="top">
            <div class="players">
                <div class="player"></div>
                <div class="player"></div>
                <div class="player"></div>
                <div class="player"></div>
            </div>
            <img class="pot"
                src="https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fstatic.zerochan.net%2FPot.of.Greed.full.3789420.png"
                alt="Pot">
            <div class="options">
                <div class="option"></div>
                <div class="option"></div>
                <div class="option"></div>
                <div class="option"></div>
                <div class="option"></div>
                <div class="option"></div>
                <div class="option"></div>
                <div class="option"></div>
                <div class="option"></div>
                <div class="option"></div>
                <div class="option"></div>
                <div class="option"></div>
                <div class="option"></div>
                <div class="option"></div>
                <div class="option"></div>
                <div class="option"></div>
            </div>
        </div>
        <div class="cards">
            <div class="card A"></div>
            <div class="card B"></div>
            <div class="card C"></div>
        </div>
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
    margin-top: 1%;
}

.cards {
    margin-top: 3vh;
    width: 60vw;
    height: 45vh;
    display: flex;
    align-items: center;
    justify-content: space-around;
}

.card {
    width: 25%;
    height: 85%;
}

.players {
    width: 20vw;
    height: 40vh;
    overflow: hidden;
    overflow-y: scroll;
}

.player {
    width: 100%;
    height: 4vh;
}

.options {
    width: 20vw;
    height: 40vh;
    display: grid;
    grid-template-columns: auto auto auto auto;
    grid-template-rows: 25% 25% 25% 25%;
}

.option {
    width: 100%;
    height: 100%;
}

.pot {
    width: 18vw;
}

.cards,
.players,
.options,
.option,
.card,
.player {
    outline: 1px solid red;
}
</style>