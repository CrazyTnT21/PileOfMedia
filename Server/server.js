//Require
import mysql from "mysql2";
import express from "express";

const app = express();
import cors from 'cors';
import Queries from "./queries.js";
import Comic from "./comic.js";
import Manga from "./manga.js";
//
//(GET, INSERT, UPDATE) query.id == where values (example: where FKStatus = (.id.join()))
//(GET,INSERT,UPDATE) query.column == selected columns (example1: select .column.join() from Tcomic, example2: insert into TComic(.column.join()) values(1,2,3))
//(GET,INSERT,UPDATE) query.where == where columns
//(GET) query.start == how many items to get (start will be multiplied by 50 to represent a page)
//(INSERT, UPDATE) .body.rows == all the items without the keys in an jagged array [][]

const result = process.argv[2].split(",");

class Server {
    constructor() {

        //MySQL
        const con = mysql.createConnection({
            host: result[0], user: result[1], password: result[2], database: result[3], dateStrings: false
        });
        const queries = new Queries(this, con)
        const comic = new Comic(queries);
        const manga = new Manga(queries);
        app.use(express.json());
        app.use(cors({origin: 'http://localhost:4200'})); //Angular site
        app.listen(8000, "localhost", () => console.log(`Listening on localhost:8000`));
        //Comic
        app.route("/api/comic").post(function (req, res) {
                comic.insertComic(req.body.rows);
        }).get(async function (req, res) {
            res.send(await comic.getComic(req.query.column, req.query.where, req.query.id, req.query.start, req.query.language));
        }).delete(function (req, res) {
            comic.DeleteComic(res, req.query.column, req.query.id);
        });
        app.route("/api/profile").post(function (req, res) {
            if (req.query.update)
                updateComic(res, req.body.rows);
            else
                insertItems(res, req.body.rows);
        }).get(async function (req, res) {
            res.send(await queries.getItems("TProfile", req.query.column, req.query.where, req.query.id, req.query.start));
        }).delete(function (req, res) {
            comic.DeleteComic(res, req.query.column, req.query.id);
        });
        app.route("/api/manga").post(async function (req, res) {
            // if (req.query.update)
            //     manga.updateManga(res, req.body.rows);
            // else
           res.send(await manga.insertManga(req.body.rows));
        }).get(async function (req, res) {
            res.send(await manga.getManga(req.query.column, req.query.where, req.query.id, req.query.start));
        }).delete(function (req, res) {
            manga.DeleteManga(req.query.column, req.query.id);
        });
        //other
        app.route("/api").post(function (req, res) {
            if (req.query.update)
                updateComic(res, req.body.rows);
            else
                insertItems(res, req.body.rows);
        }).get(async function (req, res) {
            res.send(await queries.getItems(req.query.table, req.query.column, req.query.where, req.query.id, req.query.start));
        }).delete(function (req, res) {
            comic.DeleteComic(res, req.query.column, req.query.id);
        });
    }

    responseStatus(code) {
        switch (code) {
            case 400:
                return "400 Bad Request";
            case 500:
                return "500 Server fucked up";
            default:
                break;
        }
    }
}

new Server();
