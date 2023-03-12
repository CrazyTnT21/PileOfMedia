import * as StringNames from "../Resources/StringNames.json";
import {Tools} from "./Tools";

export class Base {
  public error: boolean;
  public static language: string = "EN";
  public static languages: any[] = [];
  public get languages(): any[] {
    return Base.languages;
  }

  public get language(): String {
    return Base.language;
  }

  public StringNames = StringNames;
  public Tools = Tools;
}
