import { test, expect } from "@playwright/test";

test.describe("Todo Application", () => {
  // Helper function to delete all todos
  const deleteAllTodos = async (page) => {
    const deleteButtons = page.locator('input[type="submit"][value="X"]');
    const todoCount = await deleteButtons.count();

    // Delete all existing todos
    for (let i = 0; i < todoCount; i++) {
      await deleteButtons.first().click();
    }
  };

  test.beforeEach(async ({ page }) => {
    await page.goto("http://localhost:3000/");
    // Delete any existing todos before each test
    await deleteAllTodos(page);
  });

  test.afterEach(async ({ page }) => {
    // Ensure todos are cleaned up after each test
    await deleteAllTodos(page);
  });

  test("shell function renders correct page structure", async ({ page }) => {
    // Check page title
    await expect(page).toHaveTitle("Todos");

    // Check for main header
    const header = page.locator("header h1");
    await expect(header).toHaveText("Todos");

    // Check for main section
    const mainSection = page.locator("main");
    await expect(mainSection).toBeVisible();
  });

  test("add and delete todo functionality", async ({ page }) => {
    // Add a new todo
    const addTodoInput = page.locator('input[name="title"]');
    const addTodoButton = page.locator('input[type="submit"][value="Add"]');

    // Add first todo
    await addTodoInput.fill("Test Todo 1");
    await addTodoButton.click();

    // Wait for the todo to appear
    const todoList = page.locator("ul");
    await expect(todoList).toContainText("Test Todo 1");

    // Add second todo
    await addTodoInput.fill("Test Todo 2");
    await addTodoButton.click();

    // Verify both todos are in the list
    await expect(todoList).toContainText("Test Todo 1");
    await expect(todoList).toContainText("Test Todo 2");

    // Delete the first todo
    const deleteButtons = page.locator('input[type="submit"][value="X"]');
    await deleteButtons.first().click();

    // Verify first todo is deleted
    await expect(todoList).not.toContainText("Test Todo 1");
    await expect(todoList).toContainText("Test Todo 2");
  });

  test("handle empty todo list", async ({ page }) => {
    // Check for "No tasks were found" message
    const emptyMessage = page.locator("p");
    await expect(emptyMessage).toHaveText("No tasks were found.");
  });

  test("add multiple todos and verify count", async ({ page }) => {
    const todoItems = ["First Task", "Second Task", "Third Task"];

    const addTodoInput = page.locator('input[name="title"]');
    const addTodoButton = page.locator('input[type="submit"][value="Add"]');

    // Add multiple todos
    for (const task of todoItems) {
      await addTodoInput.fill(task);
      await addTodoButton.click();
    }

    // Verify todos are added
    const todoList = page.locator("ul li");
    await expect(todoList).toHaveCount(todoItems.length);

    // Verify each todo text
    for (const task of todoItems) {
      await expect(page.locator("ul")).toContainText(task);
    }
  });
});
