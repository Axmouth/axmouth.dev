export class Project {
  id: number;
  slug: string;
  body: string;
  createdAt: Date;
  updatedAt?: Date;
  published: boolean;
  name: string;
  coverImage?: string;
  description?: string;
  technologies?: string[];
}
