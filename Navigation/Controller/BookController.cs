using Domain.Classes;
using Domain.Common;
using Domain.Enums;
using Domain.Services;
using Domain.ValueObjects;
using Microsoft.AspNetCore.Mvc;
using MyCollectionServer.Controller.Base;

namespace MyCollectionServer.Controller;

[ApiController]
[Route("Books")]
public sealed class BookController : APIController
{
  private readonly IBookService _service;

  public BookController(IBookService service, ILogger logger) : base(logger)
  {
    _service = service;
  }

  [HttpGet]
  public async Task<ActionResult<IEnumerable<Book>>> Get(string? title, string? language)
  {
    IResult<Language> languageResult = LanguageParser.GetLanguage(language);

    return languageResult switch
    {
      Ok<Language> lang => title is null ? await Get(lang) : await GetByTitle(title, lang),
      Fail<Language> fail => new BadRequestResult(),
      _ => new StatusCodeResult(500)
    };
  }

  public async Task<ActionResult<IEnumerable<Book>>> Get(Language language)
  {
    return await _service.Get(language) switch
    {
      Ok<IEnumerable<Book>> books => books.Value.ToList(),
      Fail<IEnumerable<Book>> fail => new BadRequestResult(),
      _ => new StatusCodeResult(500)
    };
  }

  public async Task<ActionResult<IEnumerable<Book>>> GetByTitle(string title, Language language)
  {
    return await _service.GetByTitle(title, language) switch
    {
      Ok<IEnumerable<Book>> books => books.Value.ToList(),
      Fail<IEnumerable<Book>> fail => new BadRequestResult(),
      _ => new StatusCodeResult(500)
    };
  }
}
