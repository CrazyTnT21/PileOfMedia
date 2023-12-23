using Domain.Classes;
using Domain.Enums;
using Domain.Interfaces;
using Domain.ValueObjects;

namespace Domain.Services;

public interface IGameService: ILanguageGetById<Game>, ILanguageGet<Game>
{
  public Task<IResult<Game>> Create(CreateGame item);
  public Task<IResult<Game>> Update(CreateGame item);
  public Task<IResult> Delete(uint id);
  public IResult Validate(CreateGame item);
}
