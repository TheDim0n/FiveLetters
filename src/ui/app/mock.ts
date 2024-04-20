import { GameSession, LetterStatus } from "./types.d";


export const sessionCompleted: GameSession = {
    target: "рейка",
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