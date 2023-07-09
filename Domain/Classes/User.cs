using System;
using Domain.Attributes;
using Domain.Schemas;

namespace Domain.Classes;

public sealed class User
{
  [DBColumn(UserSchema.Id)] public uint PK { get; set; }
  [DBColumn(UserSchema.Name)] public string Name { get; set; }
  [DBColumn(UserSchema.Joined)] public DateOnly Joined { get; set; }
  [DBColumn(UserSchema.Description)] public string? Description { get; set; }
  [DBColumn(UserSchema.ImageSource)] public string? ImageSource { get; set; }
}
