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

//! Generator of DMN documentation.

use dmntk_model::model::{Definitions, DcDimension, DmnDiagramElement, DmnShape, DmnEdge, DcBounds, InputData, Decision, DcPoint, DmnStyle, DcColor, BusinessKnowledgeModel, KnowledgeSource};
use std::ops::Div;

const HTML_TEMPLATE: &str = include_str!("template.html");
const SVG_CONTENT: &str = "#SVG_CONTENT#";

/// Generates documentation for DMN model.
pub fn generate(definitions: &Definitions) -> String {
  let mut html = HTML_TEMPLATE.to_string();
  html = add_svg_content(html, definitions);
  html
}

fn add_svg_content(html: String, definitions: &Definitions) -> String {
  let mut svg = String::new();

  if let Some(dmndi) = definitions.dmndi() {
    let styles = svg_styles(&dmndi.styles);
    for diagram in &dmndi.diagrams {
      svg = format!("{}{}", svg, svg_begin(&diagram.size));
      svg = format!("{}{}", svg, styles);

      for diagram_element in &diagram.diagram_elements {
        match diagram_element {
          DmnDiagramElement::DmnShape(shape) => {
            if let Some(dmn_element_ref) = &shape.dmn_element_ref {
              if let Some(decision) = definitions.decision_by_id(dmn_element_ref.as_str()) {
                svg = format!("{}\n{}", svg, svg_decision(&shape, decision));
              } else if let Some(input_data) = definitions.input_data_by_id(dmn_element_ref.as_str()) {
                svg = format!("{}\n{}", svg, svg_input_data(&shape, input_data));
              } else if let Some(business_knowledge) = definitions.business_knowledge_model_by_id(dmn_element_ref.as_str()) {
                svg = format!("{}\n{}", svg, svg_business_knowledge(&shape, business_knowledge));
              }
              // else if let Some(knowledge_source) = definitions.input_data_by_id(dmn_element_ref.as_str()) {
              //   svg = format!("{}\n{}", svg, svg_knowledge_source(&shape, knowledge_source));
              // }

            }
          }
          DmnDiagramElement::DmnEdge(edge) => {
            svg = format!("{}\n{}", svg, svg_edge(edge));
          }
        }
      }

      svg = format!("{}{}", svg, svg_end());
    }
  }

  html.replace(SVG_CONTENT, svg.as_str())
}

/// Generate begin svg element
fn svg_begin(size_opt: &Option<DcDimension>) -> String {
  if let Some(size) = size_opt {
    format!("<svg width=\"{}\" height=\"{}\">", size.width, size.height)
  } else {
    String::from("<svg>")
  }
}

/// Generate end svg element
fn svg_end() -> String {
  String::from("</svg>")
}

fn get_shape_shared_style_id(shape: &DmnShape) -> String {
  if let Some(style_id) = &shape.shared_style {
    style_id.to_string()
  } else {String::new()}
}

fn get_shape_label_shared_style_id(shape: &DmnShape) -> String {
  if let Some(label) = &shape.label {
    if let Some(style_id) = &label.shared_style {
      style_id.to_string()
    }
    else { String::new() }
  } else {String::new()}
}

/// Generate svg element for Decision.
fn svg_decision(shape: &DmnShape, decision: &Decision) -> String {
  let text = get_text(shape, &decision.name);
  let text_position = get_text_position(&shape.bounds);
  let shape_class = get_shape_shared_style_id(&shape);
  let label_class = get_shape_label_shared_style_id(&shape);

  let mut svg_decision = format!("<rect width=\"{}\" height=\"{}\" x=\"{}\" y=\"{}\" class=\"{}\"/>", shape.bounds.width, shape.bounds.height, shape.bounds.x, shape.bounds.y,shape_class);
  svg_decision = format!("{}<text x=\"{}\" y=\"{}\" dominant-baseline=\"middle\" text-anchor=\"middle\" class=\"{}\" style=\"fill:black\">{}</text>", svg_decision, text_position.0, text_position.1, label_class,text);
  svg_decision
}

/// Generate svg element for Decision.
fn svg_input_data(shape: &DmnShape, input_data: &InputData) -> String {
  let rxy = shape.bounds.height.div(2.0);
  let text = get_text(shape, &input_data.name);
  let text_position = get_text_position(&shape.bounds);
  let shape_class = get_shape_shared_style_id(&shape);
  let label_class = get_shape_label_shared_style_id(&shape);

  let mut svg_input_data = format!("<rect width=\"{}\" height=\"{}\" x=\"{}\" y=\"{}\" rx=\"{}\" ry=\"{}\" class=\"{}\"/>", shape.bounds.width, shape.bounds.height, shape.bounds.x, shape.bounds.y, rxy, rxy, shape_class);
  svg_input_data = format!("{}<text x=\"{}\" y=\"{}\" dominant-baseline=\"middle\" text-anchor=\"middle\" class=\"{}\" style=\"fill:black\">{}</text>", svg_input_data, text_position.0, text_position.1, label_class,text);
  svg_input_data
}

/// Generate svg element for Business Knowledge.
fn svg_business_knowledge(shape: &DmnShape, business_knowledge: &BusinessKnowledgeModel) -> String {
  let text = get_text(shape, &business_knowledge.name);
  let text_position = get_text_position(&shape.bounds);
  let points = get_points_for_business_knowledge(&shape.bounds);
  let shape_class = get_shape_shared_style_id(&shape);
  let label_class = get_shape_label_shared_style_id(&shape);

  let mut svg_business_knowledge = format!("<polygon points=\"{}\" class=\"{}\"/>", points, shape_class);
  svg_business_knowledge = format!("{}<text x=\"{}\" y=\"{}\" dominant-baseline=\"middle\" text-anchor=\"middle\" class=\"{}\" style=\"fill:black\">{}</text>", svg_business_knowledge, text_position.0, text_position.1, label_class, text);
  svg_business_knowledge
}

fn get_points_for_business_knowledge(bounds: &DcBounds) -> String {
  let mut points = format!("{},{}",bounds.x, bounds.y + 15.0);
  points = format!("{} {},{}", points,bounds.x + 15.0, bounds.y);
  points = format!("{} {},{}", points,bounds.x + bounds.width, bounds.y);
  points = format!("{} {},{}", points,bounds.x + bounds.width, bounds.y + bounds.height - 15.0);
  points = format!("{} {},{}", points,bounds.x + bounds.width - 15.0, bounds.y + bounds.height);
  points = format!("{} {},{}", points,bounds.x, bounds.y + bounds.height);

  points
}

/// Generate svg element for Knowledge Source.
fn svg_knowledge_source(shape: &DmnShape, _knowledge_source: &KnowledgeSource) -> String {
  let empty = String::new();
  let text = get_text(shape, &empty);
  let text_position = get_text_position(&shape.bounds);
  let path = get_path_to_knowledge_source(&shape.bounds);
  let shape_class = get_shape_shared_style_id(&shape);
  let label_class = get_shape_label_shared_style_id(&shape);

  let mut svg_knowledge_source = format!("<path d=\"{}\" class=\"{}\"/>", path, shape_class);
  svg_knowledge_source = format!("{}<text x=\"{}\" y=\"{}\" dominant-baseline=\"middle\" text-anchor=\"middle\" class=\"{}\" style=\"fill:black\">{}</text>", svg_knowledge_source, text_position.0, text_position.1, label_class,text);
  svg_knowledge_source
}

fn get_path_to_knowledge_source(bounds: &DcBounds) -> String {
  let width_div_4: f64 = bounds.width.div(4.0);
  let mut path = format!("M {} {}", bounds.x, bounds.y);
  path = format!("{} L {} {}", path, bounds.x + bounds.width, bounds.y);
  path = format!("{} L {} {}", path, bounds.x + bounds.width, bounds.y + bounds.height);
  path = format!("{} C {},{} {},{} {},{}", path, bounds.x + bounds.width, bounds.y + bounds.height, bounds.x + bounds.width - width_div_4, bounds.y + bounds.height - 30.0, bounds.x + bounds.width - width_div_4*2.0, bounds.y + bounds.height);
  path = format!("{} C {},{} {},{} {},{}", path, bounds.x + bounds.width - width_div_4*2.0, bounds.y + bounds.height, bounds.x + width_div_4, bounds.y + bounds.height + 30.0, bounds.x, bounds.y + bounds.height);
  path = format!("{} L {} {} Z", path, bounds.x, bounds.y);

  path
}

/// Generate svg element for edge.
fn svg_edge(edge: &DmnEdge) -> String {
  let points: String = edge.way_points.iter().map(|w| format!("{},{} ", w.x, w.y)).collect();
  let start_point = &edge.way_points[edge.way_points.len() - 2];
  let end_point = &edge.way_points[edge.way_points.len() - 1];

  let points_for_arrowhead = format!("{},{} {},{} {},{}", end_point.x, end_point.y, end_point.x - 5.0, end_point.y - 10.0, end_point.x + 5.0, end_point.y - 10.0);
  let angle = get_angle(start_point, end_point);

  let mut edge = format!("<polyline points=\"{}\"/>", points);
  edge = format!("{}<polygon points=\"{}\" transform=\"rotate({},{},{})\" style=\"fill:rgb(0,0,0);\" />", edge, points_for_arrowhead, angle, end_point.x, end_point.y);
  edge
}

fn svg_styles(styles: &Vec<DmnStyle>) -> String {
  let mut svg_styles = String::from("<style>");
  for style in styles {
    if let Some(style_id) = &style.id {
      let fill_color = if let Some(fill_color) = &style.fill_color {
        format!("fill: {};", get_rgb_color(fill_color))
      } else { String::new() };
      let stroke_color = if let Some(stroke_color) = &style.stroke_color {
        format!("stroke: {};", get_rgb_color(stroke_color))
      } else { String::new() };
      let font_color = if let Some(font_color) = &style.font_color {
        format!("color: {};", get_rgb_color(font_color))
      } else { String::new() };
      let font_family = format!("font-family: {};", &style.font_family);
      let font_size = format!("font-size: {}px;", &style.font_size);
      let font_italic = if style.font_italic {
        format!("font-style: italic;")
      } else { String::new() };
      let font_bold = if style.font_bold {
        format!("font-weight: bold;")
      } else { String::new() };
      let font_underline = if style.font_underline {
        format!("text-decoration: underline;")
      } else { String::new() };
      let font_strike_through = if style.font_strike_through {
        format!("overflow: visible;")
      } else { String::new() };
      let label_horizontal_alignment = if let Some(_alignment) = &style.label_horizontal_alignment {
        format!("")
      } else { String::new() };
      let label_vertical_alignment = if let Some(_alignment) = &style.label_vertical_alignment {
        format!("")
      } else { String::new() };

      let svg_style = format!(".{} {{ {} {} {} {} {} {} {} {} {} {} {} }}", style_id, fill_color, stroke_color, font_color, font_family, font_size, font_italic, font_bold, font_underline, font_strike_through, label_horizontal_alignment, label_vertical_alignment);
      svg_styles = format!("{}{}", svg_styles, svg_style);
    }
  }
  svg_styles = format!("{}</style>", svg_styles);
  svg_styles
}

fn get_rgb_color(color: &DcColor) -> String {
  format!("rgb({},{},{})", color.red, color.green, color.blue)
}

/// Calculate text position inside a shape.
fn get_text_position(bounds: &DcBounds) -> (f64, f64) {
  let text_x = bounds.x + bounds.width.div(2.0);
  let text_y = bounds.y + bounds.height.div(2.0);
  (text_x, text_y)
}

fn get_text<'a>(shape: &'a DmnShape, name: &'a String) -> &'a String {
  let mut text = name;
  if let Some(label) = &shape.label {
    if let Some(label_text) = &label.text {
      text = label_text
    }
  }
  text
}

/// Calculate angle between two vectors.
fn get_angle(start: &DcPoint, end: &DcPoint) -> f64 {
  let vec1x = end.x - start.x;
  let vec1y = end.y - start.y;
  let vec2x = 0.0;
  let vec2y = 1.0;

  let result = (vec1x * vec2x + vec1y * vec2y).div((vec1x.powf(2.0) + vec1y.powf(2.0)).sqrt() * (vec2x.powf(2.0) + vec2y.powf(2.0)).sqrt()).acos().to_degrees();
  if vec1x > 0.0 {
    -result
  } else {
    result
  }
}