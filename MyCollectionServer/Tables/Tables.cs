using MySqlConnector;

namespace MyCollectionServer.Tables;


[DBTable<Manga>(MangaSchema.table)]
public sealed class Manga : PKClass, languageFields
{
  [DBColumn(MangaSchema.pk)] public uint PK { get; set; }

  [DBColumn(MangaSchema.name, TranslationSchema.table, TranslationSchema.pk)]
  public string Name { get; set; }

  [DBColumn(MangaSchema.description, TranslationSchema.table, TranslationSchema.pk)]
  public string? Description { get; set; }

  [DBColumn(MangaSchema.status, TranslationSchema.table, TranslationSchema.pk)]
  public string? Status { get; set; }

  [DBColumn(MangaSchema.chapters)] public ushort? Chapters { get; set; }
  [DBColumn(MangaSchema.volumes)] public ushort? Volumes { get; set; }
  [DBColumn(MangaSchema.publishStart)] public DateOnly? PublishStart { get; set; }
  [DBColumn(MangaSchema.publishEnd)] public DateOnly? PublishEnd { get; set; }
  [DBColumn(MangaSchema.imageSource)] public string? ImageSource { get; set; }
  [DBColumn(MangaSchema.averageScore)] public MySqlDecimal? AverageScore { get; set; }
  public LanguageField[]? LanguageFields { get; set; }
}

[DBTable<Character>(CharacterSchema.table)]
public sealed class Character : PKClass, languageFields
{
  [DBColumn(CharacterSchema.pk)] public uint PK { get; set; }

  [DBColumn(CharacterSchema.name, TranslationSchema.table, TranslationSchema.pk)]
  public string Name { get; set; }

  [DBColumn(CharacterSchema.firstName, TranslationSchema.table, TranslationSchema.pk)]
  public string FirstName { get; set; }

  [DBColumn(CharacterSchema.lastName, TranslationSchema.table, TranslationSchema.pk)]
  public string LastName { get; set; }

  [DBColumn(CharacterSchema.description, TranslationSchema.table, TranslationSchema.pk)]
  public string? Description { get; set; }

  [DBColumn(CharacterSchema.birthday)] public DateOnly? Birthday { get; set; }
  [DBColumn(CharacterSchema.height)] public uint? Height { get; set; }

  [DBColumn(CharacterSchema.imageSource)]
  public string? ImageSource { get; set; }

  public LanguageField[]? LanguageFields { get; set; }
}

[DBTable<Account>(AccountSchema.table)]
public sealed class Account
{
  [DBColumn(AccountSchema.fkuser, UserSchema.table, UserSchema.pk)]
  // public User User { get; set; }
  public uint FKUser { get; set; }

  [DBColumn(AccountSchema.email)] public string Email { get; set; }
  [DBColumn(AccountSchema.password)] public string Password { get; set; }
}

public sealed class User : PKClass
{
  [DBColumn(UserSchema.pk)] public uint PK { get; set; }
  [DBColumn(UserSchema.name)] public string Name { get; set; }
  [DBColumn(UserSchema.joined)] public DateOnly Joined { get; set; }
  [DBColumn(UserSchema.description)] public string? Description { get; set; }
  [DBColumn(UserSchema.imageSource)] public string? ImageSource { get; set; }
}

[DBTable<Game>(GameSchema.table)]
public sealed class Game : PKClass
{
  [DBColumn(GameSchema.pk)] public uint PK { get; set; }
  [DBColumn(GameSchema.name)] public string Name { get; set; }
  [DBColumn(GameSchema.description)] public string? Description { get; set; }
  [DBColumn(GameSchema.published)] public DateOnly? Published { get; set; }
  [DBColumn(GameSchema.averageScore)] public decimal? AverageScore { get; set; }
  [DBColumn(GameSchema.imageSource)] public string? ImageSource { get; set; }

  [DBColumn(GameSchema.status, TranslationSchema.table, TranslationSchema.pk)]
  public string? Status { get; set; }
}

[DBTable<Person>(PersonSchema.table)]
public class Person : PKClass, languageFields
{
  [DBColumn(PersonSchema.pk)] public uint PK { get; set; }
  [DBColumn(PersonSchema.name)] public string Name { get; set; }
  [DBColumn(PersonSchema.firstName)] public string FirstName { get; set; }
  [DBColumn(PersonSchema.lastName)] public string LastName { get; set; }

  [DBColumn(PersonSchema.description, TranslationSchema.table, TranslationSchema.pk)]
  public string? Description { get; set; }

  [DBColumn(PersonSchema.birthday)] public DateOnly? Birthday { get; set; }
  [DBColumn(PersonSchema.height)] public byte? Height { get; set; }

  [DBColumn(PersonSchema.imageSource)] public string? ImageSource { get; set; }

  public LanguageField[]? LanguageFields { get; set; }
}

[DBTable<Person>(PersonSchema.table)]
public sealed class Creator : Person
{
  [DBColumn("Role", TranslationSchema.table, TranslationSchema.pk)]
  public string? Role { get; set; }
}
