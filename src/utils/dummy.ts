import type { CardType as Card }  from "./types";

const dummy_cards: Card[] = [
    {
        id: "1",
        src: "utils/reze.jpg",
        alt: "Reze-san",
        tier: "S",
        short: "Chainsawman character"
    },
    {
        id: "2",
        src: "utils/suzuka.jpg",
        alt: "Suzuka-sama",
        tier: "B",
        short: "Uma"
    },
    {
        id: "3",
        src: "utils/freesia.jpg",
        alt: "Freesia",
        tier: "SS",
        short: "Automata"
    },
    {
        id: "4",
        src: "utils/crowley.jpg",
        alt: "Crowley",
        tier: "A",
        short: "Good Omens"
    }
]

export const getCards: () => Promise<Card[]> = () => {
    return new Promise((resolve) => {
        return setTimeout(() => { resolve(dummy_cards); }, 1000)
    })
}