import { Component, EventEmitter, Input, Output } from "@angular/core";

@Component({
    selector: 'app-CellEdit',
    templateUrl: './CellEdit.html',
    providers: []
})
export class CellEdit {

    @Input()
    value: string;
    @Output()
    valueChange = new EventEmitter<string>();
    edit: boolean = false;

    enterValue() {
        this.edit = !this.edit;
        this.valueChange.emit(this.value);
    }
}