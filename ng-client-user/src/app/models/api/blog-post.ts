import { User } from 'src/app/models/api/user';
export class BlogPost {
  id: number;
  slug: string;
  title: string;
  body: string;
  createdAt: Date;
  updatedAt?: Date;
  published: boolean;
  author: User;
  categories: string[];
  description?: string;
}
