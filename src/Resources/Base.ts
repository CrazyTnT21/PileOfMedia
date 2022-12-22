import * as StringNames from "../Resources/StringNames.json";
import {Tools} from "./Tools";

export class Base {
    public StringNames = StringNames;
    public Tools = Tools;
    private static _Languages: any[] = [
        {PK: 1, Language: "English", LanguageColumn: "English"},
        {PK: 2, Language: "German"},
        {PK: 3, Language: "Spanish"},
        {PK: 4, Language: "Japanese"},
        {PK: 5, Language: "Italian"}
    ];
    public get Languages(): any[] {
        return Base._Languages ? Base._Languages : this.setLanguages();
    }

    private static _Genres: any[];
    private static _Statuses: any[];
    private static _Themes: any[];

    private setLanguages() {
        //   Base._Languages = [];
        //  Base._Languages = await HTTPRequester.Get("api/language", new HttpParams());
        return Base._Languages;
    }
}