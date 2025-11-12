export type CardType  = {
    id: number, // ID of the character for indexing purpose
    src: string, // SRC meaning url to the image of the character
    alt: string, // ALT always consists of character's name
    series: string, // SERIES for where the character comes from
    tier: string, // TIER for tier alignment purposes
    short: string // SHORT for short description of the character, reasoning behind its placement
    // TODO: source of the image?
}