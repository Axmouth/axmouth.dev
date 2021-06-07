import { User } from './user';

export class AdminLog {
  id: number;
  objectId: string;
  user: User;
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
