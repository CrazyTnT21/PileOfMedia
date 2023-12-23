using System.ComponentModel.DataAnnotations;
using Domain.Interfaces;

namespace Domain.Classes.DB;

public sealed class GameDB: IEntity, IUserStats, IAdded
{
  public int Id { get; set; }
  public DateOnly? Published { get; set; }
  [Range(0.99, 10.01)] public decimal Score { get; set; }
  public int FKCover { get; set; }
  public DateOnly Added { get; set; }
  public int Popularity { get; set; }
  public int Favorites { get; set; }
  public int Members { get; set; }
  public int Rank { get; set; }
}
