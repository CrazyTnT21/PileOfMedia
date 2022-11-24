import {Component, Input} from '@angular/core';

@Component({
    selector: 'app-card',
    template: `
            <ng-content></ng-content>
    `,
    host: {"class": "fix-3 lazy"}
})
export class CardComponent {

//sizes starting from 1-12 using the class fix-1-12
    @Input() size: number = 3;

    constructor() {
    }
}


