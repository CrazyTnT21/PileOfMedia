using Domain.Classes;
using Domain.Enums;

namespace Application.Repositories;

public interface IGraphicNovelRepository: IRepository
{
  public Task<GraphicNovel?> GetById(uint id, Language language);
  public Task<IEnumerable<GraphicNovel>> Get(Language language);
  public Task<GraphicNovel> Create(CreateGraphicNovel item);
  public Task<GraphicNovel> Update(CreateGraphicNovel item);
  public Task Delete(uint id);
}
