import {Component} from '@angular/core';
import {TableSingle} from "../../../Resources/Templates/TableClass";
import {ActivatedRoute} from "@angular/router";
import {HttpParams} from "@angular/common/http";
import {User} from "../../../../tables";

@Component({
  selector: 'app-profile',
  templateUrl: './profile.component.html'
})
export class ProfileComponent extends TableSingle<User> {
  protected url: string = "api/user/";
  protected override loadParams: HttpParams = new HttpParams();

  constructor(private routea: ActivatedRoute) {
    super(routea);
  }

  comics: any[] = [];
  mangas: any[] = [];
  books: any[] = [];
  cartoons: any[] = [];
  tvshows: any[] = [];
  games: any[] = [];

  createItem(): User {
    return new User();
  }
}
