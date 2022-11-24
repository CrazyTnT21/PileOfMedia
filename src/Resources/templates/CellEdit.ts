import {Component, EventEmitter, Input, Output} from "@angular/core";

@Component({
    selector: 'app-CellEdit',
    template: `
        <div (click)="enterValue()">
            <ng-container *ngIf="!edit">
                <ng-content></ng-content>
            </ng-container>
        </div>
        <input *ngIf="edit"
               [(ngModel)]="value"
               (blur)="enterValue()"/>`,
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