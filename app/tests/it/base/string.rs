// Copyright 2023 Databend Labs
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use askbend::remove_markdown_links;

#[test]
fn test_remove_markdown_links() {
    let markdown_content = "This is an [inline link](https://www.example.com), an ![image](https://www.example.com/image.png), and a [reference link][1].\n\n[1]: https://www.example.org";
    let expected_output = "This is an inline link, an !image, and a [reference link][1].\n\n[1]: https://www.example.org";
    let result = remove_markdown_links(markdown_content);
    assert_eq!(result, expected_output);
}
