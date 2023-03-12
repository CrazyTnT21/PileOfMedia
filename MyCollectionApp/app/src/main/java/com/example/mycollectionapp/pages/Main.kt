package com.example.mycollectionapp

import android.widget.Toast
import androidx.compose.animation.AnimatedVisibility
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.interaction.collectIsPressedAsState
import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ShoppingCart
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.Role.Companion.Button
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import java.time.LocalDate

class Main : Page() {

  var test: Int = 0;

  @Composable
  @Preview
  override fun Draw() {
    Column {
     val text = Text(text = test.toString())
      Date(date = LocalDate.now());
      PressIconButton(
        onClick = {
          test++; MainActivity.current = 1;},
        icon = { Icon(Icons.Filled.ShoppingCart, contentDescription = null) },
        text = { Text("Add to cart") }
      )
      Column(modifier = Modifier.padding(16.dp)) {
        var name by remember { mutableStateOf("") }
        if (name.isNotEmpty()) {
          Text(
            text = "Hello, $name!",
            modifier = Modifier.padding(bottom = 8.dp),
            style = MaterialTheme.typography.h5
          )
        }
        OutlinedTextField(
          value = name,
          onValueChange = { name = it },
          label = { Text("Name") }
        )
      }


    }
  }

  @Composable
  fun Date(date: LocalDate) {
    Row {
      Text(text = "Current date is: TEST ")
      Text(date.toString());

    }
  }

  @Composable
  fun PressIconButton(
    onClick: () -> Unit,
    icon: @Composable () -> Unit,
    text: @Composable () -> Unit,
    modifier: Modifier = Modifier,
    interactionSource: MutableInteractionSource =
      remember { MutableInteractionSource() },
  ) {
    val isPressed by interactionSource.collectIsPressedAsState()
    Button(onClick = onClick, modifier = modifier,
      interactionSource = interactionSource) {
      AnimatedVisibility(visible = isPressed) {
        if (isPressed) {
          Row {
            icon()
            Spacer(Modifier.size(ButtonDefaults.IconSpacing))
          }
        }
      }
      text()
    }
  }

}
