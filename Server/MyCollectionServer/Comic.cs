using Microsoft.AspNetCore.Mvc;
namespace MyCollectionServer;
[Route("api/comic/{id?}")]
public class Comic : BaseClass<TComic>
{
  public async override Task<List<TComic>> GetItems(uint? id,string? language)
  {
    string[]? excludeColumns = null;
    // if (exclude)
    excludeColumns = new[] { "PK", "FKDescription", "FKSynopsis","FKName" };
    string lang = "English";
    Join[] leftjoins = {
      new Join("TTranslation",lang,"Name","TComic","FKName","PK","TComic","PK",id,JoinType.Inner),
      new Join("TTranslation",lang,"Description","TComic","FKDescription","PK"),
      new Join("TTranslation",lang,"Synopsis","TComic","FKSynopsis","PK"),
      new Join("TComicXCreator","FKPerson","Creator","TComic","PK","FKComic"),
    };
    return await QueryDB(BaseT.selectLeftJoin("TComic", leftjoins, null, null), excludeColumns);
  }
  public async override Task UpdateItem([FromBody] TComic item)
  {
    throw new NotImplementedException();
  }

  public async override Task DeleteItem(uint id)
  {
    throw new NotImplementedException();
  }

  public async override Task CreateItem([FromBody] TComic item)
  {
    Console.WriteLine(item);
  }
}
