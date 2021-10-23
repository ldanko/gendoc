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
  let definitions = dmntk_model::parse(dmntk_examples::DMN_2_0001, "file://2_0001.dmn").expect("parsing model 2_0001.dmn failed");
  let html = crate::generate(&definitions);
  assert_eq!("<!DOCTYPE html>", &html[0..15]);
  let mut file = File::create("./target/2_0001.html").expect("creating file ./target/2_0001.html failed");
  file.write_all(html.as_bytes()).expect("saving file ./target/2_0001.html failed");
}

#[test]
fn test_3_0087_html() {
  let definitions = dmntk_model::parse(dmntk_examples::DMN_3_0087, "file://3_0087.dmn").expect("parsing model 3_0087.dmn failed");
  let html = crate::generate(&definitions);
  assert_eq!("<!DOCTYPE html>", &html[0..15]);
  let mut file = File::create("./target/3_0087.html").expect("creating file ./target/3_0087.html failed");
  file.write_all(html.as_bytes()).expect("saving file ./target/2_0001.html failed");
}
