// Copyright 2014 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use middle::attribute::rule_type::RuleTypeStyle::*;
use attribute::model::*;

#[derive(Clone)]
pub enum RuleTypeStyle
{
  Inline,
  Invisible(Span)
}

impl RuleTypeStyle
{
  pub fn new(_: &ExtCtxt, model: &AttributeArray) -> RuleTypeStyle
  {
    let invisible_type = access::plain_value(model, "invisible_type");
    if invisible_type.has_value() {
      Invisible(invisible_type.span())
    } else {
      Inline
    }
  }

  pub fn model() -> AttributeArray
  {
    vec![
      AttributeInfo::simple(
        "invisible_type",
        "the calling site will ignore the type of this rule. The AST of the calling rule will not reference this rule.",
      )
    ]
  }
}

pub struct RuleType
{
  pub style: RuleTypeStyle
}

impl RuleType
{
  pub fn new(cx: &ExtCtxt, model: &AttributeArray) -> RuleType
  {
    RuleType {
      style: RuleTypeStyle::new(cx, model)
    }
  }

  pub fn model() -> AttributeArray
  {
    RuleTypeStyle::model()
  }
}
