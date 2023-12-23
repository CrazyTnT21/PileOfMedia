using Domain.Enums;

namespace Domain.Classes.DB;

public interface ITranslationDB
{
  public int FKTranslation { get; set; }
  public Language Language { get; set; }
}
