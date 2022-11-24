import {Server} from "./server.js";
import queries, {Join} from "./queries.js";

export default class Comic {
    constructor() {
    }

    static async getComic(columns, wherecolumn, id, start, lang) {
        if (!lang)
            lang = "English";
        const leftjoin = [
            new Join("`Name`." + Server.con.escapeId(lang), "Name", "`TComic`.`FKName`", "PK"),
            new Join("`Description`." + Server.con.escapeId(lang), "Description", "`TComic`.`FKDescription`", "PK"),
           new Join("`Synopsis`." + Server.con.escapeId(lang), "Synopsis", "`TComic`.`FKSynopsis`", "PK")
        ];
        return await queries.selectLeftJoin("TComic", leftjoin, "TTranslation");
    }

    static async insertComic(rows,languages) {
        languages = languages.split(",");
       let names = [];
        let descriptions = [];
        let synopsises = []
        for (let i = 0; i < languages.length; i++){
            names.push("Name" + languages[i]);
            descriptions.push("Description" + languages[i]);
            synopsises.push("Synopsis" + languages[i]);
        }
        for (let i = 0; i < rows.length; i++) {
            rows[i] = await queries.insertTranslation(rows[i], "FKName", names, languages);
            rows[i] = await queries.insertTranslation(rows[i], "FKDescription", descriptions, languages);
            rows[i] = await queries.insertTranslation(rows[i], "FKSynopsis", synopsises, languages);
        }
        return await queries.insertItems("TComic", rows);
    }

    static async DeleteComic(columns, id) {
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
        await queries.deleteItems("TComic", columns, id);
    }
}
