import {Component, EventEmitter, Input, OnChanges, Output, SimpleChanges} from '@angular/core';
import {Base} from 'src/app/mainapp/Base';

@Component({
    selector: 'app-table',
    templateUrl: './table.html'
})
export class table extends Base {

    @Output() create = new EventEmitter();
    @Output() edit = new EventEmitter();
    @Output() delete = new EventEmitter();
    @Output() refresh = new EventEmitter();

    @Input() showCreate: boolean;
    @Input() showEdit: boolean;
    @Input() showDelete: boolean;
    @Input() showRefresh: boolean;
    @Input() showType: boolean = true;

    giveclass(width: number | undefined): string | null {
        if (width == null)
            return "col";
        return null;
    }
    @Output()
    CellClick: EventEmitter<any>;

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


}

export interface column {
    Name: string;
    Type: columnType;
    Key: string;
    width?: number;
    Reference?: string[];
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
