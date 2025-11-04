export type CardType  = {
    id: string, // ID of the character for indexing purpose
    src: string, // SRC meaning url to the image of the character
    alt: string, // ALT always consists of character's name
    tier: string, // TIER for tier alignment purposes
    short: string // SHORT for short description of the character, reasoning behind its placement
    // TODO: source of the image?
    // TODO: source of the character (like form what game show anime manga it is)
}