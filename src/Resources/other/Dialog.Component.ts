export class DialogComponent {

    public static closeDialog(dialog: HTMLDialogElement) {
        dialog.close();
    }

    public static openDialog(dialog: HTMLDialogElement, modal: boolean = true) {
        if (modal)
            dialog.showModal();
        else
            dialog.show();
    }

}

