import {AfterViewInit, Component} from '@angular/core';
import {ActivatedRoute} from '@angular/router';
import {Base} from "../../../Resources/Base";
import {TableClass} from "../../../Resources/Templates/TableClass";
import {TComic} from "../../../../schema";
import {HTTPRequester} from "../../../Resources/HttpRequester";
import {HttpParams} from "@angular/common/http";
import {Tools} from "../../../Resources/Tools";

@Component({
    selector: 'app-comicPage',
    templateUrl: './comicPage.html'
})
export class ComicPage extends TableClass<TComic> {

    pk: number;
    item: any = {};
    error: boolean;

    constructor(private route: ActivatedRoute) {
        super();
        route.params.subscribe(async x => {
            let testitem: TComic = new TComic();
            testitem.imageSource = "Assets/testimg.svg";
            testitem.name = "One Piece";
            testitem.description = "The One Piece is real!!!!";
            testitem.synopsis = "One Piece is about luffy, a boy who ate the gum gum fruit";
            testitem.status = "Ongoing";
            testitem.volumes = 101;
            testitem.chapters = 1057;
            testitem.averageScore = 7.15;
            testitem.pk = 1;
            testitem.publishStart = new Date("1997-11-1");
            testitem.publishEnd = new Date("2022-12-12");
            testitem.genres = ["Action","Adventure","Fantasy"];
            testitem.themes = ["Fantasy","Horror","Spooky"];
            testitem.creator = ["Eiichiro Oda"];
            this.currentItem = testitem;
            await HTTPRequester.Get("api/Comic", new HttpParams().set("language", this.currentLanguage).set("id", 2));

          if (x["id"] > 0)
              return;
          else{
            this.error = true;
          }
        })
    }


    createItem(): TComic {
        return new TComic();
    }

    deleteItem(item: TComic): any {
    }

    saveItem(item: TComic): any {
    }

    updateItem(item: TComic): any {
    }
}
