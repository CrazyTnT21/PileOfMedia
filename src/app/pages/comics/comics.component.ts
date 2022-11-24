import {HttpParams} from '@angular/common/http';
import {AfterViewInit, Component} from '@angular/core';
import {Base} from 'src/app/mainapp/Base';
import {HTTPRequester} from 'src/Resources/other/HttpRequester';
import {alignment, column, columnType} from 'src/Resources/templates/table.component';
import {DialogComponent} from "../../../Resources/other/Dialog.Component";

@Component({
    selector: 'app-comics',
    templateUrl: './comics.component.html'
})
export class ComicsComponent extends Base implements AfterViewInit {

    tempitem: any;
    constructor() {
        super();
       this.tempitem = this.createTempItem();
    }
    createTempItem(){
        return {
            NameEnglish: null,
            DescriptionEnglish: null,
            SynopsisEnglish: null,
            Chapters: null,
            Volumes: null,
            PublishStart: null,
            PublishEnd: null,
            ImageSource: null};
    }
    async ngAfterViewInit() {
        await this.loadItems();
    }

    show: boolean = false;
    test = "Test";
    rows: any[];
    columns: column[] = [
        {
            Name: this.StringNames.Cover,
            Type: columnType.image,
            Key: "ImageSource",
            width: 6
        },
        {
            Name: this.StringNames.Title,
            Type: columnType.headertext,
            Key: "Name",
            alignment: alignment.left,
            width: 30,
            Reference: ["Description","Synopsis"],
            // Reference: [
            //     {
            //         Name: this.StringNames.Description,
            //         Type: columnType.text,
            //         Key: "IDeescription",
            //     },
            //     {
            //         Name: this.StringNames.Synopsis,
            //         Type: columnType.text,
            //         Key: "ISynopsis",
            //     }],
        },
        {
            Name: this.StringNames.Volumes,
            Type: columnType.text,
            Key: "Volumes",
            alignment: alignment.left
        },
        {
            Name: this.StringNames.Chapters,
            Type: columnType.text,
            Key: "Chapters",
            alignment: alignment.left
        },
        {
            Name: this.StringNames.AverageScore,
            Type: columnType.text,
            Key: "AverageScore"
        },
        {
            Name: this.StringNames.StartDate,
            Type: columnType.text,
            Key: "PublishStart"
        },
        {
            Name: this.StringNames.EndDate,
            Type: columnType.text,
            Key: "PublishEnd"
        }
    ];

    async saveItems(dialog: HTMLDialogElement, create: boolean = false) {
        if (dialog.open)
        {
            if (create) {
                console.log([this.tempitem])
                await HTTPRequester.Post("api/Comic", new HttpParams().set("language","English"), {rows: [this.tempitem]});
                this.tempitem = this.createTempItem();
            }
            DialogComponent.closeDialog(dialog);
        }
        else {
            DialogComponent.openDialog(dialog);
            console.log(this.tempitem);
        }

    }
    async loadItems() {

        this.test = HTTPRequester.url;
        this.rows = await HTTPRequester.Get("api/Comic", new HttpParams().set("language", Base.Language));
    }
}
