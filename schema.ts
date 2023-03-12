//Non-existent fields - may only exist during sending or in the client
//Client-Only fields - only exists in the client

import {BaseTable} from "./tables";

export interface TranslateFields {
  languageFields?: LanguageField[];
}
export class UserStats {
  liked?: number;
  followed?: number;
}

export class ItemStats {
  status?: string;
  genres?: string[];
  themes?: string[];
}

export class ComicXCreator extends BaseTable {
  fkPerson?: number;
  fkRole?: number;
  role?: string;
  person: TPerson;
}

export class TPerson extends BaseTable {
  name?: string;
  firstName: string;
  lastName: string;
  fkDescription?: number;
  birthday?: Date;
  height?: number;
  imageSource?: string;
}

export interface LanguageField {
  values: translation[];
  column: string;
  bindProperty: string;
}

export interface translation {
  language: string;
  value: string;
}

export class Languages {
  public static readonly English: string = "EN";
  public static readonly German: string = "German";
  public static readonly Spanish: string = "Spanish";
  public static readonly Japanese: string = "Japanese";
  public static readonly Italian: string = "Italian";
  public static readonly Korean: string = "Korean";
  public static readonly French: string = "French";
}
