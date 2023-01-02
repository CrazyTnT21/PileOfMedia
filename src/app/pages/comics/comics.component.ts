import {HttpParams} from '@angular/common/http';
import {AfterViewInit, Component, ViewChild} from '@angular/core';
import {columnType} from "../../../Resources/Templates/table.component";
import {Tools} from "../../../Resources/Tools";
import {HTTPRequester} from "../../../Resources/HttpRequester";
import {DialogComponent} from "../../../Resources/Templates/dialog.component";
import {TableClass} from "../../../Resources/Templates/TableClass";
import {TComic} from "../../../../schema";

@Component({
    selector: 'app-comics',
    templateUrl: './comics.component.html'
})
export class ComicsComponent extends TableClass<TComic> implements AfterViewInit {

    preview(value: any): string {
        return "";
    }

    @ViewChild(DialogComponent)
    dialog: DialogComponent;

    constructor() {
        super();
        this.currentItem = this.createItem();
    }

    async ngAfterViewInit() {
        await this.loadItems();
    }

    public override columns = [
        {
            Name: this.StringNames.Cover,
            Type: columnType.image,
            Key: "imageSource",
            width: 6,
            maxwidth: 6
        },
        {
            Name: this.StringNames.Title,
            Type: columnType.headertext,
            Key: "name",
            width: 6,
            Reference: [
                {
                    Name: this.StringNames.Description,
                    Type: columnType.text,
                    Key: "description"
                },
                {
                    Name: this.StringNames.Synopsis,
                    Type: columnType.text,
                    Key: "synopsis"
                }
            ],
        },
        {
            Name: this.StringNames.Volumes,
            Type: columnType.text,
            Key: "volumes",
            formatting: "Volumes: [{}]",
            Reference: [
                {
                    Name: this.StringNames.Chapters,
                    Type: columnType.text,
                    Key: "Chapters",
                    formatting: "Chapters: [{}]"
                }
            ],
            width: 6,
            maxwidth: 6
        },
        {
            Name: this.StringNames.AverageScore,
            Type: columnType.text,
            Key: "averageScore",
            width: 4,
            maxwidth: 6
        },
        {
            Name: this.StringNames.Status,
            Type: columnType.text,
            Key: "status",
            width: 3
        },
        {
            Name: this.StringNames.StartDate,
            Type: columnType.text,
            Key: "publishStart",
            formatting: "Publishing start: {}",
            formatvalue: (value: any) => Tools.convertdate(value),
            Reference: [
                {
                    Name: this.StringNames.EndDate,
                    Type: columnType.text,
                    Key: "publishEnd",
                    formatvalue: (value: any) => Tools.convertdate(value),
                    formatting: "Publishing end: {}",
                }],
            width: 12,
            maxwidth: 12
        },
    ];


    async loadItems() {
        this.Items = await HTTPRequester.Get("api/Comic/", new HttpParams().set("language", this.currentLanguage));
        console.log(this.Items)
    }

    createItem(): TComic {
        let newitem = new TComic();
        newitem.languageFields = [
            {
                column: "fkName",
                bindProperty: "Name",
                values: []
            },
            {
                column: "fkDescription",
                bindProperty: "Description",
                values: []
            },
            {
                column: "fkSynopsis",
                bindProperty: "Synopsis",
                values: []
            }
        ];
        return newitem;
    }

    async deleteItem(item: TComic): Promise<any> {
        await HTTPRequester.Delete("api/Comic", new HttpParams().set("id", item.pk as number));
    }

    async updateItem(item: TComic): Promise<any> {
    }

    async saveItem(item: TComic): Promise<any> {
        if (item.pk)
            console.log("Update!");

        await HTTPRequester.Post("api/Comic", new HttpParams().set("language", "English"), item);
    }
}

