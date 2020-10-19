import { Component, OnInit } from '@angular/core';
import { Title } from '@angular/platform-browser';
import { AdminCategoryService } from '../../services/admin-category.service';

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss'],
})
export class HomeComponent implements OnInit {
  categories;

  constructor(private categoryService: AdminCategoryService, private title: Title) {}

  ngOnInit(): void {
    this.title.setTitle(`Index | Axmouth's Website Admin Site`);
    this.categories = this.categoryService.getAll();
  }
}
