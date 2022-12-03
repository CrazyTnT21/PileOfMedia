import {HttpParams} from '@angular/common/http';
import {AfterViewInit, Component} from '@angular/core';
import {Base} from "../../../Resources/Base";
import {alignment, columnType} from "../../../Resources/Templates/table.component";
import {Tools} from "../../../Resources/Tools";
import {HTTPRequester} from "../../../Resources/HttpRequester";
import {DialogComponent} from "../../../Resources/Templates/dialog.component";
import {TableClass} from "../../../Resources/Templates/TableClass";
import {DatePipe} from "@angular/common";

@Component({
    selector: 'app-comics',
    templateUrl: './comics.component.html'
})
export class ComicsComponent extends TableClass<TComic> implements AfterViewInit {

    constructor() {
        super();
        this.currentItem = this.createItem();
    }

    async ngAfterViewInit() {
        await this.loadItems();
    }

    show: boolean = false;
    override columns = [
        {
            Name: this.StringNames.Cover,
            Type: columnType.image,
            Key: "ImageSource",
            width: 3
        },
        {
            Name: this.StringNames.Title,
            Type: columnType.headertext,
            Key: "Name",
            alignment: alignment.left,
            Reference: [
                {
                    Name: this.StringNames.Description,
                    Type: columnType.text,
                    Key: "Description"
                },
                {
                    Name: this.StringNames.Synopsis,
                    Type: columnType.text,
                    Key: "Synopsis"
                }
            ],
        },
        {
            Name: this.StringNames.Volumes,
            Type: columnType.text,
            Key: "Volumes",
            formatting: "Volumes: [{}]",
            Reference: [
                {
                    Name: this.StringNames.Chapters,
                    Type: columnType.text,
                    Key: "Chapters",
                    formatting: "Chapters: [{}]"
                }
            ],
            width: 3
        },
        {
            Name: this.StringNames.AverageScore,
            Type: columnType.text,
            Key: "AverageScore",
            width: 3
        },
        {
            Name: this.StringNames.Status,
            Type: columnType.text,
            Key: "Status",
            width: 3
        },
        {
            Name: this.StringNames.StartDate,
            Type: columnType.text,
            Key: "PublishStart",
            formatting: "Publishing start: {}",
            pipes: [new DatePipe("YYYY-MM-dd",)],
            formatvalue: (value: any) => Tools.convertdate(value),
            Reference: [
                {
                    Name: this.StringNames.EndDate,
                    Type: columnType.text,
                    Key: "PublishEnd",
                    pipes: [new DatePipe("YYYY-MM-dd",)],
                    formatvalue: (value: any) => Tools.convertdate(value),
                    formatting: "Publishing end: {}",
                }],
            width: 6
        },
    ];

    async saveItems(dialog: HTMLDialogElement, create: boolean = false) {
        if (dialog.open) {
            if (create) {
                await this.saveItem(this.currentItem);
                this.currentItem = this.createItem();
            }
            DialogComponent.closeDialog(dialog);
        } else {
            DialogComponent.openDialog(dialog);
        }

    }

    async loadItems() {
        this.rows = await HTTPRequester.Get("api/Comic", new HttpParams().set("language",this.Languages[Base.currentLanguage]));
    }

    createItem(): TComic {
        return new TComic();
    }

    async deleteItem(item: TComic): Promise<any> {
    }

    async updateItem(item: TComic): Promise<any> {
    }

    async saveItem(item: TComic): Promise<any> {
        if (item.PK)
            console.log("Update!");

        await HTTPRequester.Post("api/Comic", new HttpParams().set("language", "English"), {rows: [item]});
    }
}

export class TComic {
    PK: number;
    Name: string;
    FKName: number;
    Description: string;
    FKDescription: number;
    Synopsis: string;
    FKSynopsis: number;
    Status: string;
    Chapters: number;
    Volumes: number;
    PublishStart: Date;
    PublishEnd: Date;
    ImageSource: string;
    LanguageFields: LanguageField[];
}

export class LanguageField {
    Value: string;
    Key: string;
    FKLanguage: number;
}
