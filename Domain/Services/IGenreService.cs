using Domain.Classes;
using Domain.Enums;
using Domain.Interfaces;
using Domain.ValueObjects;

namespace Domain.Services;

public interface IGenreService: ILanguageGet<Genre>, ILanguageGetById<Genre>
{
  public Task<IResult<Genre?>> GetByName(string name, Language language);
  public Task<IResult<Genre>> Create(CreateGenre item);
  public Task<IResult<Genre>> Update(CreateGenre item);
  public Task<IResult> Delete(uint id);
  public IResult Validate(CreateGenre item);
}
