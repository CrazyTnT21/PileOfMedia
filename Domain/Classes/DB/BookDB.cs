using Domain.Interfaces;

namespace Domain.Classes.DB;

public sealed class BookDB : IEntity, IUserStats, IAdded
{
  public int Id { get; set; }
  public short? Chapters { get; set; }
  public short? Pages { get; set; }
  public int? Words { get; set; }
  public DateOnly? Published { get; set; }
  public int FKCover { get; set; }
  public decimal Score { get; set; }
  public int Popularity { get; set; }
  public int Favorites { get; set; }
  public int Members { get; set; }
  public int Rank { get; set; }
  public DateOnly Added { get; set; }
}
