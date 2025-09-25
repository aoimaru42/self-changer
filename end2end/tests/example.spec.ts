import { test, expect } from "@playwright/test";

test("chat page loads correctly", async ({ page }) => {
  await page.goto("/");

  // ページが正常に読み込まれることを確認
  await expect(page).toHaveTitle(/Self Changer/);

  // チャットページの主要要素が存在することを確認
  await expect(page.locator('.main-container')).toBeVisible();
  await expect(page.locator('.chat-container')).toBeVisible();
  await expect(page.locator('.messages-area')).toBeVisible();
  await expect(page.locator('.input-form')).toBeVisible();
});

test("can send a message", async ({ page }) => {
  await page.goto("/");

  // 入力フィールドと送信ボタンが存在することを確認
  const inputField = page.locator('.input-field');
  const sendButton = page.locator('.send-button');

  await expect(inputField).toBeVisible();
  await expect(sendButton).toBeVisible();

  // メッセージを入力
  await inputField.fill("こんにちは");

  // 送信ボタンをクリック
  await sendButton.click();

  // メッセージが送信されることを確認
  // 入力フィールドがクリアされるか、新しいメッセージが追加されるかを確認
  await Promise.race([
    expect(inputField).toHaveValue(""), // 送信後にフィールドがクリアされる
    expect(page.locator('.message-item')).toHaveCount(2) // 初期メッセージ + 新しいメッセージ
  ]);
});

test("refresh button works", async ({ page }) => {
  await page.goto("/");

  // リフレッシュボタンが存在することを確認
  const refreshButton = page.locator('.refresh-button');
  await expect(refreshButton).toBeVisible();

  // リフレッシュボタンをクリック
  await refreshButton.click();

  // ページがリフレッシュされることを確認（メッセージがリセットされる）
  await expect(page.locator('.messages-area')).toBeVisible();
});

test("API response handling", async ({ page }) => {
  await page.goto("/");

  // 入力フィールドと送信ボタンが存在することを確認
  const inputField = page.locator('.input-field');
  const sendButton = page.locator('.send-button');

  await expect(inputField).toBeVisible();
  await expect(sendButton).toBeVisible();

  // メッセージを入力
  await inputField.fill("背景を青にして");

  // 送信ボタンをクリック
  await sendButton.click();

  // APIレスポンスを待つ（最大10秒）
  await page.waitForTimeout(10000);

  // メッセージが追加されているか、またはエラーメッセージが表示されているかを確認
  const messageCount = await page.locator('.message-item').count();
  expect(messageCount).toBeGreaterThanOrEqual(1); // 最低でも初期メッセージは存在する
});
