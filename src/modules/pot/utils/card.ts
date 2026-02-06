enum Asset {
    Audio,
    Image
}

type PotCard = {
    defaultValue: number
    category: string
    question: string
    questionAsset: Asset
    answer: string
    answerAsset: Asset
}

type PotDeck = PotCard[]