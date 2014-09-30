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

#![crate_name = "peg"]
#![experimental]
#![comment = "Parsing Expression Grammar Library"]
#![license = "Apache v2"]
#![crate_type = "dylib"]

#![feature(plugin_registrar, quote, globs)]

extern crate rustc;
extern crate syntax;
extern crate attribute;

use rustc::plugin::Registry;

pub use runtime::Parser;
use front::parser;

pub mod runtime;
mod front;
mod middle;
mod back;
mod rust;
mod identifier;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) 
{
  reg.register_macro("peg", expand)
}

fn expand<'cx>(cx: &'cx mut rust::ExtCtxt, _sp: rust::Span, tts: &[rust::TokenTree]) -> Box<rust::MacResult + 'cx> 
{
  parse(cx, tts)
}

fn parse<'cx>(cx: &'cx mut rust::ExtCtxt, tts: &[rust::TokenTree]) -> Box<rust::MacResult + 'cx>
{
  let mut parser = parser::Parser::new(cx.parse_sess(), cx.cfg(), tts.to_vec());
  let ast = parser.parse_grammar();
  let ast = middle::analyse(cx, ast);
  match ast {
    Some(ast) => back::PegCompiler::compile(cx, ast),
    None => {
      cx.parse_sess.span_diagnostic.handler.abort_if_errors();
      rust::DummyResult::any(rust::DUMMY_SP)
    }
  }
}
