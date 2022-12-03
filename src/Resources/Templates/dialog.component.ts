import {Base} from "../../Resources/Base";
import {AfterViewInit, Component, EventEmitter, Input, Output, ViewChild} from "@angular/core";


@Component({
    selector: 'app-dialog',
    template: `
        <dialog #dialog [ngStyle]="{'width.rem': width, 'height.rem': height}" style="border-radius: .1rem; border: 0;" class="lazy">
            <ng-content></ng-content>
            <row *ngIf="showSave">
                <button class="save" (click)="closeDialog(true)">Send</button>
                <button class="ml" (click)="closeDialog()">Close</button>
            </row>
        </dialog>
    `
})
export class DialogComponent extends Base implements AfterViewInit {

    @ViewChild("dialog") dialog: HTMLDialogElement;

    @Input() showSave: boolean = true;
    @Input() height: number | null = 30;
    @Input() width: number | null = 50;
    @Input() autosize: boolean;

    @Output() save: EventEmitter<any> = new EventEmitter();

    public static closeDialog(dialog: HTMLDialogElement) {
        dialog.close();
    }

    public static openDialog(dialog: HTMLDialogElement, modal: boolean = true) {
        if (modal)
            dialog.showModal();
        else
            dialog.show();
    }

    constructor() {
        super();
    }

    ngAfterViewInit(): void {
        this.dialog = (this.dialog as any).nativeElement;

        if (this.autosize) {
            this.dialog.style.width = "";
            this.dialog.style.height = "";
        }
    }

    show() {
        this.dialog.showModal();
    }

    closeDialog(save: boolean = false) {
        if (save)
            this.save.emit();
        this.dialog.close();
    }
}