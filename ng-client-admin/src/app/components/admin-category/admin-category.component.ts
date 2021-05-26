import { Component, OnInit, Input } from '@angular/core';
import { AdminCategory } from 'src/app/models/definitions/admin-category';

@Component({
  selector: 'app-admin-category',
  templateUrl: './admin-category.component.html',
  styleUrls: ['./admin-category.component.scss'],
})
export class AdminCategoryComponent implements OnInit {
  @Input()
  category: AdminCategory;

  constructor() {}

  ngOnInit(): void {}
}
