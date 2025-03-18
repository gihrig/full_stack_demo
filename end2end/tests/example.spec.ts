import { test, expect } from "@playwright/test";

test("homepage has title and heading text", async ({ page }) => {
  await page.goto("http://localhost:3000/");

  await expect(page).toHaveTitle("Todos");

  await expect(page.locator("h1")).toHaveText("Todos");
});
