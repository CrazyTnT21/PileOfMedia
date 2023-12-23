using Domain.Classes;
using Domain.Common;
using Domain.Enums;
using Domain.Services;
using Domain.ValueObjects;
using Infrastructure.Services;
using Microsoft.AspNetCore.Mvc;
using MyCollectionServer.Controller.Base;

namespace MyCollectionServer.Controller;

[ApiController]
[Route("Genres")]
public sealed class GenreController : APIController
{
  private readonly IGenreService _service;

  public GenreController(IGenreService service, ILogger logger) : base(logger)
  {
    _service = service;
  }

  [HttpGet("{id}")]
  public async Task<ActionResult<Genre>> GetById(uint id, string? language)
  {
    return await GenericLanguageGetById(id, language, _service);
  }

  [HttpGet]
  public async Task<ActionResult<IEnumerable<Genre>>> Get(string? language)
  {
    return await GenericLanguageGet(language, _service);
  }

  [HttpGet("name/{name}")]
  public async Task<ActionResult<Genre?>> GetByName(string name, string? language)
  {
    return LanguageParser.GetLanguage(language) switch
    {
      Fail<Language> => new BadRequestResult(),
      Ok<Language> lang => await _service.GetByName(name, lang) switch
      {
        Fail<Genre?> fail => new BadRequestResult(),
        Ok<Genre?> genre => genre.Value is null ? new NotFoundResult() : genre.Value,
        _ => new StatusCodeResult(500)
      },
      _ => new StatusCodeResult(500)
    };
  }
}
