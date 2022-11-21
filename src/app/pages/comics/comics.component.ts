import {HttpParams} from '@angular/common/http';
import {AfterViewInit, Component} from '@angular/core';
import {Base} from 'src/app/mainapp/Base';
import {HTTPRequester} from 'src/Resources/other/HttpRequester';
import {alignment, column, columnType} from 'src/Resources/templates/table.component';

@Component({
    selector: 'app-comics',
    templateUrl: './comics.component.html'
})
export class ComicsComponent extends Base implements AfterViewInit {

    constructor() {
        super();
    }

    async ngAfterViewInit() {
        await this.loadItems();
    }

    rows: any[];
    columns: column[] = [
        {
            Name: this.StringNames.Cover,
            Type: columnType.image,
            Key: "ImageSource",
            width: 5
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
    ]

    async testpost() {
        await HTTPRequester.postTable("api/items", new HttpParams().set("table", "TComic"), HTTPRequester.toTObject(this.rows));

    }

    async saveItems() {
        console.log(this.rows);
        const emptytable: any = {rows:[{
            AverageScore: 8.45,
            Chapters: 12,
            DescriptionEnglish: "EnglishDescription1",
            DescriptionGerman: "GermanDescription1",
            ImageSource: "https://upload.wikimedia.org/wikipedia/en/a/a2/Watchmen%2C_issue_1.jpg",
            NameEnglish: "Not Watchmen",
            NameGerman: "Wächtermänner",
            PublishEnd: "2104-12-02",
            PublishStart: "2033-12-01",
            Volumes: 2,
        },
            {
                AverageScore: 8.45,
                Chapters: 12,
                DescriptionEnglish: "EnglishDescription12",
                DescriptionGerman: "GermanDescription12",
                ImageSource: "https://upload.wikimedia.org/wikipedia/en/a/a2/Watchmen%2C_issue_1.jpg",
                NameEnglish: "Not Watchmen2",
                NameGerman: "Wächtermänner2",
                PublishEnd: "2104-12-02",
                PublishStart: "2033-12-01",
                Volumes: 2,
            }]};
        await HTTPRequester.Post("api/Comic", new HttpParams(), emptytable);

    }

    async loadItems() {
        this.rows = await HTTPRequester.Get("api/Comic", new HttpParams().set("language",Base.Language));
        console.log(this.rows)
    }
}
