import {AfterViewInit, Component} from '@angular/core';
import {ActivatedRoute} from '@angular/router';
import {Base} from "../../../Resources/Base";
import {TableClass} from "../../../Resources/Templates/TableClass";
import {TComic, TPerson} from "../../../../schema";
import {HTTPRequester} from "../../../Resources/HttpRequester";
import {HttpParams} from "@angular/common/http";
import {Tools} from "../../../Resources/Tools";

@Component({
  selector: 'app-comicPage',
  templateUrl: './comicPage.html'
})
export class ComicPage extends TableClass<TComic> {

  showAdd: boolean = false;

  pk: number;
  item: any = {};
  error: boolean;

  constructor(private route: ActivatedRoute) {
    super();
    route.params.subscribe(async x => {
      if (x["id"] > 0) {
        const result = (await HTTPRequester.Get("api/Comic/" + x["id"], new HttpParams().set("language", this.currentLanguage)))[0];
        if (result != null)
          this.currentItem = result;
        else
          this.error = true;
        console.log(this.currentItem);
      } else
        this.error = true;
      return;
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
