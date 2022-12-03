import { Router } from "@angular/router";

export class Tools {

    public static router: Router;
    public static isNullorZero(value: undefined | number): boolean {
        return value == null || value == 0;
    }
    public static getdays(startdate: Date, enddate: Date): number {
        let days = enddate.getTime() - startdate.getTime();
        return Math.round(days / 1000 / 60 / 60 / 24) + 1;
    }
    public static convertdate(date: Date): string {
        return new Date(date).toISOString().slice(0, 10).toString();
    }

    public static navigatePage(page: string,id?: number, params?: any) {
        let commands: any = [page];
        if (id)
            commands.push(id);
        Tools.router.navigate(commands, { queryParams: params });
    }
    public static createurl(page: string, params?: any): string {
        if (params.length > 0) {
            page += "?";
            page += params[0][0] + "=" + params[0][1];
            for (let i = 1; i < params.length; i++) {
                page += "&&" + params[i][0] + "=" + params[i][1];
            }
        }
        return page;
    }
    public static GetComicStatus(pk: number): string {
        switch (pk) {
            case 1:
                return "Not yet started";
            case 2:
                return "Publishing";
            case 3:
                return "Finished";


            default:
                return "unkown"

        }
    }
}