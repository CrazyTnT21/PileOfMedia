using Application.DBMapping;
using Domain.Classes;
using Npgsql;

namespace Application.Repositories;

// public interface IImageUpload
// {
//   public Task<IResult<Image>> UploadImage(string uploadLocation);
// }

public sealed class ImageRepository : Repository, IImageRepository
{
  // private readonly IImageUpload _imageUpload;

  public ImageRepository(NpgsqlConnection connection) : base(connection)
  {
    // _imageUpload = imageUpload;
  }

  public async Task<Image?> GetById(uint id)
  {
    var result  = await new Select<Image>()
      .Where("Id", id)
      .UniqueResult(_connection);
    return result;
  }

  public async Task<IEnumerable<Image>> Get()
  {
    List<Image> results = await new Select<Image>()
      .List(_connection);
    return results;
  }

  public async Task<Image> Create(CreateImage item)
  {
    // var result = await _imageUpload.UploadImage("");
    // if (result.Failed)
    //   throw result.Exception;
    // item = result.Value;
    // object? id = await Insert(item, "Image");
    // return await GetById((uint)id);
    throw new NotImplementedException();
  }

  public async Task<Image> Update(CreateImage item)
  {
    throw new NotImplementedException();
  }

  public async Task Delete(uint id)
  {
    throw new NotImplementedException();
  }
}
