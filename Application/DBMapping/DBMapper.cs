using System.Reflection;

namespace Application.DBMapping;

public static class DBMapper
{
  public static IDBMapping[]? Mappings { get; private set; }

  public static void AddMapping(List<IDBMapping> list, IDBMapping mapping) => list.Add(mapping);
  public static void AddMapping<T>(List<IDBMapping> list) where T : IMapping, new() => list.Add(new T().Mapping);

  public static bool ValidateMappings(bool ValidateSchema = true)
  {
    return true;
  }

  public static void SetMappings(bool validateSchema)
  {
    Assembly[] assemblies = AppDomain.CurrentDomain.GetAssemblies();
    List<IDBMapping> mappings = new List<IDBMapping>();
    for (int i = 0; i < assemblies.Length; i++)
    {
      Type[] types = assemblies[i].GetTypes();
      for (int j = 0; j < types.Length; j++)
      {
        if (!types[j].IsInterface &&
            types[j].IsAssignableTo(typeof(IMapping)))
        {
          IMapping value = (IMapping)Activator.CreateInstance(types[j])!;
          AddMapping(mappings, value.Mapping);
        }
      }
    }

    Mappings = mappings.ToArray();
    ValidateMappings(validateSchema);
  }
}
