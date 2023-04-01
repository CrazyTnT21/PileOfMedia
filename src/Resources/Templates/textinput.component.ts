import {Component, Input} from '@angular/core';

@Component({
  selector: 'text-input',
  template: `
    <div class="col-{{size}} lazy">
      <row>{{title}}</row>
      <input class="col-{{size}}">
    </div>
  `,
})
export class TextInputComponent {
  @Input() title: string = "";
  @Input() size: number = 4;

  constructor() {
  }
}


