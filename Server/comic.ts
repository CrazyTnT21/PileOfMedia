import Queries, {Join} from "./queries";
import {Server} from "./server";
import {Languages, TComic} from "../schema";
import GetSetDelete from "./GetSetDelete";

export default class Comic implements GetSetDelete<TComic> {
    constructor() {
    }

    public async getItems(columns?: string[], wherecolumn?: string, whereValue?: any, start?: number, lang?: string): Promise<TComic[]> {
        if (!lang)
            lang = Languages.English;
        const leftjoin = [
            new Join("`Name`." + Server.con.escapeId(lang), "Name", "`TComic`.`FKName`", "PK"),
            new Join("`Description`." + Server.con.escapeId(lang), "Description", "`TComic`.`FKDescription`", "PK"),
            new Join("`Synopsis`." + Server.con.escapeId(lang), "Synopsis", "`TComic`.`FKSynopsis`", "PK"),
        ];

        return await Queries.selectLeftJoin("TComic", leftjoin, "TTranslation") as TComic[];

    }

    public async insertItem(item: TComic): Promise<number> {
        if (!item || !item.languageFields) {
            Server.Writelog(`Comic insertItem - Values missing: item ${item}"`);
            throw 400;
        }
        const tempname = item.languageFields.find(x => x.column == "FKName");
        if (tempname)
            item.fkName = await GetSetDelete.insert(GetSetDelete.langKeyVal(tempname), "TTranslation")
        else
            throw 400;


        const tempdescription = item.languageFields.find(x => x.column == "FKDescription");
        if (tempdescription)
            item.fkDescription = await GetSetDelete.insert(GetSetDelete.langKeyVal(tempdescription), "TTranslation")

        const tempsynopsis = item.languageFields.find(x => x.column == "FKSynopsis");
        if (tempsynopsis)
            item.fkSynopsis = await GetSetDelete.insert(GetSetDelete.langKeyVal(tempsynopsis), "TTranslation")
        GetSetDelete.deleteFields(item, ["LanguageFields", "Description", "Name", "Synopsis"]);
        return GetSetDelete.insert(item, "TComic").catch(err => {
            throw err
        });
    }

    public async deleteItem(whereValue: any, whereColumn?: string): Promise<number> {
        if (!whereValue || whereValue < 1) {
            Server.Writelog("Comic deleteItem - Values missing:");
            console.log("whereValue");
            console.log(whereValue);
            throw 400;
        }
        await GetSetDelete.delete("TComicChapter", "FKComic", whereValue);
        await GetSetDelete.delete("TComicVolume", "FKComic", whereValue);
        await GetSetDelete.delete("TComicXCharacter", "FKComic", whereValue);
        await GetSetDelete.delete("TComicXCreator", "FKComic", whereValue);
        await GetSetDelete.delete("TComicXGenre", "FKComic", whereValue);
        await GetSetDelete.delete("TComicXTheme", "FKComic", whereValue);
        //Could instead just replace FKComic with, where 1 is a placeholder for an unkown comic so that the user doesn't lose their progress
        await GetSetDelete.delete("TUserXComic", "FKComic", whereValue);

        return await GetSetDelete.delete("TComic", "PK", whereValue);
    }

    // static async DeleteComic(columns, id) {
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
//         await queries.deleteItems("TComic", columns, id);
//     }
}
