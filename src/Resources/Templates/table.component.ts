import {Component, EventEmitter, Input, Output, PipeTransform} from '@angular/core';
import {Base} from '../Base';

@Component({
    selector: 'app-table',
    templateUrl: './table.html'
})
export class table extends Base {

    @Output() create = new EventEmitter();
    @Output() edit = new EventEmitter();
    @Output() delete = new EventEmitter();
    @Output() refresh = new EventEmitter();

    @Input() showColumns: boolean = true;

    @Input() showCreate: boolean;
    @Input() showEdit: boolean;
    @Input() showDelete: boolean;
    @Input() showRefresh: boolean;
    @Input() showType: boolean = true;

    @Output() editRow: EventEmitter<any> = new EventEmitter();

    @Input() title: string;

    @Input() columns: column[];
    @Input() rows: any[];
    @Input() displayType: displayType = displayType.list;

    distype: typeof displayType = displayType;
    coltype: typeof columnType = columnType;

    getvalue(rowvalue: string, formatting?: string, formatfunction?: Function) {
      rowvalue = this.Tools.linkText(rowvalue);
        if (formatfunction)
            rowvalue = formatfunction(rowvalue);
        if (formatting && rowvalue != undefined)
            return formatting.replace("{}", rowvalue);
        return rowvalue;
    }
}

export class column {
    Name: string;
    Type: columnType = columnType.text;
    Key: string;
    width?: number;
    maxwidth?: number;
    formatting?: string;
    formatvalue?(value: any): string;
    Reference?: column[];
    editable?: boolean;
    alt?: string;
}

export enum columnType {
    text,
    image,
    link,
    headertext
}

export enum displayType {
    list,
    card
}

export enum alignment {
    left = "left",
    center = "center",
    right = "right"
}
