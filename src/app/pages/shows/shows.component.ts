import { Component, OnInit } from '@angular/core';
import { Base } from 'src/app/mainapp/Base';

@Component({
  selector: 'app-shows',
  templateUrl: './shows.component.html'
})
export class ShowsComponent extends Base implements OnInit {

  constructor() {
    super();
  }

  ngOnInit(): void {
  }

}
