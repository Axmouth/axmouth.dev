import { User } from './user';

export class BlogPostComment {
  id: number;
  body: string;
  author: User;
  postId: number;
  createdAt: Date;
  updatedAt?: Date;
}
