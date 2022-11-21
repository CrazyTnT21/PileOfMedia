import { AfterViewInit, Component } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { Base } from 'src/app/mainapp/Base';
import { HTTPRequester } from 'src/Resources/other/HttpRequester';

@Component({
  selector: 'app-animeItem',
  templateUrl: './animeItem.component.html'
})
export class AnimeItemComponent extends Base implements AfterViewInit {


  pk: number = 1;
  item: any = {};
  constructor(private route: ActivatedRoute) {
    super();
  }
  async ngAfterViewInit() {
    this.route.queryParams
      .subscribe(params => {
        this.pk = params['pk'];
        this.loadItems();
      }
      );
  }
  async loadItems() {
    this.item = (await HTTPRequester.getItems("/Comic","TComic", "PK", [this.pk]))[0];
  }
}
