export enum LetterStatus{
    Undefined,
    InUncorrectPosition,
    InCorrectPosition,
    NotFound
}
  
export interface Attemption {
    word: string,
    statuses: LetterStatus[]
}
  
  
export interface GameSession {
    target: string,
    attemptions: Attemption[],
    current_attempt: number
}