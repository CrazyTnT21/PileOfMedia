import {column} from "./table.component";
import {Base} from "../Base";
import {TranslateFields} from "../../../schema";
import {Component, Directive, Injectable, OnInit} from "@angular/core";
import {HTTPRequester} from "../HttpRequester";
import {HttpParams} from "@angular/common/http";
import {ActivatedRoute} from "@angular/router";
import {BaseTable} from "../../../tables";

@Component({
  template: ``,
})
export abstract class TableClass<T> extends Base implements OnInit {
  protected abstract readonly url: string;
  oldLanguage: string = "EN";


  public abstract load(): Promise<void>;


  async ngOnInit(): Promise<void> {
    if (!Base.languages || Base.languages.length < 2)
      Base.languages = await HTTPRequester.Get("api/lang", new HttpParams());
    console.log(Base.languages)
    await this.load();
  }
}

export abstract class TableSingle<T> extends TableClass<T> {
  public item: T = this.createItem();
  public edit: boolean;


  public constructor(private route: ActivatedRoute) {
    super();
  }

  public async load(): Promise<void> {
    this.item = await this.loadItem();
  }

  public async loadItem(): Promise<T> {
    return new Promise((resolve) => {

      this.route.params.subscribe(async params => {

        if (params["id"] > 0) {
          const result = await HTTPRequester.Get(this.url + params["id"], new HttpParams().set("language", Base.language));
          if (result && result.length > 0) {
            resolve(result[0]);

          }
          else
            this.error = true;
        } else
          this.edit = true;
      });
    });
  };

  public async deleteItem(item: BaseTable): Promise<any> {
    if (item.pk)
      return await HTTPRequester.Delete(this.url, new HttpParams().set("id", item.pk));
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

export abstract class TableMulti<T, Page = null> extends TableClass<T> {
  public items: T[];
  protected selectedItems: T[];
  protected abstract columns: column[];

  public async load(): Promise<void> {
    this.items = await this.loadItems();
  }

  public async loadItems(): Promise<T[]> {
    return await HTTPRequester.Get(this.url, new HttpParams().set("language", Base.language));
  }

}
