import {Component} from '@angular/core';
import {Base} from "../Base";

@Component({
    selector: 'app-footer',
    template: `
        Test
    `,
    host: {"class": "col-12 lazy footer"}
})
export class FooterComponent extends Base {

    constructor() {
        super();
    }

}