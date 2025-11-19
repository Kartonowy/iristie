import axios from "axios";
import type { CardType as Card }  from "./types";

const dummy_cards: Card[] = [
    {
        id: 1,
        src: "utils/reze.jpg",
        alt: "Reze-san",
        tier: "S",
        series: "Chainsaw Man",
        short: "Tragic really"
    },
    {
        id: 2,
        src: "utils/suzuka.jpg",
        alt: "Suzuka-sama",
        tier: "B",
        series: "Uma Musume",
        short: "Hey guys, did you know that in terms of male human and female Pokémon breeding, Vaporeon is the most compatible Pokémon for humans? Not only are they in the field egg group, which is mostly comprised of mammals, Vaporeon are an average of 3”03’ tall and 63.9 pounds, this means they’re large enough to be able handle human dicks, and with their impressive Base Stats for HP and access to Acid Armor, you can be rough with one. Due to their mostly water based biology, there’s no doubt in my mind that an aroused Vaporeon would be incredibly wet, so wet that you could easily have sex with one for hours without getting sore. They can also learn the moves Attract, Baby-Doll Eyes, Captivate, Charm, and Tail Whip, along with not having fur to hide nipples, so it’d be incredibly easy for one to get you in the mood. With their abilities Water Absorb and Hydration, they can easily recover from fatigue with enough water. No other Pokémon comes close to this level of compatibility. Also, fun fact, if you pull out enough, you can make your Vaporeon turn white. Vaporeon is literally built for human dick. Ungodly defense stat+high HP pool+Acid Armor means it can take cock all day, all shapes and sizes and still come for more"
    },
    {
        id: 3,
        src: "utils/freesia.jpg",
        alt: "Freesia",
        tier: "SS",
        series: "NieR:Automata",
        short: "Name given by Rose"
    },
    {
        id: 4,
        src: "utils/crowley.jpg",
        alt: "Crowley",
        tier: "A",
        series:"Good Omens",
        short: "For he does not crawl"
    }
];

export const _getCards: () => Promise<Card[]> = () => {
    return new Promise((resolve) => {
        return setTimeout(() => { resolve(dummy_cards); }, 1000)
    })
}
export const getCards = () => {
    return axios({
        method: "get",
        url: "/api/print"
    })
    .then((res) => {
        console.log(res.data)
        return res.data;
    })
}