import queries, {Join} from "./queries.js";
import {Server} from "./server.js";
export default class Stuff {
    constructor() {
    }

    static async getStatus(id,lang) {
        if (!lang)
            lang = "English";
        const leftjoin = [
            new Join("`Status`." + Server.con.escapeId(lang), "Status", "`TStatus`.`FKStatus`", "PK"),
        ];
        return (await queries.selectLeftJoin("TStatus", leftjoin, "TTranslation"))[0].Status;
    }
}
