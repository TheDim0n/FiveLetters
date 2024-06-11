import { GameSession, LetterStatus } from "./types.d";


export const sessionCompleted: GameSession = {
    id: -1,
    target: "",
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
    id: -1,
    target: "",
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


export const sessionEmpty: GameSession = {
    id: -1,
    target: "",
    completed: false,
    current_attempt: 1,
    attemptions: [
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