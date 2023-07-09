namespace MyCollectionServer.Controller;


public struct ColumnMapping
{
  public readonly uint[] PropertyPosition;
  public readonly string Column;

  public ColumnMapping(uint[] propertyPosition, string column)
  {
    PropertyPosition = propertyPosition;
    Column = column;
  }
}
