export default class Comic {
    queries;

    constructor(quer) {
        this.queries = quer;
    }

    async getComic(columns, wherecolumn, id, start, lang) {
        return await this.queries.getLanguageItem("TComic", lang, columns, "FKName", "Name");
    }

    async insertComic(rows) {
        for (let i = 0; i < rows.length; i++) {
            rows[i] = await this.queries.insertTranslation(rows[i], "FKName", ["NameEnglish", "NameGerman"], ["English", "German"]);
            rows[i] = await this.queries.insertTranslation(rows[i], "FKDescription", ["DescriptionEnglish", "DescriptionGerman"], ["English", "German"]);
        }
        return await this.queries.insertItems("TComic", rows);
    }

    async DeleteComic(columns, id) {
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
        await this.queries.deleteItems("TComic", columns, id);
    }
}
