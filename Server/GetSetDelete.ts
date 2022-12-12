import {Server} from "./server";
import Config from "./Config";
import Queries from "./queries";
import {OkPacket} from "mysql2";
import {LanguageField} from "../schema";

export default abstract class GetSetDelete<T> {

    public abstract getItems(columns?: string[], wherecolumn?: string, whereValue?: any, start?: number, lang?: string): Promise<T[]>;

    public abstract insertItem(item: T): Promise<number>;

    public abstract deleteItem(whereValue: any, whereColumn?: string): Promise<number>;

    static async QueryDB(Query: string): Promise<any> {
        return new Promise((resolve, reject) => Server.con.query(Query, (err: any, result: any) => {
            if (err) {
                console.log(err);
                switch (err.errno) {
                    case 1364:
                        reject(400);
                        break;
                    default:
                        reject(500);
                }
            }
            resolve(result);
        }));
    }

    public static async insert(item: any, tableName: string) {
        if (!item || !tableName) {
            console.log("GetSetDelete insert - Values missing:");
            console.log("\nitem\n");
            console.log(item);
            console.log("\ntableName\n");
            console.log(tableName);
            throw 400;
        }
        const result = await this.QueryDB(Queries.Insert(tableName, item)).catch(err => {
            throw err;
        });
        return (result as OkPacket).insertId;
    }

    public static async get(tableName: string, columns?: string[], whereColumn?: string, whereValue?: any, start?: number, count?: number, override?: boolean) {
        if (!tableName) {
            console.log("GetSetDelete get - Values missing:");
            console.log("\ntableName\n");
            console.log(tableName);
            throw 400;
        }
        const Query = GetSetDelete.Select(tableName, columns, whereValue, whereColumn) + GetSetDelete.Limit(start, count, override);

        return await this.QueryDB(Query).catch(err => {
            throw err;
        });
    }

    public static async delete(tableName: string, whereColumn: string, whereValue: any): Promise<number> {
        if (!tableName || !whereColumn || !whereValue) {
            console.log("GetSetDelete delete - Values missing:");
            console.log("\ntableName\n");
            console.log(tableName);
            console.log("\nwhereColumn\n");
            console.log(whereColumn);
            console.log("\nwhereValue\n");
            console.log(whereValue);
            throw 400;
        }
        return await this.QueryDB(GetSetDelete.Delete(tableName, whereColumn, whereValue)).catch(err => {
                throw err;
            }
        );
    }

    //Returns an LanguageField Object as a Key-Value Object, with the language as the key
    //e.g: values: [{value: "test", language: "English"}] = {English: "test"}
    public static langKeyVal(values: LanguageField): any {
        let allvalues: any = {};
        for (let i = 0; i < values.values.length; i++) {
            allvalues[values.values[i].language] = values.values[i].value;
        }
        return allvalues;
    }

    static deleteFields(item: any, fields: string[]): any {
        for (let i = 0; i < fields.length; i++)
            delete item[fields[i]];
    }

    static Limit(start?: number, count?: number, override?: boolean, multiply: boolean = true): string {
        start = (start && start > 0) ? start : 0;
        if (!override)
            count = (count && count > 0 && count < Config.count) ? count : Config.count;
        else
            count = count as number;
        return ` LIMIT ${multiply ? start * count : start},${count} `
    }

    static Select(tableName: string, columns?: string[], whereValue?: any, whereColumn?: string): string {
        let Query = ` SELECT ${columns && columns.length > 0 ? columns.join(",") : "*"} FROM ${Server.con.escapeId(tableName)} `;
        if (whereValue && whereColumn)
            Query += ` WHERE ${Server.con.escapeId(whereColumn)} IN (${Server.con.escape(whereValue)}) `;
        return Query;
    }

    static Delete(tableName: string, whereColumn: string, whereValue: any): string {
        return ` DELETE FROM ${Server.con.escapeId(tableName)} WHERE ${Server.con.escapeId(whereColumn)} in ${Server.con.escape(whereValue)} `;
    }

    static LeftJoin(tableName: string, alias: string, match: string, tomatch: string): string {
        return ` LEFT JOIN ${Server.con.escapeId(tableName)} AS ${Server.con.escapeId(alias)} ON ${match} = ${alias}.${Server.con.escapeId(tomatch)} `;
    }
}