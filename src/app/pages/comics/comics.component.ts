import {HttpParams} from '@angular/common/http';
import {AfterViewInit, Component, ViewChild} from '@angular/core';
import {alignment, columnType} from "../../../Resources/Templates/table.component";
import {Tools} from "../../../Resources/Tools";
import {HTTPRequester} from "../../../Resources/HttpRequester";
import {DialogComponent} from "../../../Resources/Templates/dialog.component";
import {TableClass} from "../../../Resources/Templates/TableClass";
import {DatePipe} from "@angular/common";
import {TComic} from "../../../../schema";

@Component({
    selector: 'app-comics',
    templateUrl: './comics.component.html'
})
export class ComicsComponent extends TableClass<TComic> implements AfterViewInit {


    @ViewChild(DialogComponent)
    dialog: DialogComponent;

    getvalue(column: string, language: string): string {
        if (!this.currentItem.LanguageFields)
            this.currentItem.LanguageFields = [];
        const result = this.currentItem.LanguageFields.findIndex(x => x.column == column);
        console.log(result);
        if (result > 0) {
            const resulttwo = this.currentItem.LanguageFields[result].values.findIndex(x => x.language == language);
            if (resulttwo > 0)
                return this.currentItem.LanguageFields[result].values[resulttwo].value;
            else
                return "";
        } else
            return "";
    }

    changevalue(value: string, column: string, language: string) {
        if (!this.currentItem.LanguageFields)
            this.currentItem.LanguageFields = [];
        console.log(this.currentItem.LanguageFields);
        const result = this.currentItem.LanguageFields.findIndex(x => x.column == column);
        console.log(result);
        if (result > 0) {
            const resulttwo = this.currentItem.LanguageFields[result].values.findIndex(x => x.language == language);
            if (resulttwo > 0)
                this.currentItem.LanguageFields[result].values[resulttwo].value = value;
            else
                this.currentItem.LanguageFields[result].values.push({language: language, value: value});
        } else
            this.currentItem.LanguageFields.push({column: column, values: [{value: value, language: language}]});
    }

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


    async loadItems() {
        this.rows = await HTTPRequester.Get("api/Comic", new HttpParams().set("language", this.Languages[this.currentLanguage].Language));
        console.log(this.rows);
    }

    createItem(): TComic {
        let newitem = new TComic();
        newitem.LanguageFields = [
            {
                column: "FKName",
                values: [{value: "", language: this.Languages[this.currentLanguage].Language}]
            },
            {
                column: "FKDescription",
                values: [{value: "", language: this.Languages[this.currentLanguage].Language}]
            },
            {
                column: "FKSynopsis",
                values: [{value: "", language: this.Languages[this.currentLanguage].Language}]
            }
        ];
        return newitem;
    }

    async deleteItem(item: TComic): Promise<any> {
        await HTTPRequester.Delete("api/Comic", new HttpParams().set("id", item.PK as number));
    }

    async updateItem(item: TComic): Promise<any> {
    }

    async saveItem(item: TComic): Promise<any> {
        if (item.PK)
            console.log("Update!");

        await HTTPRequester.Post("api/Comic", new HttpParams().set("language", "English"), item);
    }
}

