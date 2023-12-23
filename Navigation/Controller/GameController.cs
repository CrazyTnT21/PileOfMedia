using Domain.Classes;
using Domain.Services;
using Microsoft.AspNetCore.Mvc;
using MyCollectionServer.Controller.Base;

namespace MyCollectionServer.Controller;

[ApiController]
[Route("Games")]
public sealed class GameController : APIController
{
  private readonly IGameService _service;

  public GameController(IGameService service, ILogger logger) : base(logger)
  {
    _service = service;
  }

  [HttpGet("{id}")]
  public async Task<ActionResult<Game>> GetById(uint id, string? language)
  {
    return await GenericLanguageGetById(id, language, _service);
  }

  [HttpGet]
  public async Task<ActionResult<IEnumerable<Game>>> Get(string? language)
  {
    return await GenericLanguageGet(language, _service);
  }
}
