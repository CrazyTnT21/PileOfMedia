import { AfterViewInit, Component } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import {Base} from "../../../Resources/Base";
@Component({
  selector: 'app-comicPage',
  templateUrl: './comicPage.html'
})
export class ComicPage extends Base implements AfterViewInit {

  pk: number;
  item: any = {};
  constructor(private route: ActivatedRoute) {
    super();
  }
  async ngAfterViewInit() {
    // console.log(this.route);
    // this.route.queryParams
    //   .subscribe(params => {
    //     this.pk = params['pk'];
    //     this.loadItems();
    //   }
    //   );
  }
  async loadItems() {
   // this.item = (await HTTPRequester.getItems("/Comic","TComic", "PK", [this.pk]))[0];
    // const status = (await HTTPRequester.getItems("","TStatus", "id", [this.item.FKStatus]))[0];
   // this.item.status = status.IStatus;
  }
}
