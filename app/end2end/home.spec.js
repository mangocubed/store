import { test, expect } from "@playwright/test";
import { loginAndGoToHome, waitForSplash } from "./shared_expects";

test("should open home page", async ({ page }) => {
    await page.goto("/");

    await waitForSplash(page);

    await expect(page).toHaveURL("/");
    await expect(page.locator("h1", { hasText: "Home" })).toBeVisible();
});
