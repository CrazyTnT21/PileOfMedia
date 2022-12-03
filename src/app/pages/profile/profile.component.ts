import {Component, OnInit} from '@angular/core';
import {HTTPRequester} from '../../../Resources/HttpRequester';
import {HttpParams} from "@angular/common/http";
import {TableClass} from "../../../Resources/Templates/TableClass";

@Component({
    selector: 'app-profile',
    templateUrl: './profile.component.html'
})
export class ProfileComponent extends TableClass<any> implements OnInit {

    pk: number = 1;
    comics: any[] = [];
    mangas: any[] = [];
    books: any[] = [];
    cartoons: any[] = [];
    tvshows: any[] = [];
    games: any[] = [];

    async ngOnInit() {
        await this.loadItems();
    }

    async loadItems() {
        const result = await HTTPRequester.Get("api/user", new HttpParams().set("where", this.pk));
        this.currentItem = result.user[0];
        this.comics = result.comics;
    }

    createItem(): any {
    }

    deleteItem(item: any): any {
    }

    saveItem(item: any): any {
    }

    updateItem(item: any): any {
    }
}
