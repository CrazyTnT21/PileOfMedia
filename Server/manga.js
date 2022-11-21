import Queries from "./queries.js";

export default class Manga {
    queries;

    constructor(quer) {
        this.queries = quer;
    }

    async getManga(columns, wherecolumn, id, start) {
        return await this.queries.getItems("TManga", columns, wherecolumn, id, start);
    }

    async insertManga(rows) {
       return await this.queries.insertItems("TManga",rows);
    }

    async updateManga(rows) {

    }

    async DeleteManga(columns, id) {
        //     const pk = pks.split(",");
//     for (let i = 0; i < pk.length; i++) {
//         //TMangaXGenre
//         deleteItems(res, "TMangaXGenre", pk[i], "FKManga");
//         //TMangaXTheme
//         deleteItems(res, "TMangaXTheme", pk[i], "FKManga");
//         //TUserXManga
//         updateItems(res, "TMangaXGenre", ["FKManga"], [[1]], "PK");
//         //TMangaXCreator
//         deleteItems(res, "TMangaXGenre", pk[i], "FKManga");
//         //TManga
//         deleteItems(res, Table, pk[i], wherecolumn);
//     }
        await this.queries.deleteItems("TManga", columns, id);
    }
}
