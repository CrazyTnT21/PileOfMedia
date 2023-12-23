using Application.Repositories;
using Domain.Classes;
using Domain.Enums;
using Domain.Error;
using Domain.Services;
using Domain.ValueObjects;

namespace Infrastructure.Services;

public sealed class GameService : Service, IGameService
{
  private readonly IGameRepository _repository;
  private readonly IImageService _imageService;

  public GameService(IGameRepository repository, IImageService imageService)
  {
    _repository = repository;
    _imageService = imageService;
  }

  public async Task<IResult<Game?>> GetById(uint id, Language language)
  {
    return new Ok<Game?>(await _repository.GetById(id, language));
  }

  public async Task<IResult<IEnumerable<Game>>> Get(Language language)
  {
    var result = await _repository.Get(language);
    return new Ok<IEnumerable<Game?>>(result);
  }

  public async Task<IResult<Game>> Create(CreateGame item)
  {
    Validate(item);
    return Validate(item) switch
    {
      Fail fail => new Fail<Game>(fail),
      Ok => await _imageService.Create(item.Cover!) switch
      {
        Fail<Image> fail => new Fail<Game>(fail),
        Ok<Image> image => new Ok<Game>(await Create(item, image)),
        _ => throw new ArgumentException()
      },
      _ => throw new ArgumentException()
    };
  }

  public async Task<Game> Create(CreateGame item, Image image)
  {
    item.Cover!.Id = image.Id;

    return await _repository.Create(item);
  }

  public async Task<IResult<Game>> Update(CreateGame item)
  {
    Validate(item);

    //Remove old image and upload new image
    // if (item.Cover is not null)
    // {
    //   IResult<Image> image = await _imageService.Update(item.Cover);
    //   if (image.Failed)
    //     return image.Exception;
    //
    //   item.Cover.Id = image.Value.Id;
    // }

    return new Ok<Game>(await _repository.Create(item));
  }

  public async Task<IResult> Delete(uint id)
  {
    //Delete From Translation
    //Delete From GameXCharacter
    //Delete From GameXGenre
    //Delete From GameXTheme
    //Delete From GameXCreator
    //Delete From UserXGame
    //Delete
    throw new NotImplementedException();
  }

  public IResult Validate(CreateGame item)
  {
    if (item.Id < 1)
    {
      if (item.Cover is null)
        return new Fail(new MissingValueErrorMessage("Cover is missing"));

      IResult result = _imageService.Validate(item.Cover);
      if (result.Failure)
      {
        return result;
      }
    }

    return new Ok();
  }
}
