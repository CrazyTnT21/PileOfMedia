using Domain.Enums;
using Domain.ValueObjects;

namespace Domain.Interfaces;

public interface ILanguageGet<T>
{
  public Task<IResult<IEnumerable<T>>> Get(Language language);
}
