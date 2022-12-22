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
            Key: "ImageSource",
            width: 6,
            maxwidth: 6
        },
        {
            Name: this.StringNames.Title,
            Type: columnType.headertext,
            Key: "Name",
            width: 6,
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
            width: 6,
            maxwidth: 6
        },
        {
            Name: this.StringNames.AverageScore,
            Type: columnType.text,
            Key: "AverageScore",
            width: 4,
            maxwidth: 6
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
            formatvalue: (value: any) => Tools.convertdate(value),
            Reference: [
                {
                    Name: this.StringNames.EndDate,
                    Type: columnType.text,
                    Key: "PublishEnd",
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
                column: "FKName",
                bindProperty: "Name",
                values: []
            },
            {
                column: "FKDescription",
                bindProperty: "Description",
                values: []
            },
            {
                column: "FKSynopsis",
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

