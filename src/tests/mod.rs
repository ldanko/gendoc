/*
 * DMNTK - Decision Model and Notation Toolkit
 *
 * DMN documentation generator
 *
 * Copyright 2018-2021 Dariusz Depta Engos Software <dariusz.depta@engos.software>
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use std::fs::File;
use std::io::Write;

#[test]
fn test_2_0001_html() {
  let definitions = dmntk_model::parse(dmntk_examples::DMN_2_0001, "file://2_0001.dmn").unwrap();
  let html = crate::generate(&definitions);
  assert_eq!("<!DOCTYPE html>", &html[0..15]);

  if let Ok(mut file) = File::create("2_0001.html") {
    let _result = file.write(html.as_bytes());
  }

}

#[test]
fn test_3_0087_html() {
  let definitions = dmntk_model::parse(dmntk_examples::DMN_3_0087, "file://3_0087.dmn").unwrap();
  let html = crate::generate(&definitions);
  assert_eq!("<!DOCTYPE html>", &html[0..15]);

  if let Ok(mut file) = File::create("3_0087.html") {
    let _result = file.write(html.as_bytes());
  }

}