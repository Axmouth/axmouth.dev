export enum FieldType {
  titleField = 'titleField',
  textField = 'textField',
  integerField = 'integerField',
  floatField = 'floatField',
  unsignedIntegerField = 'unsignedIntegerField',
  unsignedFloatField = 'unsignedFloatField',
  htmlField = 'htmlField',
  editorJsField = 'editorJsField',
  markdownField = 'markdownField',
  nameListField = 'nameListField',
  imageField = 'imageField',
  linkField = 'linkField',
  dateField = 'dateField',
  booleanField = 'booleanField',
}

export class AdminModelField {
  maxLength?: number;
  minLength?: number;
  pattern?: string;
  nullable: boolean;
  editable: boolean;
  insertable: boolean;
  name: string;
  identifier: string;
  uploadUrl?: string;
  type: FieldType;
  content?: any;
  displayField?: string;
  idField?: string;
}
