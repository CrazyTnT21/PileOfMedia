using System.ComponentModel.DataAnnotations;
using System.ComponentModel.DataAnnotations.Schema;
using Domain.Interfaces;

namespace Domain.Classes;

[Table("Translation")]
public sealed class Translations : IEntity
{
  [DatabaseGenerated(DatabaseGeneratedOption.Identity)]
  [Key]
  [Column("Id")]
  public uint Id { get; set; }

  [Column] public string EN { get; set; }
  [Column] public string DE { get; set; }
  [Column] public string ES { get; set; }
  [Column] public string JA { get; set; }
  [Column] public string NL { get; set; }
}
