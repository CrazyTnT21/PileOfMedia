import {HttpClient} from '@angular/common/http';
import {Component} from '@angular/core';
import {Router} from '@angular/router';
import {HTTPRequester} from '../Resources/HttpRequester';
import {Page} from '../Resources/Templates/taskbar.component';
import {Tools} from '../Resources/Tools';
import {Base} from "../Resources/Base";

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent extends Base {

  constructor(http: HttpClient, router: Router) {
    super();
    Tools.router = router;
    HTTPRequester.start(http);
  }

  pages: Page[] =
    [
      {url: this.StringNames.Main},
      {url: this.StringNames.Manga},
      {
        url: this.StringNames.Comics,
        children:
          [{
            url: this.StringNames.Comic,
            query: {pk: 1}
          }]
      },
      {url: this.StringNames.Movies},
      {url: this.StringNames.Books},
      {url: this.StringNames.Anime},
      {url: this.StringNames.Profile},
      {url: this.StringNames.Albums},
      {url: this.StringNames.Editor},
    ]
}
