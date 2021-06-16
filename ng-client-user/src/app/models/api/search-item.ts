export class SearchItem {
  title: string;
  createdAt?: Date | null;
  updatedAt?: Date | null;
  image?: string | null;
  description?: string | null;
  itemType: 'Project' | 'BlogPost' | 'Page' | 'ExternalLink';
  link: string;
}
