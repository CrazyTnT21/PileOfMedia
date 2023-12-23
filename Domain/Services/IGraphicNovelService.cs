using Domain.Classes;
using Domain.Enums;
using Domain.Interfaces;
using Domain.ValueObjects;

namespace Domain.Services;

public interface IGraphicNovelService : ILanguageGet<GraphicNovel>, ILanguageGetById<GraphicNovel>
{
  public Task<IResult<GraphicNovel>> Create(CreateGraphicNovel item);
  public Task<IResult<GraphicNovel>> Update(CreateGraphicNovel item);
  public Task<IResult> Delete(uint id);
  public IResult Validate(CreateGraphicNovel item);
}
