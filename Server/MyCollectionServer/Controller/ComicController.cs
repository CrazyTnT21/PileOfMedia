using Infrastructure.EF;
using Microsoft.AspNetCore.Mvc;
using MyCollectionServer.Controller.Base;

namespace MyCollectionServer.Controller;

public sealed class ComicController: BaseAPIController<Domain.Classes.Comic>
{
  public ComicController(ILogger logger, AppDBContext dbContext) : base(logger, dbContext)
  {
  }

  public override async Task<Domain.Classes.Comic?> GetSingle(uint id)
  {
    return await _dbContext.Comics.FindAsync(id);
  }

  public override async Task<IEnumerable<Domain.Classes.Comic>> Get()
  {
    return _dbContext.Comics.Take(50);
  }

  public override async Task<uint> Create(Domain.Classes.Comic item)
  {
    return 0;
    // var result = (_dbContext.Add(item)).Entity.PK;
    // await _dbContext.SaveChangesAsync();
    // return result;
  }

  public override async Task Update(Domain.Classes.Comic item)
  {
    _dbContext.Update(item);
    await _dbContext.SaveChangesAsync();
  }

  public override async Task Delete(uint id)
  {
    _dbContext.Remove(new Comic { PK = id });
    await _dbContext.SaveChangesAsync();
  }

  public override async Task Delete(uint[] ids)
  {
    for (int i = 0; i < ids.Length; i++)
    {
      _dbContext.Remove(new Comic { PK = ids[i] });
    }

    await _dbContext.SaveChangesAsync();
  }

  public override void Validate(Domain.Classes.Comic item, bool update = false)
  {
    throw new NotImplementedException();
  }

  public override async Task<ActionResult<Domain.Classes.Comic?>> GetSingleResult(uint id)
  {
    return await GetSingle(id);
  }

  public override async Task<ActionResult<IEnumerable<Domain.Classes.Comic>>> GetResult()
  {
    return (await Get()).ToList();
  }

  public override async Task<ActionResult<uint>> CreateResult(Domain.Classes.Comic item)
  {
    return await CreateResult(item);
  }

  public override async Task<IActionResult> UpdateResult(Domain.Classes.Comic item)
  {
    return await UpdateResult(item);
  }

  public override async Task<IActionResult> DeleteResult(uint id)
  {
    await Delete(id);
    return new OkResult();
  }

  public override async Task<IActionResult> DeleteResult(uint[] ids)
  {
    await Delete(ids);
    return new OkResult();
  }
}
