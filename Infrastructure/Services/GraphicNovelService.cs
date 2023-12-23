using Application.Repositories;
using Domain.Classes;
using Domain.Enums;
using Domain.Services;
using Domain.ValueObjects;

namespace Infrastructure.Services;

public sealed class GraphicNovelService : Service, IGraphicNovelService
{
  private readonly IGraphicNovelRepository _repository;
  private readonly IImageService _imageService;

  public GraphicNovelService(IGraphicNovelRepository repository, IImageService imageService)
  {
    _repository = repository;
    _imageService = imageService;
  }

  public async Task<IResult<GraphicNovel?>> GetById(uint id, Language language)
  {
    return new Ok<GraphicNovel?>(await _repository.GetById(id, language));
  }

  public async Task<IResult<IEnumerable<GraphicNovel>>> Get(Language language)
  {
    return new Ok<IEnumerable<GraphicNovel>>(await _repository.Get(language));
  }

  public async Task<IResult<GraphicNovel>> Create(CreateGraphicNovel item)
  {
    return Validate(item) switch
    {
      Fail fail => new Fail<GraphicNovel>(fail),
      Ok => new Ok<GraphicNovel>(await _repository.Create(item)),
      _ => throw new ArgumentException()
    };
  }

  public async Task<IResult<GraphicNovel>> Update(CreateImage item)
  {
    throw new NotImplementedException();
  }

  public async Task<IResult<GraphicNovel>> Update(CreateGraphicNovel item)
  {
    return Validate(item) switch
    {
      Fail fail => new Fail<GraphicNovel>(fail),
      Ok => new Ok<GraphicNovel>(await _repository.Update(item)),
      _ => throw new ArgumentException()
    };
  }

  public async Task<IResult> Delete(uint id)
  {
    await _repository.Delete(id);
    return new Ok();
  }

  public IResult Validate(CreateGraphicNovel item)
  {
    IResult result = _imageService.Validate(item.Cover);

    return result;
  }
}
