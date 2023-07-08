using System.ComponentModel.DataAnnotations;
using System.ComponentModel.DataAnnotations.Schema;
using Domain.Attributes;
using Domain.Enums;
using Domain.Interfaces;

namespace Domain.Classes;

[Table("Status")]
public sealed class Status : IEntity
{
  [Key] [Column("Id")] public uint Id { get; set; }

  [DBJoin(typeof(Translations), "FKStatus")]
  [DBAssociation(Key.Language)]
  public string Name { get; set; } = null!;

  [Column("Status")] public StatusEnum Value { get; set; }
}
