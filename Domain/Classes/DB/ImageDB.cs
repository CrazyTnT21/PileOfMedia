using System.ComponentModel.DataAnnotations;
using Domain.Interfaces;

namespace Domain.Classes.DB;

public sealed class ImageDB: IEntity
{
  public int Id { get; set; }
  [StringLength(2047)]
  public string Uri { get; set; } = null!;
  public short Width { get; set; }
  public short Height { get; set; }
  [StringLength(15)]
  public string? Extension { get; set; }
}
