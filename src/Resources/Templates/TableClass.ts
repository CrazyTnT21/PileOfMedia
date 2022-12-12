import {column} from "./table.component";
import {Base} from "../Base";

export abstract class TableClass<T> extends Base{

    public columns: column[];
    public rows: any[];

    currentItem: T;
    Items: T[];

    public abstract createItem(): T
    public abstract updateItem(item: T): any
    public abstract deleteItem(item: T): any
    public abstract saveItem(item: T): any
}
