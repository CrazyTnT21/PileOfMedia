package com.example.mycollectionapp

import androidx.compose.material.Button
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.tooling.preview.Preview

class Settings : Page() {

  @Composable
  @Preview
  override fun Draw() {
    Text(text = "Welcome to Settings!");
    SimpleButton();
  }

  @Composable
  fun SimpleButton() {
    Button(onClick = { MainActivity.current = 0; }) {
      Text(text = "Simple Button")
    }
  }
}
