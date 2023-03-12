import {Component, ViewChild} from "@angular/core";
import {TableSingle} from "../../../Resources/Templates/TableClass";
import {ActivatedRoute} from "@angular/router";
import {DialogComponent} from "../../../Resources/Templates/dialog.component";
import {Manga} from "../../../../tables";

@Component({
  selector: 'app-mangaPage',
  templateUrl: './mangaPage.html'
})
export class MangaPageComponent extends TableSingle<Manga> {

  @ViewChild(DialogComponent)
  public createDialog: DialogComponent;

  protected readonly url: string = "api/Manga/";

  constructor(private routea: ActivatedRoute) {
    super(routea);
  }

  createItem(): Manga {
    let newitem: Manga = new Manga();
    newitem.languageFields = [
      {
        column: "fkName",
        bindProperty: "name",
        values: []
      },
      {
        column: "fkDescription",
        bindProperty: "description",
        values: []
      },
    ];
    return newitem;
  }

  override async saveItem(item: Manga): Promise<any> {
    this.changelanguage(this.language);
    return super.saveItem(item);
  }
}
