import { Component, OnInit } from '@angular/core';
import { AdminCategoryService } from '../../services/admin-category.service';

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss'],
})
export class HomeComponent implements OnInit {
  categories;

  constructor(private categoryService: AdminCategoryService) {}

  ngOnInit(): void {
    this.categories = this.categoryService.getAll();
  }
}
