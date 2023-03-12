package com.example.mycollectionapp

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Column
import androidx.compose.material.Button
import androidx.compose.material.Text

class MainActivity : ComponentActivity() {

  private var pages: Array<Page> = arrayOf(Main(), Settings());
  companion object{
    public var current: Int = 0;

  }
  var test = 1;
  override fun onCreate(savedInstanceState: Bundle?) {
    super.onCreate(savedInstanceState)
    setContent {
      Column{
        Button(onClick = {
          test++;
          //  MainActivity.current = 1;
        }) {
          Text(text = "Simple Button$test")
        }
        Text(text = current.toString())
        pages[current].Draw();
      }

    }
  }
}
