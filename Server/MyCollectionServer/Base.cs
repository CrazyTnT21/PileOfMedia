using MySqlConnector;

namespace MyCollectionServer;

    public class BaseT
    {
        public static async Task<List<List<object>>> QueryDB(string query){
        Console.WriteLine(query);
        List<List<object>> all = new();
        var result = await new MySqlCommand(query, Server.con).ExecuteReaderAsync();

        try
        {
            byte columns = (byte)result.FieldCount;
            List<object> tempall = new();
            while (await result.ReadAsync())
            {
                for (byte i = 0; i  < columns; i++)
                {
                    tempall.Add(result.GetValue(i));
                }
                all.Add(tempall);
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
}

