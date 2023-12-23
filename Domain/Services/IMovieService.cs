using Domain.Classes;
using Domain.Enums;
using Domain.Interfaces;
using Domain.ValueObjects;

namespace Domain.Services;

public interface IMovieService: ILanguageGetById<Movie>,ILanguageGet<Movie>
{
  public Task<IResult<Movie>> Create(CreateMovie item);
  public Task<IResult<Movie>> Update(CreateMovie item);
  public Task<IResult> Delete(uint id);
  public IResult ValidateCreate(CreateMovie item);
  public IResult ValidateUpdate(CreateMovie item);
}
