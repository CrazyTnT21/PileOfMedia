import * as fs from "fs";
import openapiTS, {astToString} from "openapi-typescript";
import ts from "typescript"

const BLOB = ts.factory.createTypeReferenceNode(ts.factory.createIdentifier("Blob"));
const FILE = ts.factory.createTypeReferenceNode(ts.factory.createIdentifier("File"));

const DATE = ts.factory.createTypeReferenceNode(ts.factory.createIdentifier("`${number}-${number}-${number}`"));
const NULL = ts.factory.createLiteralTypeNode(ts.factory.createNull());

const json = await fs.promises.readFile("openapi.json", "utf8");
const openApiSchema = JSON.parse(json.toString());

//OperationsIds currently contain duplicates
removeOperationIds(openApiSchema);

const output = await openapiTS(openApiSchema, {
  transform(schemaObject, metadata) {
    if (schemaObject.format === "date")
      return transformSchemaObject(schemaObject, DATE);

    if (metadata.path.endsWith('multipart/form-data'))
      return transformSchemaObject(schemaObject, FILE);

    if (metadata.path.endsWith('application/octet-stream') || schemaObject.format === "binary")
      return transformSchemaObject(schemaObject, BLOB);
  },
  enum: true
});

function transformSchemaObject(schemaObject, type) {
  return schemaObject.nullable ?
    ts.factory.createUnionTypeNode([type, NULL]) :
    type;
}

await fs.promises.writeFile("openapi.d.ts", astToString(output));

function removeOperationIds(schema) {
  const pathsKeys = Object.keys(schema.paths)
  for (const pathKey of pathsKeys) {
    const keys = Object.keys(schema.paths[pathKey]);
    for (const key of keys) {
      delete schema.paths[pathKey][key].operationId
    }
  }
}
