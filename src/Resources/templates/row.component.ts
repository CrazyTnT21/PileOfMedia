import {Component} from '@angular/core';

@Component({
    selector: 'row',
    template: `
        <ng-container>
            <ng-content></ng-content>
        </ng-container>`,
    host: {"class": "col-12"}
})
export class RowComponent {
}
