using Domain.Attributes;
using Domain.Interfaces;
using Domain.Schemas;

namespace Domain.Classes;

[DBTable<Character>(CharacterSchema.Table)]
public sealed class Character : ITranslationFields
{
  [DBColumn(CharacterSchema.Id)] public uint PK { get; set; }

  [DBColumn(CharacterSchema.Name, TranslationSchema.Table)]
  public string Name { get; set; }

  [DBColumn(CharacterSchema.FirstName, TranslationSchema.Table)]
  public string FirstName { get; set; }

  [DBColumn(CharacterSchema.LastName, TranslationSchema.Table)]
  public string LastName { get; set; }

  [DBColumn(CharacterSchema.Description, TranslationSchema.Table)]
  public string? Description { get; set; }

  [DBColumn(CharacterSchema.Birthday)] public DateOnly? Birthday { get; set; }
  [DBColumn(CharacterSchema.Height)] public uint? Height { get; set; }

  [DBColumn(CharacterSchema.ImageSource)]
  public string? ImageSource { get; set; }

  public TranslationField[]? TranslationFields { get; set; }
}
