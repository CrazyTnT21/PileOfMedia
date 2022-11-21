import { Component, EventEmitter, Input, OnInit, Output } from '@angular/core';
import { Base } from 'src/app/mainapp/Base';
import { column, columnType } from 'src/Resources/templates/table.component';
@Component({
  selector: 'app-item',
  templateUrl: './item.component.html'
})
export class ItemComponent extends Base implements OnInit {

  @Output()
  @Output() create = new EventEmitter();
  @Output() edit = new EventEmitter();
  @Output() delete = new EventEmitter();
  @Output() refresh = new EventEmitter();
  @Input("title")
  title: string;

  @Input("columns")
  columns: column[];

  @Input("rows")
  rows: any[];

  coltype: typeof columnType = columnType;

  displaycreate: boolean = false;
  displayedit: boolean = false;
  displaydelete: boolean = false;

  getvalues(item: any) {
    return Object.values(item);
  }


  constructor() {
    super();
  }
  convertdate(date: Date): string {
    let tempdate: string = "";
    if (date.getDate().toString().length == 1)
      tempdate += "0";
    tempdate += date.getDate();
    tempdate += ".";
    if (date.getMonth().toString().length == 1)
      tempdate += "0";
    tempdate += date.getMonth();
    tempdate += ".";
    tempdate += date.getFullYear();
    return tempdate;
  }
  public getdays(startdate: Date, enddate: Date): number {
    let days = enddate.getTime() - startdate.getTime();
    return Math.round(days / 1000 / 60 / 60 / 24) + 1;
  }

  ngOnInit(): void {
  }

  public ApplyChanges(title: string): void {
    this.title = title;
  }
}
