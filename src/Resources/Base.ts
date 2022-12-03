import * as StringNames from "../Resources/StringNames.json";
import {Tools} from "../Resources/Tools";
import {HTTPRequester} from "./HttpRequester";
import {HttpParams} from "@angular/common/http";

export class Base {
    public static currentLanguage: number = 0;
    public StringNames = StringNames;
    public Tools = Tools;
    private static _Languages: any[] = ["English","German","Spanish","Japanese","Italian"];

    public get Languages(): any[] {
        return Base._Genres ? Base._Genres : this.setLanguages();
    }
    private static _Genres: any[];
    public get Genres(): Promise<any[]> | any[] {
        return Base._Genres ? Base._Genres : this.setGenres();
    }

    private static _Statuses: any[];
    public get Statuses(): Promise<any> | any[] {
        return Base._Statuses ? Base._Statuses : this.setStatuses();
    }

    private static _Themes: any[];
    public get Themes(): Promise<any[]> | any[] {
        return Base._Themes ? Base._Themes : this.setThemes();
    }

    private async setGenres() {
        Base._Genres = [];
        Base._Genres = await HTTPRequester.Get("api/genre", new HttpParams());
        return Base._Genres;
    }

    private async setStatuses() {
        Base._Statuses = [];
        Base._Statuses = await HTTPRequester.Get("api/statuses", new HttpParams());
        return Base._Genres;
    }

    private async setThemes() {
        Base._Themes = [];
        Base._Themes = await HTTPRequester.Get("api/themes", new HttpParams());
        return Base._Genres;
    }
    private setLanguages() {
     //   Base._Languages = [];
      //  Base._Languages = await HTTPRequester.Get("api/language", new HttpParams());
        return Base._Languages;
    }
}