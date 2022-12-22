import {column} from "./table.component";
import {Base} from "../Base";
import {Languages, TComic, TranslateFields} from "../../../schema";

export abstract class TableClass<T> extends Base {

    public columns: column[];
    currentItem: T;
    Items: T[];
    currentLanguage: string = "English";
    oldLanguage: string = "English";
    public abstract createItem(): T

    public abstract updateItem(item: T): any

    public abstract deleteItem(item: T): any

    public abstract saveItem(item: T): any

    changelanguage(language: any) {
        let current = this.currentItem as TranslateFields;
        if (!current.languageFields)
            return;
        for (let i = 0; i < current.languageFields.length; i++) {
            let found: boolean = false;
            for (let j = 0; j < current.languageFields[i].values.length; j++) {
                console.log(current.languageFields[i].values[j])
                if (this.oldLanguage == current.languageFields[i].values[j].language) {
                    found = true;
                    break;
                }
                if (language == current.languageFields[i].values[j].language) {
                    (current as any)[current.languageFields[i].bindProperty] = current.languageFields[i].values[j].value;
                }
            }
            if (!found) {
                current.languageFields[i].values.push({
                    language: this.oldLanguage,
                    value: (current as any)[current.languageFields[i].bindProperty]
                });
                (current as any)[current.languageFields[i].bindProperty] = "";
            }

        }
        this.oldLanguage = language;
        // if (!this.currentItem.languageFields)
        //     this.currentItem.languageFields = [];
        // console.log(this.currentItem.languageFields);
        // const result = this.currentItem.languageFields.findIndex(x => x.column == column);
        // console.log(result);
        // if (result > 0) {
        //     const resulttwo = this.currentItem.languageFields[result].values.findIndex(x => x.language == language);
        //     if (resulttwo > 0)
        //         this.currentItem.languageFields[result].values[resulttwo].value = value;
        //     else
        //         this.currentItem.languageFields[result].values.push({language: language, value: value});
        // } else
        //     this.currentItem.languageFields.push({column: column, values: [{value: value, language: language}]});
    }
}
