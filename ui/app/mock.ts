import { GameSession, LetterStatus } from "./types.d";


export const sessionCompleted: GameSession = {
    completed: true,
    current_attempt: 5,
    attemptions: [
        {
            word: "палец",
            statuses: [LetterStatus.NotFound, LetterStatus.InUncorrectPosition, LetterStatus.NotFound, LetterStatus.InUncorrectPosition, LetterStatus.NotFound],
        },
        {
            word: "веган",
            statuses: [LetterStatus.NotFound, LetterStatus.InCorrectPosition, LetterStatus.NotFound, LetterStatus.InUncorrectPosition, LetterStatus.NotFound],
        },
        {
            word: "речка",
            statuses: [LetterStatus.InCorrectPosition, LetterStatus.InCorrectPosition, LetterStatus.NotFound, LetterStatus.InCorrectPosition, LetterStatus.InCorrectPosition],
        },
        {
            word: "решка",
            statuses: [LetterStatus.InCorrectPosition, LetterStatus.InCorrectPosition, LetterStatus.NotFound, LetterStatus.InCorrectPosition, LetterStatus.InCorrectPosition],
        },
        {
            word: "рейка",
            statuses: [LetterStatus.InCorrectPosition, LetterStatus.InCorrectPosition, LetterStatus.InCorrectPosition, LetterStatus.InCorrectPosition, LetterStatus.InCorrectPosition],
        },
        {
            word: "",
            statuses: [LetterStatus.Undefined, LetterStatus.Undefined, LetterStatus.Undefined, LetterStatus.Undefined, LetterStatus.Undefined],
        },
    ]
}


export const sessionActive: GameSession = {
    completed: false,
    current_attempt: 3,
    attemptions: [
        {
            word: "палец",
            statuses: [LetterStatus.NotFound, LetterStatus.InUncorrectPosition, LetterStatus.NotFound, LetterStatus.InUncorrectPosition, LetterStatus.NotFound],
        },
        {
            word: "веган",
            statuses: [LetterStatus.NotFound, LetterStatus.InCorrectPosition, LetterStatus.NotFound, LetterStatus.InUncorrectPosition, LetterStatus.NotFound],
        },
        {
            word: "",
            statuses: [LetterStatus.Undefined, LetterStatus.Undefined, LetterStatus.Undefined, LetterStatus.Undefined, LetterStatus.Undefined],
        },
        {
            word: "",
            statuses: [LetterStatus.Undefined, LetterStatus.Undefined, LetterStatus.Undefined, LetterStatus.Undefined, LetterStatus.Undefined],
        },
        {
            word: "",
            statuses: [LetterStatus.Undefined, LetterStatus.Undefined, LetterStatus.Undefined, LetterStatus.Undefined, LetterStatus.Undefined],
        },
        {
            word: "",
            statuses: [LetterStatus.Undefined, LetterStatus.Undefined, LetterStatus.Undefined, LetterStatus.Undefined, LetterStatus.Undefined],
        },
    ]
}