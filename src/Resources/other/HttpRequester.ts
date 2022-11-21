import { HttpClient, HttpHeaders, HttpParams } from "@angular/common/http";
import { firstValueFrom } from "rxjs";

export abstract class HTTPRequester {

    static readonly url: string = "http://localhost:8000/";
    static httpHeaders: HttpHeaders;
    static httpClient: HttpClient;

    static start(httpClient: HttpClient) {
        HTTPRequester.httpClient = httpClient;
        HTTPRequester.httpHeaders = new HttpHeaders()
            .set('Content-Type', 'application/json')
            .set('Authorization', 'Basic QWxhZGRpb');
    }
    static async Get(address: string, params: HttpParams): Promise<any> {
        const options = { headers: this.httpHeaders, params: params };
        return await firstValueFrom(this.httpClient
            .get<any>(this.url + address, options));
    }
    static async Post(address: string, params: HttpParams, value: any): Promise<any> {
        const options = { headers: this.httpHeaders, params: params };
        return await firstValueFrom(this.httpClient
            .post<any>(this.url + address, value, options));

    }
    /*  static async Delete(address: string, params: HttpParams, value: any): Promise<any[]> {
          const options = { headers: this.httpHeaders, params: params };
          return await firstValueFrom(this.httpClient
              .delete<TDelete>(this.url + address, value, options));

      } */
    static async postTable(address: string, params: HttpParams, value: any): Promise<TObject> {
        const options = { headers: this.httpHeaders, params: params };
        return await firstValueFrom(this.httpClient
            .post<TObject>(this.url + address, value, options));

    }
    static toTObject(val: any[]): TObject {
        let columns: string[] = [];
        let rows: any[][] = [];
        const all = Object.entries(val);
        const firstentry = Object.entries(val[0]);
        //filter columns, so we don't have to send the keys with every value
        for (let i = 0; i < firstentry.length; i++) {
            columns.push(firstentry[i][0]);
        }
        for (let i = 0; i < all.length; i++) {
            const values = Object.entries(all[i][1]);
            let row: any[] = [];
            for (let j = 0; j < values.length; j++) {
                let val = values[j][1];
                if (typeof val === "string")
                    val = `'${val}'`;
                row.push(val);
            }
            rows.push(row);
        }
        return { column: columns, rows: rows };
    }
    static async getItems(address: string,table: string, where?: string, pk?: number[], start: number = 0, count: number = 50): Promise<any> {
        if (table == null)
            return;
        let params = new HttpParams()
            .set("table", table)
            .set("start", start)
            .set("count", count);
        if (where != null)
            params = params.set("where", where);
        if (pk != null)
            params = params.append("id", pk.join());
        return await HTTPRequester.Get("api" + address, params);
    }
}
export interface TObject {
    column: string[];
    rows: any[];
}
export interface TDelete {
    pkcolumnname: string;
    pk: number[];
}
export interface TGet {
    columns: string[];
    pk: number[];
}
