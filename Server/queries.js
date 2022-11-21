export default class Queries {
    server;
    con;

    constructor(servers, con) {
        this.server = servers;
        this.con = con;
    }

    count = 50;

    async QueryDB(Query) {
        return new Promise(resolve => this.con.query(Query, (err, result) => {
            if (err)
                resolve(this.LogError(Object.entries(err)));
            // this.Log("Query");
            resolve(result);
        }));
    }

    // async Log(log) {
    //     const table = "TLog";
    //     const columns = ["Log", this.con.escapeId("Date")];
    //     const now = new Date(new Date().getTime() - (new Date().getTimezoneOffset() * 60 * 1000)).toISOString().slice(0, 19).replace('T', ' ');
    //     const values = [this.con.escape(log), this.con.escape(now)];
    //     const Query = `INSERT INTO ${table}(${columns.join()}) values(${values.join()});`;
    //     return new Promise(resolve => this.con.query(Query, (err, result) => {
    //         if (err)
    //             resolve(this.LogError(err));
    //
    //         resolve(result);
    //     }));
    // }

    async LogError(error) {
        const table = "TError";
        const columns = ["Error", this.con.escapeId("Date")];
        const now = new Date(new Date().getTime() - (new Date().getTimezoneOffset() * 60 * 1000)).toISOString().slice(0, 19).replace('T', ' ');
        const values = [`"${error.code}"`, `"${now}"`];
        const Query = `INSERT INTO ${table}(${columns.join()}) values(${values.join()});`;
        return new Promise(resolve => this.con.query(Query, (err, result) => {
            if (err) {
                console.log(err)
                resolve(this.server.responseStatus(500));
            }
            resolve(result);
        }));
    }

    async getItems(Table, columns, wherecolumn, id, start) {
        let colvalue = "*";
        if (!Table)
            return this.server.responseStatus(400);
        if (!start)
            start = 0;
        if (columns)
            colvalue = columns.join();
        let Query = `SELECT ${colvalue} FROM ${this.con.escapeId(Table)} `;
        if (id && wherecolumn)
            Query += `WHERE ${this.con.escapeId(wherecolumn)} IN (${this.con.escape(id)}) `;

        Query += `LIMIT ${start * this.count},${this.count};`;
        return await this.QueryDB(Query);
    }

    async insertItems(Table, rows) {
        if (!Table || !rows || rows.length > 50)
            return this.server.responseStatus(400);
        let Insertrows = [];
        for (let i = 0; i < rows.length - 1; i++) {
            const values = Object.entries(rows[i])
            Insertrows.push(this.getRow(values));
        }
        const lastvalues = Object.entries(rows[rows.length - 1])
        const InsertColumns = this.getColumns(lastvalues);

        Insertrows.push(this.getRow(lastvalues));
        Insertrows = Insertrows.join(",") + ";";
        const Query = `INSERT INTO ${this.con.escapeId(Table)}${InsertColumns} values${Insertrows}`
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

    async deleteItems(res, table, pks, wherecolumn) {
        if (!table || !wherecolumn || !pks || pks.length > 50)
            return res.send(this.server.responseStatus(400));
        // const Query = `DELETE FROM ${table} WHERE ${wherecolumn} IN (${pks.join()})`;
        // Log(`DELETE FROM ${table} WHERE PK = (${pks});`);
        // this.con.query(Query, function (err, result, fields) {
        //     if (err) LogError(Object.entries(error));
        //     res.send(result);
        // });
    }

    async selectLeftJoin(table, tablecolumn, table2, table2column, tablematch, table2match, as) {
        let colvalue = "*"
        if (!table || !table2column)
            return this.server.responseStatus(400);
        if (tablecolumn)
            colvalue = this.con.escapeId(tablecolumn.join());
        // select TComic.*,TTranslation.English from tcomic left join TTranslation on TComic.FkName = TTranslation.PK;
        const Query = `select ${this.con.escapeId(table)}.${colvalue},${this.con.escapeId(table2)}.${this.con.escapeId(table2column)} as ${this.con.escapeId(as)} from ${this.con.escapeId(table)} left join ${this.con.escapeId(table2)} on ${this.con.escapeId(table2)}.${this.con.escapeId(table2match)} = ${this.con.escapeId(table)}.${this.con.escapeId(tablematch)}`
        return await this.QueryDB(Query);
    }
    async getLanguageItem(Table,lang,columns,columnmatch,as){
        if (!lang)
            lang = "English";
        return await this.selectLeftJoin(Table, columns, "TTranslation", lang, columnmatch, "PK", as);
    }

    getRow(obj) {
        let values = [];
        for (let i = 0; i < obj.length; i++) {
            values.push(this.con.escape(obj[i][1]));
        }
        return "(" + values + ")";
    }

    //gets the columns from an object / row
    getColumns(obj) {
        let values = [];
        for (let i = 0; i < obj.length; i++) {
            values.push(this.con.escapeId(obj[i][0]));
        }
        return "(" + values.join(",") + ")";
    }

    async insertTranslation(row, fkcolumn, values, columnnames) {
        let insertvalues = [];
        let test = {};
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