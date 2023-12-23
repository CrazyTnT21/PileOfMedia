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
[Route("GraphicNovels")]
public sealed class GraphicNovelController : APIController
{
  private readonly IGraphicNovelService _service;

  public GraphicNovelController(IGraphicNovelService service, ILogger logger) : base(logger)
  {
    _service = service;
  }

  [HttpGet]
  public async Task<ActionResult<IEnumerable<GraphicNovel>>> Get(string? language)
  {
    return await GenericLanguageGet(language, _service);
  }

  [HttpGet("{id}")]
  public async Task<ActionResult<GraphicNovel>> GetById(uint id, string? language)
  {
    return await GenericLanguageGetById(id, language, _service);
  }


  [HttpPost]
  public async Task<ActionResult<GraphicNovel>> Create(CreateGraphicNovel item)
  {
    IResult<GraphicNovel> result = await _service.Create(item);

    return result switch
    {
      Fail<GraphicNovel> fail => new BadRequestResult(),
      Ok<GraphicNovel> graphicNovel => graphicNovel.Value,
      _ => new StatusCodeResult(500)
    };
  }

  [HttpPut]
  public async Task<ActionResult<GraphicNovel>> Update(CreateGraphicNovel item)
  {
    IResult<GraphicNovel> result = await _service.Update(item);
    return result switch
    {
      Fail<GraphicNovel> fail => new BadRequestResult(),
      Ok<GraphicNovel> graphicNovel => graphicNovel.Value,
      _ => new StatusCodeResult(500)
    };
  }

  [HttpDelete("{id}")]
  public async Task<IActionResult> Delete(uint id)
  {
    await _service.Delete(id);
    return new OkResult();
  }
}
