use std::{fs, path::Path};
use babble_processor::process_markdown;

#[test]
fn test_languages() {
    let output_dir = "./test_output";

    // Conteúdo do arquivo Markdown com blocos para cada linguagem
    let markdown_content = r#"
# TestFile

```rust
fn main() {
    println!("Hello, Rust!");
}
```

```python
print("Hello, Python!")
```

```javascript
console.log("Hello, JavaScript!");
```

```typescript
let greeting: string = "Hello, TypeScript!";
console.log(greeting);
```

```java
public class HelloWorld {
    public static void main(String[] args) {
        System.out.println("Hello, Java!");
    }
}
```

```c
#include <stdio.h>
int main() {
    printf("Hello, C!\\n");
    return 0;
}
```

```cpp
#include <iostream>
int main() {
    std::cout << "Hello, C++!" << std::endl;
    return 0;
}
```

```go
package main
import "fmt"
func main() {
    fmt.Println("Hello, Go!")
}
```

```ruby
puts "Hello, Ruby!"
```

```php
<?php
echo "Hello, PHP!";
```

```swift
print("Hello, Swift!")
```

```kotlin
fun main() {
    println("Hello, Kotlin!")
}
```
"#;

    // Caminho do arquivo de teste
    let test_file = "test_languages.md";

    // Escreve o conteúdo no arquivo de teste
    fs::write(test_file, markdown_content).unwrap();

    // Processa o arquivo Markdown
    process_markdown(test_file, output_dir, false, false).unwrap();

    // Verifica se os arquivos foram criados corretamente
    let expected_files = vec![
        "TestFile.rs",
        "TestFile.py",
        "TestFile.js",
        "TestFile.ts",
        "TestFile.java",
        "TestFile.c",
        "TestFile.cpp",
        "TestFile.go",
        "TestFile.rb",
        "TestFile.php",
        "TestFile.swift",
        "TestFile.kt",
    ];

    for file in &expected_files {
        let path = Path::new(output_dir).join(file);
        assert!(path.exists(), "Expected file {:?} to exist", path);
    }

    // Limpa os arquivos gerados para o teste
    fs::remove_file(test_file).unwrap();
    fs::remove_dir_all(output_dir).unwrap();
}
