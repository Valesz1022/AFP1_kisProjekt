
export {};

export namespace Types{
    interface Joke{
        id: number;
        user_name: string;
        content: string;
        votes: number;
    }
}


declare global{
    var globalUsername : string;
}