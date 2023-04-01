import {column} from "./table.component";
import {Base} from "../Base";
import {TranslateFields} from "../../../schema";
import {Component, OnInit} from "@angular/core";
import {HTTPRequester} from "../HttpRequester";
import {HttpParams} from "@angular/common/http";
import {ActivatedRoute} from "@angular/router";
import {BaseTable} from "../../../tables";
import {MainComponent} from "../../app/pages/main/main.component";

@Component({
  template: ``,
})
export abstract class TableClass<T> extends Base implements OnInit {
  protected abstract readonly url: string;
  oldLanguage: string = "EN";
  language: string = "EN";
  languages: any[] = [];

  public abstract load(): Promise<void>;

  async ngOnInit(): Promise<void> {
    if (!this.languages || this.languages.length < 2) {
      this.languages = await HTTPRequester.Get("api/lang", new HttpParams());
      console.log(this.languages)
    }
    await this.load();
  }
}

export abstract class TableSingle<T> extends TableClass<T> {
  public item: T = this.createItem();
  public edit: boolean;
  protected loadParams: HttpParams = new HttpParams().set("language", MainComponent.config.language);
  protected deleteParams: HttpParams = new HttpParams();
  protected saveParams: HttpParams = new HttpParams();
  protected updateParams: HttpParams = new HttpParams();

  protected constructor(private route: ActivatedRoute) {
    super();
  }

  public async load(): Promise<void> {
    try {
      this.item = await this.loadItem();
    } catch (e) {
      console.log(e);
    }
  }

  public async loadItem(): Promise<T> {
    return new Promise((resolve) => {
      this.route.params.subscribe(async params => {
        if (params["id"] > 0) {
          try {
            const result = await HTTPRequester.Get(this.url + params["id"], this.loadParams);
            if (result) {
              resolve(result);
            } else
              this.error = true;
          } catch (e) {
            resolve(this.createItem());
          }
        }
        else
          this.edit = true;
      });
    });
  };

  public async deleteItem(item: BaseTable): Promise<any> {
    if (item.pk)
      return await HTTPRequester.Delete(this.url, this.deleteParams.set("id", item.pk));
  }

  public async saveItem(item: T): Promise<any> {
    return HTTPRequester.Post<T>(this.url, new HttpParams(), item);
  }

  public async updateItem(item: T): Promise<any> {
    return HTTPRequester.Put<T>(this.url, new HttpParams(), item);
  }

  changelanguage(language: any) {
    let current = this.item as TranslateFields;
    if (!current.languageFields)
      return;
    for (let i = 0; i < current.languageFields.length; i++) {
      let found: boolean = false;
      let foundnew: boolean = false;
      for (let j = 0; j < current.languageFields[i].values.length; j++) {
        if (current.languageFields[i].values[j].language == this.oldLanguage) {
          found = true;
          current.languageFields[i].values[j].value = (current as any)[current.languageFields[i].bindProperty];
          break;
        }
      }
      if (!found) {
        current.languageFields[i].values.push({
          language: this.oldLanguage,
          value: (current as any)[current.languageFields[i].bindProperty]
        })
      }
      for (let j = 0; j < current.languageFields[i].values.length; j++) {
        if (current.languageFields[i].values[j].language == language) {
          foundnew = true;
          (current as any)[current.languageFields[i].bindProperty] = current.languageFields[i].values[j].value;
          break;
        }
      }
      if (!foundnew)
        (current as any)[current.languageFields[i].bindProperty] = "";
      console.log(current);
    }
    this.oldLanguage = language;
  }

  public abstract createItem(): T
}

export abstract class TableMulti<T, Page extends TableSingle<T> | null> extends TableClass<T> {
  public items: T[];
  protected selectedItems: T[];
  protected abstract columns: column[];

  public async load(): Promise<void> {
    this.items = await this.loadItems();
    console.log(this.items);
  }

  public async loadItems(): Promise<T[]> {
    return await HTTPRequester.Get(this.url, new HttpParams().set("language", MainComponent.config.language));
  }

}
