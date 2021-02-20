import { AdminCategory } from './admin-category';
import { FieldType } from './admin-model-field';
import { apiRoot } from 'src/environments/environment';
import { AdminModel } from './admin-model';

const blogPostModel: AdminModel = {
  name: 'Blog Posts',
  idField: 'id',
  displayField: 'title',
  endpoint: `${apiRoot}/blog-posts`,
  fields: [
    {
      name: 'Title',
      identifier: 'title',
      type: FieldType.titleField,
      editable: true,
      insertable: true,
      nullable: false,
    },
    {
      name: 'Description',
      identifier: 'description',
      type: FieldType.textField,
      editable: true,
      insertable: true,
      nullable: true,
    },
    {
      name: 'Body',
      identifier: 'body',
      type: FieldType.editorJsField,
      editable: true,
      insertable: true,
      nullable: false,
    },
    {
      name: 'Categories',
      identifier: 'categories',
      type: FieldType.nameListField,
      editable: true,
      insertable: true,
      nullable: false,
    },
    {
      name: 'Published',
      identifier: 'published',
      type: FieldType.booleanField,
      editable: true,
      insertable: false,
      nullable: false,
    },
  ],
};

const blogPostCommentModel: AdminModel = {
  name: 'Blog Post Comments',
  idField: 'id',
  displayField: 'body',
  endpoint: `${apiRoot}/blog-post-comments`,
  fields: [
    {
      name: 'Body',
      identifier: 'body',
      type: FieldType.textField,
      editable: true,
      insertable: true,
      nullable: false,
    },
  ],
};

const userModel: AdminModel = {
  name: 'Users',
  idField: 'id',
  displayField: 'displayName',
  endpoint: `${apiRoot}/users`,
  fields: [
    {
      name: 'Display Name',
      identifier: 'displayName',
      type: FieldType.titleField,
      editable: true,
      insertable: true,
      nullable: false,
    },
    {
      name: 'E-Mail Address',
      identifier: 'email',
      type: FieldType.titleField,
      editable: true,
      insertable: true,
      nullable: false,
    },
    {
      name: 'Password',
      identifier: 'password',
      type: FieldType.titleField,
      editable: true,
      insertable: false,
      nullable: false,
    },
    {
      name: 'Role',
      identifier: 'role',
      type: FieldType.titleField,
      editable: true,
      insertable: false,
      nullable: false,
    },
  ],
};

const linkModel: AdminModel = {
  name: 'Links',
  idField: 'id',
  displayField: 'name',
  endpoint: `${apiRoot}/links`,
  fields: [
    {
      name: 'Name',
      identifier: 'name',
      type: FieldType.titleField,
      editable: true,
      insertable: true,
      nullable: false,
    },
    {
      name: 'Target',
      identifier: 'target',
      type: FieldType.titleField,
      editable: true,
      insertable: true,
      nullable: false,
    },
    {
      name: 'Image',
      identifier: 'image',
      type: FieldType.imageField,
      editable: true,
      insertable: true,
      nullable: false,
    },
  ],
};

const textBodyModel: AdminModel = {
  name: 'Text Bodies',
  idField: 'id',
  displayField: 'id',
  endpoint: `${apiRoot}/links`,
  fields: [
    {
      name: 'Title',
      identifier: 'title',
      type: FieldType.titleField,
      editable: true,
      insertable: true,
      nullable: true,
    },
    {
      name: 'Slug',
      identifier: 'slug',
      type: FieldType.titleField,
      editable: true,
      insertable: true,
      nullable: false,
    },
    {
      name: 'Body',
      identifier: 'body',
      type: FieldType.editorJsField,
      editable: true,
      insertable: true,
      nullable: false,
    },
    {
      name: 'Url used',
      identifier: 'urlUsed',
      type: FieldType.titleField,
      editable: true,
      insertable: true,
      nullable: true,
    },
  ],
};

const categoryModel: AdminModel = {
  name: 'Categories',
  idField: 'id',
  displayField: 'name',
  endpoint: `${apiRoot}/categories`,
  fields: [
    {
      name: 'Name',
      identifier: 'name',
      type: FieldType.titleField,
      editable: true,
      insertable: true,
      nullable: false,
    },
  ],
};

const projectModel: AdminModel = {
  name: 'Projects',
  idField: 'id',
  displayField: 'name',
  endpoint: `${apiRoot}/projects`,
  fields: [
    {
      name: 'Name',
      identifier: 'name',
      type: FieldType.titleField,
      editable: true,
      insertable: true,
      nullable: false,
    },
    {
      name: 'Description',
      identifier: 'description',
      type: FieldType.textField,
      editable: true,
      insertable: true,
      nullable: true,
    },
    {
      name: 'Body',
      identifier: 'body',
      type: FieldType.editorJsField,
      editable: true,
      insertable: true,
      nullable: false,
    },
    {
      name: 'Cover',
      identifier: 'coverImage',
      type: FieldType.imageField,
      editable: true,
      insertable: true,
      nullable: false,
    },
    {
      name: 'Technologies',
      identifier: `technologies`,
      type: FieldType.nameListField,
      editable: true,
      insertable: true,
      nullable: false,
    },
    {
      name: 'Published',
      identifier: 'published',
      type: FieldType.booleanField,
      editable: true,
      insertable: false,
      nullable: false,
    },
  ],
};

const technologyModel: AdminModel = {
  name: 'Technologies',
  idField: 'id',
  displayField: 'name',
  endpoint: `${apiRoot}/technologies`,
  fields: [
    {
      name: 'Name',
      identifier: 'name',
      type: FieldType.titleField,
      editable: true,
      insertable: true,
      nullable: false,
    },
  ],
};

export const categoryList: AdminCategory[] = [
  {
    name: 'Blog',
    models: [blogPostModel, categoryModel, blogPostCommentModel],
  },
  {
    name: 'Projects',
    models: [projectModel, technologyModel],
  },
  {
    name: 'Users Management',
    models: [userModel],
  },
  {
    name: 'Home Page',
    models: [linkModel, textBodyModel],
  },
];
