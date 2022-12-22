import { Pipe, PipeTransform } from '@angular/core';
/*
 * Formats the value using the given function and formatting
 * e.g:
 *
 * formatting = Chapters: {}
 * rowvalue = 12
 * result = Chapters: 12
 * Usage:
 *   value | formatting:formatting
 * Example:
 *   {{ 2 | exponentialStrength:10 }}
 *   formats to: 1024
*/
@Pipe({name: 'Formatting'})
export class FormattingPipe implements PipeTransform {

    transform(value: any, format: string): any {
        if (format && value != undefined)
            return format.replace("{}", value);
        return value;
    }
}