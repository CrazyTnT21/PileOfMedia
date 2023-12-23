using Domain.Common;
using Domain.Enums;
using Domain.Interfaces;
using Domain.ValueObjects;
using Microsoft.AspNetCore.Mvc;

namespace MyCollectionServer.Controller.Base;

[ApiController]
[Route("[controller]")]
public abstract class APIController
{
  protected readonly ILogger _logger;

  protected APIController(ILogger logger)
  {
    _logger = logger;
  }

  public async Task<ActionResult<T>> GenericLanguageGetById<T>(uint id, string? language, ILanguageGetById<T> service)
  {
    var languageResult = LanguageParser.GetLanguage(language);

    if (languageResult is Fail<Language> languageFail)
      return new BadRequestResult();

    Language lang = (Ok<Language>)languageResult;

    var result = await service.GetById(id, lang);

    if (result is Fail<T?> failResult)
      return new BadRequestResult();

    T? item = (Ok<T?>)result;

    if (item is null)
      return new NotFoundResult();

    return item;
  }

  public async Task<ActionResult<IEnumerable<T>>> GenericLanguageGet<T>(string? language, ILanguageGet<T> service)
  {
    return LanguageParser.GetLanguage(language) switch
    {
      Fail<Language> => new BadRequestResult(),
      Ok<Language> lang => await service.Get(lang) switch
      {
        Fail<IEnumerable<T>> fail => new BadRequestResult(),
        Ok<IEnumerable<T>> items => items.Value.ToList(),
        _ => new StatusCodeResult(500)
      },
      _ => new StatusCodeResult(500)
    };
  }
}
