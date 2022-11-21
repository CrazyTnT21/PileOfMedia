import { Component, EventEmitter, Input, OnChanges, Output, SimpleChanges } from '@angular/core';
import { Base } from 'src/app/mainapp/Base';
@Component({
    selector: 'app-table',
    templateUrl: './table.html'
})
export class table extends Base implements OnChanges {
    ngOnChanges(changes: SimpleChanges): void {
        this.convertitems();
    }
    iindex: number = 0;
    jindex: number = 0;
    giveclass(width: number | undefined): string | null {
        if (width == null)
            return "col un";
        return null;
    }
    rowEntries: any[][] = [];
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
    getreference(ref: string, j: number): any {
        if (ref != null) {
            const entry = Object.entries(this.rows[j]);
            for (let i = 0; i < entry.length; i++)
                if (entry[i][0] == ref)
                    return entry[i][1];

        }
    }
    convertitems() {
        if (this.rows != null) {
            this.rowEntries = [];
            let relations = [];
            const entry = Object.entries(this.rows[0]);

            for (let j = 0; j < entry.length; j++) {
                for (let k = 0; k < this.columns.length; k++)
                    if (this.columns[k].Key == entry[j][0])
                        relations.push([k, j]);
            }
            relations = relations.sort(function (a, b) {
                return a[0] - b[0];
            });
            for (let i = 0; i < this.rows.length; i++) {
                const row = Object.entries(this.rows[i]);
                let currentrow = [];
                for (let j = 0; j < relations.length; j++) {
                    currentrow.push(row[relations[j][1]][1]);
                }
                this.rowEntries.push(currentrow);
            }
        }

    }

}

export interface column {
    Name: string;
    Type: columnType;
    Key: string;
    width?: number;
    Reference?: string[];
    alignment?: alignment;
    editable?: boolean;
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
