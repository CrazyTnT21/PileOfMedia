import {Component, Input} from '@angular/core';

@Component({
  selector: 'textArea-input',
  template: `
    <div class="col-{{size}}">
      <row>{{title}}</row>
      <textarea [rows]="rows" class="col-{{size}}"></textarea>
    </div>
  `,
})
export class TextAreaInputComponent {
  @Input() title: string = "";
  @Input() size: number = 4;
  @Input() rows: number = 4;

  constructor() {
  }
}


