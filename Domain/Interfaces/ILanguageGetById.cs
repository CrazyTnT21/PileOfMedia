using Domain.Enums;
using Domain.ValueObjects;

namespace Domain.Interfaces;

public interface ILanguageGetById<T>
{
  public Task<IResult<T?>> GetById(uint id, Language language);
}
