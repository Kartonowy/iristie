export const stuff: {
    anime: category,
    shows: category,
    movies: category,
    books: category,
    manga: category,
    games: category,
} = {
    anime: {
        name: "Anime",
        target: 80,
        items: [
            {
                title: "Fate/stay night: Unlimited Blade Works 2nd Season",
                cover: "https://cdn.myanimelist.net/images/anime/1881/124810.jpg",
                date: new Date("2026-01-03")
            },
            {
                title: "Yahari Ore no Seishun Love Comedy wa Machigatteiru.",
                cover: "https://cdn.myanimelist.net/images/anime/1786/120117.jpg",
                date: new Date("2026-01-11")
            },
            {
                title: "Yahari Ore no Seishun Love Comedy wa Machigatteiru. OVA",
                cover: "https://cdn.myanimelist.net/images/anime/9/54831.jpg",
                date: new Date("2026-01-11")
            },
            {
                title: "Fate/stay night: Unlimited Blade Works 2nd Season",
                cover: "https://cdn.myanimelist.net/images/anime/1881/124810.jpg",
                date: new Date("2026-01-03")
            },
            {
                title: "Yahari Ore no Seishun Love Comedy wa Machigatteiru.",
                cover: "https://cdn.myanimelist.net/images/anime/1786/120117.jpg",
                date: new Date("2026-01-11")
            },
            {
                title: "Yahari Ore no Seishun Love Comedy wa Machigatteiru. OVA",
                cover: "https://cdn.myanimelist.net/images/anime/9/54831.jpg",
                date: new Date("2026-01-11")
            },
            {
                title: "Fate/stay night: Unlimited Blade Works 2nd Season",
                cover: "https://cdn.myanimelist.net/images/anime/1881/124810.jpg",
                date: new Date("2026-01-03")
            },
            {
                title: "Yahari Ore no Seishun Love Comedy wa Machigatteiru.",
                cover: "https://cdn.myanimelist.net/images/anime/1786/120117.jpg",
                date: new Date("2026-01-11")
            },
            {
                title: "Yahari Ore no Seishun Love Comedy wa Machigatteiru. OVA",
                cover: "https://cdn.myanimelist.net/images/anime/9/54831.jpg",
                date: new Date("2026-01-11")
            },
            {
                title: "Fate/stay night: Unlimited Blade Works 2nd Season",
                cover: "https://cdn.myanimelist.net/images/anime/1881/124810.jpg",
                date: new Date("2026-01-03")
            },
            {
                title: "Yahari Ore no Seishun Love Comedy wa Machigatteiru.",
                cover: "https://cdn.myanimelist.net/images/anime/1786/120117.jpg",
                date: new Date("2026-01-11")
            },
            {
                title: "Yahari Ore no Seishun Love Comedy wa Machigatteiru. OVA",
                cover: "https://cdn.myanimelist.net/images/anime/9/54831.jpg",
                date: new Date("2026-01-11")
            },
            {
                title: "Fate/stay night: Unlimited Blade Works 2nd Season",
                cover: "https://cdn.myanimelist.net/images/anime/1881/124810.jpg",
                date: new Date("2026-01-03")
            },
            {
                title: "Yahari Ore no Seishun Love Comedy wa Machigatteiru.",
                cover: "https://cdn.myanimelist.net/images/anime/1786/120117.jpg",
                date: new Date("2026-01-11")
            },
            {
                title: "Yahari Ore no Seishun Love Comedy wa Machigatteiru. OVA",
                cover: "https://cdn.myanimelist.net/images/anime/9/54831.jpg",
                date: new Date("2026-01-11")
            },
            {
                title: "Fate/stay night: Unlimited Blade Works 2nd Season",
                cover: "https://cdn.myanimelist.net/images/anime/1881/124810.jpg",
                date: new Date("2026-01-03")
            },
            {
                title: "Yahari Ore no Seishun Love Comedy wa Machigatteiru.",
                cover: "https://cdn.myanimelist.net/images/anime/1786/120117.jpg",
                date: new Date("2026-01-11")
            },
            {
                title: "Yahari Ore no Seishun Love Comedy wa Machigatteiru. OVA",
                cover: "https://cdn.myanimelist.net/images/anime/9/54831.jpg",
                date: new Date("2026-01-11")
            },
        ]
    },
shows: {
    name: "Shows",
    target: 15,
    items: []
},
movies: {
    name: "Movies",
    target: 20,
    items: []
},
books: {
    name: "Books",
    target: 8,
    items: []
},
manga: {
    name: "Manga",
    target: 15,
    items: []
},
games: {
    name: "Games",
    target: 25,
    items: []
}
}

export type item = {
    title: string,
    cover: string,
    date: Date
}

type category = {
    name: string,
    target: number,
    items: item[]
}
