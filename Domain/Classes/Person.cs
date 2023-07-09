using System;
using Domain.Attributes;
using Domain.Interfaces;
using Domain.Schemas;

namespace Domain.Classes;

[DBTable<Person>(PersonSchema.Table)]
public class Person : ITranslationFields
{
  [DBColumn(PersonSchema.Id)] public uint PK { get; set; }
  [DBColumn(PersonSchema.Name)] public string Name { get; set; }
  [DBColumn(PersonSchema.FirstName)] public string FirstName { get; set; }
  [DBColumn(PersonSchema.LastName)] public string LastName { get; set; }

  [DBColumn(PersonSchema.Description, TranslationSchema.Table)]
  public string? Description { get; set; }

  [DBColumn(PersonSchema.Birthday)] public DateOnly? Birthday { get; set; }
  [DBColumn(PersonSchema.Height)] public byte? Height { get; set; }

  [DBColumn(PersonSchema.ImageSource)] public string? ImageSource { get; set; }

  public TranslationField[]? TranslationFields { get; set; }
}
