//(GET, INSERT, UPDATE) query.id == where values (example: where FKStatus = (.id.join()))
//(GET,INSERT,UPDATE) query.columns == selected columns (example1: select .column.join() from Tcomic, example2: insert into TComic(.column.join()) values(1,2,3))
//(GET,INSERT,UPDATE) query.where == where columns
//(GET,INSERT,UPDATE) query.whereValue == where value
//(GET) query.start == how many items to get (start will be multiplied by 50 to represent a page)
import * as process from "process";
import * as cors from 'cors';
import * as mysql from "mysql2";
import * as express from "express";
import Comic from "./comic";
import {TComic} from "../schema";
import GetSetDelete from "./GetSetDelete";

export class Server {
    static con: mysql.Connection;
    static comic: Comic = new Comic();

    constructor() {
        const result = process.argv[2].split(",");
        Server.con = mysql.createConnection({host: result[0], user: result[1], password: result[2], database: result[3]});

        const app = express();
        app.use(express.json());
        app.use(cors({origin: `http://${result[4]}:4200`})); //Angular site
        app.listen(8000, result[4], () => Server.Writelog(`Listening on ${result[4]}:8000`));
        //  Comic
        app.route("/api/comic").post(async (req: any, res: any) => {
            try {
                await GetSetDelete.QueryDB("START TRANSACTION");
                await Server.comic.insertItem(req.body);
                await GetSetDelete.QueryDB("COMMIT");
            } catch (reason: any) {
                await GetSetDelete.QueryDB("ROLLBACK").catch(reason => res.sendStatus(reason));
                res.sendStatus(reason);
            }
        })
            .get(async (req: any, res: any) =>
                res.send(await Server.comic.getItems(req.query.columns, req.query.where, req.query.id, req.query.start, req.query.language)
                    .catch((reason: number) => res.sendStatus(reason))))
            .delete(async (req: any, res: any) => res.send(await Server.comic.deleteItem(req.query.id).catch(reason => res.sendStatus(reason))));

        //Language
        app.route("/api/language")
            .get(async (req: any, res: any) => res.send(await GetSetDelete.get("TLanguage", ["Language", "ColumnName"]).catch((reason: number) => res.sendStatus(reason))));
    }
    public static Writelog(log: string): void{
        const now: string = new Date(new Date().getTime() - (new Date().getTimezoneOffset() * 60 * 1000)).toISOString().slice(0, 19).replace('T', ' ');
        console.log(now +": " + log);
    }
}

new Server();