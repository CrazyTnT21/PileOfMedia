//Non-existent fields - may only exist during sending or in the client
//Client-Only fields - only exists in the client


export class TComic {
    PK?: number;
    FKName: number;
    FKDescription?: number;
    FKSynopsis?: number;
    Chapters?: number;
    Volumes?: number;
    PublishStart?: Date;
    PublishEnd?: Date;
    ImageSource?: string;
    //Client-Only fields
    Name?: string;
    Description?: string;
    Status?: string;
    Synopsis?: string;
    //Non-existent fields
    LanguageFields?: LanguageField[];
}

export interface LanguageField {
    values: translation[];
    column: string;
}

export interface translation {
    language: string;
    value: string;
}
export class Languages{
    public static readonly English: string = "English";
    public static readonly German: string = "German";
    public static readonly Spanish: string = "Spanish";
    public static readonly Japanese: string = "Japanese";
    public static readonly Italian: string = "Italian";
    public static readonly Korean: string = "Korean";
    public static readonly French: string = "French";
}