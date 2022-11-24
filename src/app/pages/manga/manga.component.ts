import {Component, OnInit} from '@angular/core';
import {Tools} from 'src/app/tools/Tools';
import {alignment, column, columnType} from "../../../Resources/templates/table.component";
import {Base} from "../../mainapp/Base";
import {HTTPRequester} from "../../../Resources/other/HttpRequester";
import {HttpParams} from "@angular/common/http";

@Component({
    selector: 'app-manga',
    templateUrl: './manga.component.html'
})
export class MangaComponent extends Base implements OnInit {

    rows: any[];
    columns: column[] = [
        {
            Name: this.StringNames.Cover,
            Type: columnType.image,
            Key: "ImageSource",
            width: 10
        },
        {
            Name: this.StringNames.Title,
            Type: columnType.headertext,
            Key: "Name",
            alignment: alignment.left,
            width: 30,
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

    constructor() {
        super();
    }

    ngOnInit(): void {
    }

    async loadItems() {
        this.rows = await HTTPRequester.Get("api/manga", new HttpParams());
        console.log(this.rows)
    }

    async saveItems() {
        const emptytable: any = {
            rows: [{
                AverageScore: 8.45,
                Chapters: 12,
                Description: "STATUS 2 This is a test description This is a test description This is a test description This is a test description This is a test description. The end",
                Synopsis: "This is a test description This is a test description This is a test description This is a test description This is a test description. The end",
                ImageSource: "https://upload.wikimedia.org/wikipedia/en/a/a2/Watchmen%2C_issue_1.jpg",
                Name: "Not Watchmen",
                PublishEnd: "2002-12-2",
                PublishStart: "2002-12-3",
                Volumes: 2,
            },
                {
                    AverageScore: 8.45,
                    Chapters: 12,
                    Description: "STATUS 4 This is a test description This is a test description This is a test description This is a test description This is a test description. The end",
                    Synopsis: "This is a test description This is a test description This is a test description This is a test description This is a test description. The end",
                    ImageSource: "https://upload.wikimedia.org/wikipedia/en/a/a2/Watchmen%2C_issue_1.jpg",
                    Name: "Not Watchmen",
                    PublishEnd: "2002-12-2",
                    PublishStart: "2120-1-1",
                    Volumes: 2,
                }]
        };
        // let items = this.rows;
        // for (let i = 0; i < items.length; i++) {
        //   delete items[i].PK;
        //   delete items[i].IPublishEnd;
        //   delete items[i].IPublishStart;
        // }
        await HTTPRequester.Post("api/manga", new HttpParams(), emptytable);

    }
}
