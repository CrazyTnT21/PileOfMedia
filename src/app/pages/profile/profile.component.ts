import { AfterViewInit, Component, Input } from '@angular/core';
import { ActivatedRoute, Router } from '@angular/router';
import { Base } from 'src/app/mainapp/Base';
import { HTTPRequester } from 'src/Resources/other/HttpRequester';

@Component({
  selector: 'app-profile',
  templateUrl: './profile.component.html'
})
export class ProfileComponent extends Base implements AfterViewInit {

  @Input("profilepic")
  description: string = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et. ";
  profilepic: string = "https://cdn.myanimelist.net/images/userimages/13921943.jpg?t=1664021400";
  pk: number = 1;
  item: any = {};
  constructor(private route: ActivatedRoute, public router: Router) {
    super();
  }
  async ngAfterViewInit() {
    this.route.queryParams
      .subscribe(params => {

        // this.pk = params['pk'];

      }
      );
    await this.loadItems();
  }
  async loadItems() {

    this.item = (await HTTPRequester.getItems("","TUser", "PK", [this.pk]))[0];
    // this.item.person = (await HTTPRequester.getItems("TPerson", "PK", [this.item.FKPerson]))[0]
    this.item.comics = await HTTPRequester.getItems("","TUserXComic", "FKUser", [this.pk]);
    this.item.testcomics = [];
    for (let i = 0; i < this.item.comics.length; i++)
      this.item.testcomics.push((await HTTPRequester.getItems("/Comic","TComic", "PK", [this.item.comics[i].FKComic]))[0])
  }
}
