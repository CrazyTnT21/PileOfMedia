import {Router} from "@angular/router";

export class Tools {

    public static router: Router;

    public static getdays(startdate: Date, enddate: Date): number {
        let days = enddate.getTime() - startdate.getTime();
        return Math.round(days / 1000 / 60 / 60 / 24) + 1;
    }

    public static convertdate(date?: Date): string  {
        return date ? new Date(date).toISOString().slice(0, 10).toString() : "";
    }

    public static navigatePage(page: string, id?: number, params?: any) {
        let commands: any = [page];
        if (id)
            commands.push(id);
        Tools.router.navigate(commands, {queryParams: params});
    }
    public static linkText(text: string): string{
      return text.replace(new RegExp("test([\\s\\S]*?)1"),"<a href='testlink'></a>");
  }
}
