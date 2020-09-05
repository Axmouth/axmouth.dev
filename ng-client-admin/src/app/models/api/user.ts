export class User {
  id: { $oid: string };
  displayName?: string;
  email?: string;
  role?: string;
  createdAt?: number;
}
