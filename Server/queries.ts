import {Server} from "./server";
import GetSetDelete from "./GetSetDelete";

export default class Queries {

    static Insert(tableName: string, item: any): string {
        const allvalues = Object.entries(item);
        let columns = [];
        let values = [];
        for (let i = 0; i < allvalues.length; i++) {
            columns.push(Server.con.escapeId(allvalues[i][0]));
            values.push(Server.con.escape(allvalues[i][1]));
        }
        return ` INSERT INTO ${tableName}(${columns.join(",")}) VALUES(${values.join(",")}) `;
    }

    static async selectLeftJoin(table: string, joins: any[], table2: string, start?: number, count?: number) {
        let selectAliases = [];
        let leftjoins = [];
        let Query = "select " + table + ".*, ";
        for (let i = 0; i < joins.length; i++) {
            selectAliases.push(joins[i].selects + " as " + joins[i].alias);
            leftjoins.push(GetSetDelete.LeftJoin(table2, joins[i].alias, joins[i].match, joins[i].tomatch));
        }
        Query += selectAliases.join(",");
        Query += " from " + table + " ";
        Query += leftjoins.join(" ");
        Query += `${GetSetDelete.Limit(start, count)};`;
        return await GetSetDelete.QueryDB(Query);
    }
}

export class Join {
    constructor(selects: any, alias: any, match: any, tomatch: any) {
        this.selects = selects;
        this.alias = alias;
        this.match = match;
        this.tomatch = tomatch;
    }

    selects;// "`Name`.`English`",
    alias;// "Name",
    match;// "`TComic`.`FKName`",
    tomatch;// "PK" --Result: `Name`.`PK`
}
