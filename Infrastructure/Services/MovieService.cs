using Application.Repositories;
using Domain.Classes;
using Domain.Common;
using Domain.Enums;
using Domain.Error;
using Domain.Interfaces;
using Domain.Repositories;
using Domain.Services;
using Domain.ValueObjects;

namespace Infrastructure.Services;

public sealed class MovieService : Service, IMovieService
{
  private readonly IMovieRepository _repository;
  private readonly IImageService _imageService;
  private readonly IDateTimeProvider _dateTimeProvider;

  public MovieService(IMovieRepository repository, IImageService imageService, IDateTimeProvider dateTimeProvider)
  {
    _repository = repository;
    _imageService = imageService;
    _dateTimeProvider = dateTimeProvider;
  }

  public async Task<IResult<Movie?>> GetById(uint id, Language language)
  {
    return new Ok<Movie?>(await _repository.GetById(id, language));
  }

  public async Task<IResult<IEnumerable<Movie>>> GetPreview(Language language)
  {
    return new Ok<IEnumerable<Movie>>(await _repository.Get(language));
  }

  public async Task<IResult<IEnumerable<Movie>>> Get(Language language)
  {
    var result = await _repository.Get(language);
    return new Ok<IEnumerable<Movie>>(result);
  }

  public async Task<IResult> RemoveGenresFromMovie(uint id, uint genres)
  {
    return new Ok();
  }

  public async Task<IResult> RemoveThemesFromMovie(uint id, uint themes)
  {
    return new Ok();
  }

  public async Task<IResult> DeleteTranslations(uint id, List<Language> translations)
  {
    return new Ok();
  }

  public async Task<IResult<Movie>> Create(CreateMovie item)
  {
    return ValidateCreate(item) switch
    {
      Fail fail => new Fail<Movie>(fail),
      Ok => await _imageService.Create(item.Cover!) switch
      {
        Fail<Image> fail => new Fail<Movie>(fail),
        Ok<Image> image => new Ok<Movie>(await Create(item, image)),
        _ => throw new ArgumentException()
      },
      _ => throw new ArgumentException()
    };
  }

  private async Task<Movie> Create(CreateMovie item, Image image)
  {
    item.Cover!.Id = image.Id;

    return await _repository.Create(item);
  }

  public async Task<IResult<Movie>> Update(CreateMovie item)
  {
    //TODO: Should complex nested items be updated separately?
    //TODO: How to handle values being set to null

    return ValidateUpdate(item) switch
    {
      Fail fail => new Fail<Movie>(fail),
      Ok => new Ok<Movie>(await _repository.Update(item)),
      _ => throw new ArgumentException()
    };
  }

  public async Task<IResult> Delete(uint id)
  {
    var userCount = (await _repository.GetUserIdsByMovie(id)).Count();
    if (userCount > 0)
      return new Fail(new ErrorMessage($"Movie has already been added by '{userCount}' users and cannot be deleted"));

    await _repository.Delete(id);
    return new Ok();
  }

  public IResult ValidateCreate(CreateMovie item)
  {
    if (item.Id != 0)
      return new Fail(new ErrorMessage("Id cannot have an existing value"));

    if (item.Cover is null)
      return new Fail(new MissingValueErrorMessage("Cover is missing"));

    var result = _imageService.Validate(item.Cover);
    if (result.Failure)
      return result;

    if (item.Translations is null)
      return new Fail(new MissingValueErrorMessage("Translations are missing"));

    var validTranslations = ValidateTranslations(item.Translations, true);

    if (validTranslations.Failure)
      return validTranslations;

    return Validate(item);
  }

  public IResult ValidateUpdate(CreateMovie item)
  {
    if (item.Cover is not null)
    {
      var result = _imageService.Validate(item.Cover);
      if (result.Failure)
        return result;
    }

    if (item.Translations is not null)
    {
      var validTranslations = ValidateTranslations(item.Translations, false);

      if (validTranslations.Failure)
        return validTranslations;
    }

    return Validate(item);
  }

  private IResult Validate(CreateMovie item)
  {
    if (item.Airing is not null)
    {
      if (_dateTimeProvider.Today.AddYears(5) > item.Airing)
        return new Fail(new ErrorMessage("Dates 5 years later will not be added"));
    }

    if (item.Genres is not null)
    {
      for (int i = 0; i < item.Genres.Length; i++)
      {
        if (item.Genres[i].Id < 1)
          return new Fail(new ErrorMessage(
                            $"Item contains a genre with an invalid id value of '{item.Genres[i].Id}' at position '{i}'"));
      }
    }

    if (item.Themes is not null)
    {
      for (int i = 0; i < item.Themes.Length; i++)
      {
        if (item.Themes[i].Id < 1)
          return new Fail(new ErrorMessage(
                            $"Item contains a theme with an invalid id value of '{item.Themes[i].Id}' at position '{i}'"));
      }
    }

    return new Ok();
  }

  private IResult ValidateTranslations(Dictionary<Language, TitleDescription> translations, bool requireEnglishTitle)
  {
    bool englishAvailable = false;
    foreach (var translation in translations)
    {
      if (translation.Key == Language.EN)
        englishAvailable = true;

      const int titleMaxLength = 150;
      if (translation.Value.Title.Length > titleMaxLength)
        return new Fail(new ErrorMessage(
                          $"Translations value for language '{translation.Key}' has an title with more than {titleMaxLength} characters"));

      const int descriptionMaxLength = 500;
      if (translation.Value.Description?.Length > descriptionMaxLength)
        return new Fail(new ErrorMessage(
                          $"Translations value for language '{translation.Key}' has an description with more than {descriptionMaxLength} characters"));
    }

    if (requireEnglishTitle && !englishAvailable)
      return new Fail(new MissingValueErrorMessage("An english translation for the title was not found"));

    return new Ok();
  }
}
