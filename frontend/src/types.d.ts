declare namespace Types{
    interface Joke{
        id: number;
        user_name: string;
        content: string;
        votes: number;
    }
}

export {};

declare global{
    var globalUsername : string;
}