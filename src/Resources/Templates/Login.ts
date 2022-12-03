import {Base} from "../../Resources/Base";
import {Component, Input} from "@angular/core";
import {DialogComponent} from "../../Resources/Templates/dialog.component";


@Component({
    selector: 'app-login',
    template: `
        <dialog #loginDialog style="border-radius: .1rem; border: 0; height: 30rem; width: 50rem" class="lazy">
            <row class="pad">
                <button class="save mar" (click)="login(loginDialog, true)">Send</button>
                <button class="mar" (click)="showDialog(loginDialog)">Close</button>
            </row>
        </dialog>
    `
})
export class LoginComponent extends Base {

    loggedin: boolean;

    login(dialog: HTMLDialogElement, login: boolean) {
        if (login)
            console.log("login");

        DialogComponent.closeDialog(dialog);
    }

    showDialog(dialog: HTMLDialogElement) {
        DialogComponent.closeDialog(dialog);
    }

    constructor() {
        super();
    }
}