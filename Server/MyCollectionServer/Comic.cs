using Microsoft.AspNetCore.Mvc;
using MySqlConnector;
using System.Data;

namespace MyCollectionServer;
[Route("api/comic/{id?}")]
public class Comic : BaseClass<TComic>
{
    public async override Task<List<TComic>> GetItems(uint? id)
    {
        List<List<object>> temp = await BaseT.QueryDB("select * from TComic");
        for (int i = 0; i < temp.Count; i++)
        {
            for (int j = 0; j < temp[i].Count; j++)
            {
                Console.WriteLine(temp[i][j]);
            }
        }
        Console.WriteLine(id);
        string[] columns = { };
        List<TComic> all = new List<TComic>();
        string select = "*";
        if (columns is not null)
            select = string.Join(',', columns);
        var result = await new MySqlCommand("select * from TComic", Server.con).ExecuteReaderAsync();

        try
        {
            while (await result.ReadAsync())
            {
                TComic tempcomic = new TComic()
                {
                    PK = result.GetUInt32("PK"),
                    FKName = result.GetUInt32("FKName"),
                    AverageScore = !result.IsDBNull("AverageScore") ? result.GetUInt32("AverageScore") : null,
                    FKDescription = !result.IsDBNull("FKDescription") ? result.GetUInt32("FKDescription") : null,
                    FKSynopsis = !result.IsDBNull("FKSynopsis") ? result.GetUInt32("FKSynopsis") : null,
                    ImageSource = !result.IsDBNull("ImageSource") ? result.GetString("ImageSource") : null,
                    PublishEnd = !result.IsDBNull("PublishEnd") ? result.GetDateTime("PublishEnd") : null,
                    PublishStart = !result.IsDBNull("PublishStart") ? result.GetDateTime("PublishStart") : null,
                    Volumes = !result.IsDBNull("Volumes") ? result.GetUInt32("Volumes") : null,
                };
                all.Add(tempcomic);
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine(ex);
        }
        finally
        {
            result.Close();
        }
        return all;
    }
    public static T NullorDefault<T>(object obj)
    {
        if (obj is null || obj == DBNull.Value)
        {
            return default;
        }
        else
        {
            return (T)obj;
        }
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