import { HttpClient } from '@angular/common/http';
import { Component } from '@angular/core';
import { Router } from '@angular/router';
import { HTTPRequester } from 'src/Resources/other/HttpRequester';
import * as StringNames from "src/Resources/other/StringNames.json";
import { Page } from 'src/Resources/templates/taskbar.component';
import { Tools } from '../tools/Tools';
@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent {
  stringnames = StringNames;
  title: string = "MyCollection"
  currentuserPK = 1;
  constructor(http: HttpClient, router: Router) {
    Tools.router = router;
    HTTPRequester.start(http);
  }
  pages: Page[] =
    [
      { url: this.stringnames.Main },
      { url: this.stringnames.Manga },
      {
        url: this.stringnames.Comics,
        children: [
          {
            url: this.stringnames.Comic,
            query: { pk: 1 }
          }
        ]

      },
      { url: this.stringnames.Movies },
      { url: this.stringnames.Books },
      { url: this.stringnames.Anime },
      { url: this.stringnames.Profile },
      { url: this.stringnames.Albums },
      { url: this.stringnames.Editor },
    ]
}