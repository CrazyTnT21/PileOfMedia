import {Server} from "./server.js";

export default class Queries {

    constructor() {
    }

    static count = 50;

    static async QueryDB(Query) {
        return new Promise(resolve => Server.con.query(Query, (err, result) => {
            if (err)
                resolve(this.LogError(Object.entries(err)));
            // this.Log("Query");
            resolve(result);
        }));
    }

    // async Log(log) {
    //     const table = "TLog";
    //     const columns = ["Log", Server.con.escapeId("Date")];
    //     const now = new Date(new Date().getTime() - (new Date().getTimezoneOffset() * 60 * 1000)).toISOString().slice(0, 19).replace('T', ' ');
    //     const values = [Server.con.escape(log), Server.con.escape(now)];
    //     const Query = `INSERT INTO ${table}(${columns.join()}) values(${values.join()});`;
    //     return new Promise(resolve => Server.con.query(Query, (err, result) => {
    //         if (err)
    //             resolve(this.LogError(err));
    //
    //         resolve(result);
    //     }));
    // }

    static async LogError(error) {
        const table = "TError";
        const columns = ["Error", Server.con.escapeId("Date")];
        const now = new Date(new Date().getTime() - (new Date().getTimezoneOffset() * 60 * 1000)).toISOString().slice(0, 19).replace('T', ' ');
        const values = [`"${error.code}"`, `"${now}"`];
        const Query = `INSERT INTO ${table}(${columns.join()}) values(${values.join()});`;
        return new Promise(resolve => Server.con.query(Query, (err, result) => {
            if (err) {
                console.log(err)
                resolve(Server.responseStatus(500));
            }
            console.log(error);
            resolve(result);
        }));
    }

    static async getItems(Table, columns, wherecolumn, id, start) {
        let colvalue = "*";
        if (!Table)
            return Server.responseStatus(400);
        if (!start)
            start = 0;
        if (columns)
            colvalue = columns.join();
        let Query = `SELECT ${colvalue} FROM ${Server.con.escapeId(Table)} `;
        if (id && wherecolumn)
            Query += `WHERE ${Server.con.escapeId(wherecolumn)} IN (${Server.con.escape(id)}) `;

        Query += `LIMIT ${start * this.count},${this.count};`;
        return await this.QueryDB(Query);
    }

    static async insertItems(Table, rows) {
        if (!Table || !rows || rows.length > 50)
            return Server.responseStatus(400);
        let Insertrows = [];
        for (let i = 0; i < rows.length - 1; i++) {
            const values = Object.entries(rows[i])
            Insertrows.push(this.getRow(values));
        }
        console.log(rows);
        const lastvalues = Object.entries(rows[rows.length - 1])
        console.log(lastvalues);
        const InsertColumns = this.getColumns(lastvalues);

        Insertrows.push(this.getRow(lastvalues));
        Insertrows = Insertrows.join(",") + ";";
        const Query = `INSERT INTO ${Server.con.escapeId(Table)}${InsertColumns} values${Insertrows}`
        return await this.QueryDB(Query);
    }

    // async updateItems(Table, rows, where) {
    //     if (!Table || !rows || rows.length > 50)
    //         return this.server.responseStatus(400);
    //     // let columnvalue = [];
    //     // // for (let i = 0; i < rows.length; i++)
    //     // //     for (let j = 0; j < columns.length; j++) {
    //     // //         let rowvalue = "null";
    //     // //         if (rows[i][j])
    //     // //             rowvalue = rows[i][j];
    //     // //         columnvalue.push(`${columns[j]} = ${rowvalue}`)
    //     // //     }
    //     // const Query = `UPDATE ${Table} SET(${columnvalue.join()}) where ${where} =`;
    //     // return await this.QueryDB(Query);
    // }

    static async deleteItems(res, table, pks, wherecolumn) {
        if (!table || !wherecolumn || !pks || pks.length > 50)
            return res.send(Server.responseStatus(400));
        // const Query = `DELETE FROM ${table} WHERE ${wherecolumn} IN (${pks.join()})`;
        // Log(`DELETE FROM ${table} WHERE PK = (${pks});`);
        // Server.con.query(Query, function (err, result, fields) {
        //     if (err) LogError(Object.entries(error));
        //     res.send(result);
        // });
    }

    static async selectLeftJoin(table, joins, table2, options) {
        if (!options)
            options = {count: 50, start: 0}
        if (!options.count || options.count > 50)
            options.count = this.count;
        let selectAliases = [];
        let leftjoins = [];
        let Query = "select " + table + ".*, ";
        for (let i = 0; i < joins.length; i++) {
            selectAliases.push(joins[i].selects + " as " + joins[i].alias);
            leftjoins.push("left join " + Server.con.escapeId(table2) + " as " + Server.con.escapeId(joins[i].alias) + " on " + joins[i].match + " = " + joins[i].alias + "." + Server.con.escapeId(joins[i].tomatch))
        }
        Query += selectAliases.join(",");
        Query += " from " + table + " ";
        Query += leftjoins.join(" ");
        Query += ` LIMIT ${options.start * options.count},${options.count};`;
        console.log(Query);
        return await this.QueryDB(Query);
    }

   static getRow(obj) {
        let values = [];
        for (let i = 0; i < obj.length; i++) {
            values.push(Server.con.escape(obj[i][1]));
        }
        return "(" + values + ")";
    }

    //gets the columns from an object / row
    static getColumns(obj) {
        let values = [];
        for (let i = 0; i < obj.length; i++) {
            values.push(Server.con.escapeId(obj[i][0]));
        }
        return "(" + values.join(",") + ")";
    }


   static async insertTranslation(row, fkcolumn, values, columnnames) {
        let insertvalues = [];
        let test = {};
        console.log(row);
        for (let i = 0; i < columnnames.length; i++) {
            test[columnnames[i]] = row[values[i]];

            delete row[values[i]];
        }
        insertvalues.push(test);
        console.log(insertvalues);
        row[fkcolumn] = (await this.insertItems("TTranslation", insertvalues)).insertId;
        return row;
    }
}

export class Join {
    constructor(selects, alias, match, tomatch) {
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
