import {Component, OnInit} from '@angular/core';
import {TableSingle} from "../../../Resources/Templates/TableClass";
import {TPerson} from "../../../../schema";
import {ActivatedRoute} from "@angular/router";

@Component({
  selector: 'app-profile',
  templateUrl: './profile.component.html'
})
export class ProfileComponent extends TableSingle<TPerson> {
  protected url: string = "api/profile/";

  constructor(private routea: ActivatedRoute) {
    super(routea);
  }

  pk: number = 1;
  comics: any[] = [];
  mangas: any[] = [];
  books: any[] = [];
  cartoons: any[] = [];
  tvshows: any[] = [];
  games: any[] = [];

  createItem(): TPerson {
    return new TPerson();
  }
}
