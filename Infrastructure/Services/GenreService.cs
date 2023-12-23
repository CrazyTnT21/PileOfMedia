using Application.Repositories;
using Domain.Classes;
using Domain.Enums;
using Domain.Services;
using Domain.ValueObjects;

namespace Infrastructure.Services;

public sealed class GenreService : Service, IGenreService
{
  private readonly IGenreRepository _repository;

  public GenreService(IGenreRepository repository)
  {
    _repository = repository;
  }

  public async Task<IResult<Genre?>> GetById(uint id, Language language)
  {
    return new Ok<Genre?>(await _repository.GetById(id, language));
  }

  public async Task<IResult<Genre?>> GetByName(string name, Language language)
  {
    return new Ok<Genre?>(await _repository.GetByName(name, language));
  }

  public async Task<IResult<IEnumerable<Genre>>> Get(Language language)
  {
    var result = await _repository.Get(language);
    return new Ok<IEnumerable<Genre>>(result);
  }

  public async Task<IResult<Genre>> Create(CreateGenre item)
  {
    return new Ok<Genre>(await _repository.Create(item));
  }

  public async Task<IResult<Genre>> Update(CreateGenre item)
  {
    return new Ok<Genre>(await _repository.Update(item));
  }

  public async Task<IResult> Delete(uint id)
  {
    await _repository.Delete(id);
    return new Ok();
  }

  public IResult Validate(CreateGenre item)
  {
    return new Ok();
  }
}
