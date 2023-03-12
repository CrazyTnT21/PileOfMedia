import {LanguageField} from "./schema";

export class BaseTable {
  pk?: number;
}

export class TranslationFields {
  languageFields?: LanguageField[];
}

export class Manga extends BaseTable implements TranslationFields {
  name: string;
  description?: string;
  publishStart?: Date;
  publishEnd?: Date;
  volumes?: number;
  chapters?: number;
  averageScore: number;
  status: string;
  imageSource?: string;
  added: string;
  languageFields?: LanguageField[];
  //Seperate queries
  genres?: string[];
  themes?: string[];
  creators?: string[];
}
export class Comic extends BaseTable implements TranslationFields {
  name: string;
  description?: string;
  publishStart?: Date;
  publishEnd?: Date;
  volumes?: number;
  chapters?: number;
  averageScore: number;
  status: string;
  imageSource?: string;
  added: string;
  languageFields?: LanguageField[];
  //Seperate queries
  genres?: string[];
  themes?: string[];
  creators?: string[];

  characters?: Character[];
}
export class Character extends BaseTable implements TranslationFields{

  name: string;
  firstname: string;
  lastname: string;
  description?: string;
  birthday?: Date;
  height?: number;
  imageSource?: string;
  languageFields?: LanguageField[];
}
