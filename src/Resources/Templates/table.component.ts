import {Component, EventEmitter, Input, Output, PipeTransform} from '@angular/core';
import {Base} from '../Base';
import {MainComponent} from "../../app/pages/main/main.component";

@Component({
  selector: 'app-table',
  templateUrl: './table.html'
})
export class table extends Base {
  @Input() clickUrl: string = "";
  @Input() showColumns: boolean = true;

  @Input() showCreate: boolean;
  @Input() showEdit: boolean;
  @Input() showDelete: boolean;

  @Output() editRow: EventEmitter<any> = new EventEmitter();

  @Input() title: string;

  @Input() columns: column[];
  @Input() rows: any[];
  @Input() DisplayType: DisplayType = MainComponent.config.view;

  distype: typeof DisplayType = DisplayType;
  coltype: typeof ColumnType = ColumnType;

  getvalue(rowvalue: string, formatting?: string, formatfunction?: Function) {
   // rowvalue = this.Tools.linkText(rowvalue);
    if (formatfunction)
      rowvalue = formatfunction(rowvalue);
    if (formatting && rowvalue != undefined)
      return formatting.replace("{}", rowvalue);
    return rowvalue;
  }
}

export class column {
  Name: string;
  Type: ColumnType = ColumnType.text;
  Key: string;
  width?: number;
  maxwidth?: number;
  formatting?: string;

  formatvalue?(value: any): string;

  Reference?: column[];
  editable?: boolean;
  alt?: string;
}

export enum ColumnType {
  text,
  image,
  link,
  headertext
}

export enum DisplayType {
  list,
  card
}

export enum Alignment {
  left = "left",
  center = "center",
  right = "right"
}
