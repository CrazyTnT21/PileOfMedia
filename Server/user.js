import queries from "./queries.js";
import Comic from "./comic.js";
import Manga from "./manga.js";

export default class User {
    constructor() {
    }

    static async getUser(id) {
        let alluser = {};
        alluser.user = await queries.getItems("TUser", null, "PK", id, null);
        alluser.comics = await queries.getItems("TUserXComic", null, "FKUser", id, null, 2);
        for (let i = 0; i < alluser.comics.length; i++)
            alluser.comics[i].comic = (await Comic.getComic(null, "PK", alluser.comics[i].FKComic, 0, "English"))[0];
        alluser.mangas = await queries.getItems("TUserXManga", null, "FKUser", id, null, 2);
        for (let i = 0; i < alluser.mangas.length; i++)
            alluser.mangas[i].manga = (await Manga.getManga(null, "PK", alluser.mangas[i].FKmanga, 0, "English"))[0];

        return alluser;
    }

    static async insertUser(rows) {
        return await queries.insertItems("TUser", rows);
    }

    static async loginUser(email, password) {
        const result = await queries.getItems("TAccount", null, "EMail", email, null)[0];
        if (result) {
            if (password === result.Password)
                return "Login!";
        } else
            return "Not Found";

    }
}
