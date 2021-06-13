import { AdminModelField } from './admin-model-field';

export class AdminModel {
  idField: string;
  displayField?: string;
  name: string;
  endpoint: string;
  fields: AdminModelField[];
  modelId: string;
}
