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

export class Server {
    static con;

    constructor() {

        //MySQL
        Server.con = mysql.createConnection({host: result[0], user: result[1], password: result[2], database: result[3]});
        app.use(express.json());
        app.use(cors({origin: `http://${result[4]}:4200`})); //Angular site
        app.listen(8000, result[4], () => console.log(`Listening on ${result[4]}:8000`));
        //Comic
        app.route("/api/comic").post(function (req) {
            Comic.insertComic(req.body.rows, req.query.language);
        }).get(async function (req, res) {
            res.send(await Comic.getComic(req.query.column, req.query.where, req.query.id, req.query.start, req.query.language));
        }).delete(function (req, res) {
            Comic.DeleteComic(res, req.query.column, req.query.id);
        });
        app.route("/api/profile").post(function (req, res) {
            if (req.query.update)
                updateComic(res, req.body.rows);
            else
                insertItems(res, req.body.rows);
        }).get(async function (req, res) {
            res.send(await Queries.getItems("TProfile", req.query.column, req.query.where, req.query.id, req.query.start));
        }).delete(function (req, res) {
            Comic.DeleteComic(res, req.query.column, req.query.id);
        });
        app.route("/api/manga").post(async function (req, res) {
            // if (req.query.update)
            //     manga.updateManga(res, req.body.rows);
            // else
            res.send(await Manga.insertManga(req.body.rows));
        }).get(async function (req, res) {
            res.send(await Manga.getManga(req.query.column, req.query.where, req.query.id, req.query.start));
        }).delete(function (req) {
            Manga.DeleteManga(req.query.column, req.query.id);
        });
        //other
        app.route("/api").post(function (req, res) {
            if (req.query.update)
                updateComic(res, req.body.rows);
            else
                insertItems(res, req.body.rows);
        }).get(async function (req, res) {
            res.send(await Queries.getItems(req.query.table, req.query.column, req.query.where, req.query.id, req.query.start));
        }).delete(function (req, res) {
            Comic.DeleteComic(res, req.query.column, req.query.id);
        });
    }

    static responseStatus(code) {
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
