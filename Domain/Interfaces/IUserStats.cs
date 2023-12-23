namespace Domain.Interfaces;

public interface IUserStats : IRank, IScore
{
  public int Popularity { get; set; }
  public int Favorites { get; set; }
  public int Members { get; set; }
}

public interface IRank
{
  public int Rank { get; set; }
}

public interface IScore
{
  public decimal Score { get; set; }
}
