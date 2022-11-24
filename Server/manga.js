import queries from "./queries.js";

export default class Manga {

    constructor() {
    }

    static async getManga(columns, wherecolumn, id, start) {
        return await queries.getItems("TManga", columns, wherecolumn, id, start);
    }

    static async insertManga(rows) {
       return await queries.insertItems("TManga",rows);
    }

    static async updateManga(rows) {

    }

    static async DeleteManga(columns, id) {
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
