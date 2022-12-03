import {Component, EventEmitter, Input, OnChanges, Output, PipeTransform, SimpleChanges} from '@angular/core';
import {Base} from '../../Resources/Base';

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

    @Output()
    editRow: EventEmitter<any> = new EventEmitter();

    @Input()
    title: string;

    @Input()
    columns: column[];

    @Input()
    rows: any[];

    @Input()
    displayType: displayType = displayType.list;
    distype: typeof displayType = displayType;
    coltype: typeof columnType = columnType;

    editrow(value: any) {
        this.editRow.emit(value);
    }

    getvalue(rowvalue: string, formatting?: string, formatfunction?: Function) {
        if (formatfunction)
            rowvalue = formatfunction(rowvalue);
        if (formatting && rowvalue != undefined)
            return formatting.replace("{}", rowvalue);
        return rowvalue;

    }

    giveclass(width: number | undefined): string | null {
        if (width)
            return "fix-" + width;
        else
            return "col";
    }
}

export interface column {
    Name: string;
    Type: columnType;
    Key: string;
    width?: number;
    formatting?: string;
    formatvalue?(value: any): string;
    pipes?: PipeTransform[];
    Reference?: column[];
    alignment?: alignment;
    editable?: boolean;
    alt?: string;

    formatFunction?(value: any): any;
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
