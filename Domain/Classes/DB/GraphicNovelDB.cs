using System.ComponentModel.DataAnnotations;
using Domain.Enums;
using Domain.Interfaces;

namespace Domain.Classes.DB;

public sealed class GraphicNovelDB: IEntity, IUserStats, IAdded
{
  public int Id { get; set; }
  public short? Chapters { get; set; }
  public short? Volumes { get; set; }
  public DateOnly? PublishStart { get; set; }
  public DateOnly? PublishEnd { get; set; }
  public int FKCover { get; set; }
  [Range(0.99,10.01)]
  public decimal Score { get; set; }
  public Status Status { get; set; }
  public DateOnly Added { get; set; }
  public int Popularity { get; set; }
  public int Favorites { get; set; }
  public int Members { get; set; }
  public int Rank { get; set; }
}
