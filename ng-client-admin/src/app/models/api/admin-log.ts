export class AdminLog {
  id: number;
  objectId: string;
  userId: number;
  label: string;
  model: string;
  actionTime: string;
  action: AdminLogAction;
  newData: string;
  oldData: string;
  baseLink: string;
}

export enum AdminLogAction {
  Create,
  Update,
  Delete,
}
