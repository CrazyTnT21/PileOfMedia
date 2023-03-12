import {Component} from '@angular/core';
import {ActivatedRoute} from '@angular/router';
import {TableSingle} from "../../../Resources/Templates/TableClass";
import {Comic} from "../../../../tables";

@Component({
  selector: 'app-comicPage',
  templateUrl: './comicPage.html'
})
export class ComicPage extends TableSingle<Comic> {
  public url: string = "api/comic/";

  constructor(private routea: ActivatedRoute) {
    super(routea);
  }
  public loadItems(): Promise<Comic[]> {
    throw new Error('Method not implemented.');
  }

  showAdd: boolean = false;
  createItem(): Comic {
    return new Comic();
  }
}
