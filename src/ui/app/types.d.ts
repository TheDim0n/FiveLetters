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
    attemptions: Attemption[],
    current_attempt: number,
    completed: boolean
}