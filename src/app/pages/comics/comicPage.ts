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
  public showAdd: boolean = false;
  public currentPage: number = 0;

  constructor(private routea: ActivatedRoute) {
    super(routea);
  }

  advanceEdit() {
    this.currentPage++;
  }

  reverseEdit() {
    if (this.currentPage > 0)
      this.currentPage--;
  }

  createItem(): Comic {
    return new Comic();
  }
}
