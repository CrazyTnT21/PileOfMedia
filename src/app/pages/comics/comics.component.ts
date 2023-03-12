import {Component, ViewChild} from '@angular/core';
import {columnType} from "../../../Resources/Templates/table.component";
import {Tools} from "../../../Resources/Tools";
import {DialogComponent} from "../../../Resources/Templates/dialog.component";
import {TableMulti} from "../../../Resources/Templates/TableClass";
import {ComicPage} from "./comicPage";
import {Comic} from "../../../../tables";

@Component({
  selector: 'app-comics',
  templateUrl: './comics.component.html',
  styleUrls: []
})
export class ComicsComponent extends TableMulti<Comic, ComicPage> {
  protected url: string = "api/Comic/";

  @ViewChild(DialogComponent)
  dialog: DialogComponent;

  constructor() {
    super();
  }

  protected columns = [
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
          Key: "chapters",
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
}

