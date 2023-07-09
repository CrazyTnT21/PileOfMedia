using MyCollectionServer.Core;
using MySqlConnector;

namespace MyCollectionServer.Pages;

public class CreatorClass : BaseClass<Creator>
{

  public CreatorClass(ILogger logger, MySqlConnection mysqlCon) : base(logger, mysqlCon)
  {
  }

  public override void Validate(Creator item, bool update = false)
  {
    throw new NotImplementedException();
  }

  public override async Task<Creator?> GetItem(uint id)
  {
    throw new NotImplementedException();
  }

  public override async Task<List<Creator>> GetItems(uint? start, uint? limit, string? orderColumn, Order? order)
  {
    throw new NotImplementedException();
  }

  public override async Task<long> CreateItem(Creator item)
  {
    throw new NotImplementedException();
  }

  public override async Task UpdateItem(Creator item)
  {
    throw new NotImplementedException();
  }

  public override async Task DeleteItem(uint id)
  {
    throw new NotImplementedException();
  }

  public override async Task DeleteItems(uint[] id)
  {
    throw new NotImplementedException();
  }
  public static Select GetX(Select select, string language)
  {
    return select
      .AddColumns(PersonSchema.table, GetColumns<Person>(PersonSchema.excludeGet))
      .AddColumn(language, "TDescription", "Description")
      .AddColumn(language, "TRole", "Role")
      .Join(new Join(TranslationSchema.table, TranslationSchema.pk, "Role", "FKRole", "TRole"))
      .Join(new Join(TranslationSchema.table, TranslationSchema.pk, PersonSchema.table,
        PersonSchema.fkdescription, "TDescription"));
  }
}
