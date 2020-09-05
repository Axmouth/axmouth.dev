import { Pagination } from './pagination';

export class Response<T> {
  data?: T;
  errors?: string[];
  messages?: string[];
  success?: boolean;
  pagination?: Pagination;
}
