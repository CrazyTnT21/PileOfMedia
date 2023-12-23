using Domain.Classes;
using Domain.Classes.DB;

namespace Application.Repositories;

public interface IImageRepository: IRepository
{
  public Task<Image?> GetById(uint id);
  public Task<IEnumerable<Image>> Get();
  public Task<Image> Create(CreateImage item);
  public Task<Image> Update(CreateImage item);
  public Task Delete(uint id);
}
