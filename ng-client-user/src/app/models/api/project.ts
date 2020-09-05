export class Project {
  id: number;
  body: string;
  createdAt: Date;
  updatedAt?: Date;
  published: boolean;
  name: string;
  coverImage?: string;
  description?: string;
  technologies?: string[];
}
