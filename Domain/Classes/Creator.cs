using Domain.Attributes;
using Domain.Schemas;

namespace Domain.Classes;

[DBTable<Person>(PersonSchema.Table)]
public sealed class Creator : Person
{
  [DBColumn("Role", TranslationSchema.Table)]
  public string? Role { get; set; }
}
