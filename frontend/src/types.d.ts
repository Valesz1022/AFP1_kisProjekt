declare namespace Types {
    interface Joke {
        id: number;
        user_name: string;
        content: string;
        votes: number;
    }
    
    interface Vote{
        vote: number;
    }
}