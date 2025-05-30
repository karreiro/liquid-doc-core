// Node.js compatible version of ohm-parser.js
import ohm from 'ohm-js';

const ConcreteNodeTypes = {
  HtmlDoctype: "HtmlDoctype",
  HtmlComment: "HtmlComment",
  HtmlRawTag: "HtmlRawTag",
  HtmlVoidElement: "HtmlVoidElement",
  HtmlSelfClosingElement: "HtmlSelfClosingElement",
  HtmlTagOpen: "HtmlTagOpen",
  HtmlTagClose: "HtmlTagClose",
  AttrSingleQuoted: "AttrSingleQuoted",
  AttrDoubleQuoted: "AttrDoubleQuoted",
  AttrUnquoted: "AttrUnquoted",
  AttrEmpty: "AttrEmpty",
  LiquidVariableOutput: "LiquidVariableOutput",
  LiquidRawTag: "LiquidRawTag",
  LiquidTag: "LiquidTag",
  LiquidTagOpen: "LiquidTagOpen",
  LiquidTagClose: "LiquidTagClose",
  TextNode: "TextNode",
  YAMLFrontmatter: "YAMLFrontmatter",

  LiquidVariable: "LiquidVariable",
  LiquidFilter: "LiquidFilter",
  NamedArgument: "NamedArgument",
  LiquidLiteral: "LiquidLiteral",
  VariableLookup: "VariableLookup",
  String: "String",
  Number: "Number",
  Range: "Range",
  Comparison: "Comparison",
  Condition: "Condition",

  AssignMarkup: "AssignMarkup",
  ContentForMarkup: "ContentForMarkup",
  CycleMarkup: "CycleMarkup",
  ForMarkup: "ForMarkup",
  RenderMarkup: "RenderMarkup",
  PaginateMarkup: "PaginateMarkup",
  RenderVariableExpression: "RenderVariableExpression",
  RenderAliasExpression: "RenderAliasExpression",
  ContentForNamedArgument: "ContentForNamedArgument",

  LiquidDocParamNode: "LiquidDocParamNode",
  LiquidDocParamNameNode: "LiquidDocParamNameNode",
  LiquidDocDescriptionNode: "LiquidDocDescriptionNode",
  LiquidDocExampleNode: "LiquidDocExampleNode",
  LiquidDocPromptNode: "LiquidDocPromptNode",
};

// Simplified toAST function since we don't have ohm-extras in Node.js
function toAST(matchResult, mapping) {
  const visit = (node, mapping) => {
    if (typeof mapping === 'function') {
      return mapping.call(node, ...node.children.map(child => visit(child, mapping)));
    } else if (typeof mapping === 'object') {
      const result = {};
      for (const [key, value] of Object.entries(mapping)) {
        if (typeof value === 'function') {
          result[key] = value.call(node, node);
        } else if (typeof value === 'number') {
          result[key] = visit(node.children[value], mapping);
        } else {
          result[key] = value;
        }
      }
      return result;
    }
    return node.sourceString;
  };

  return visit(matchResult, mapping);
}

function toCST(source, grammar, offset = 0) {
  const locStart = (tokens) => offset + tokens[0].source.startIdx;
  const locEnd = (tokens) => offset + tokens[tokens.length - 1].source.endIdx;
  const textNode = () => ({
    type: ConcreteNodeTypes.TextNode,
    value: function () {
      return this.sourceString;
    },
    locStart,
    locEnd,
    source,
  });

  const res = grammar.match(source, "Node");
  if (res.failed()) {
    throw new Error(res);
  }

  const LiquidDocMappings = {
    Node(implicitDescription, body) {
      const self = this;
      const implicitDescriptionNode =
        implicitDescription.sourceString.length === 0
          ? []
          : [implicitDescription.toAST(self.args.mapping)];
      return implicitDescriptionNode.concat(body.toAST(self.args.mapping));
    },
    ImplicitDescription: {
      type: ConcreteNodeTypes.LiquidDocDescriptionNode,
      name: "description",
      locStart,
      locEnd,
      source,
      content: 0,
      isImplicit: true,
      isInline: true,
    },
    TextNode: textNode(),
    paramNode: {
      type: ConcreteNodeTypes.LiquidDocParamNode,
      name: "param",
      locStart,
      locEnd,
      source,
      paramType: 2,
      paramName: 4,
      paramDescription: 8,
    },
    descriptionNode: {
      type: ConcreteNodeTypes.LiquidDocDescriptionNode,
      name: "description",
      locStart,
      locEnd,
      source,
      content: 2,
      isImplicit: false,
      isInline: function (_node) {
        return !this.children[1].sourceString.includes("\n");
      },
    },
    descriptionContent: textNode(),
    paramType: 2,
    paramTypeContent: textNode(),
    paramName: {
      type: ConcreteNodeTypes.LiquidDocParamNameNode,
      content: 0,
      locStart,
      locEnd,
      source,
      required: true,
    },
    optionalParamName: {
      type: ConcreteNodeTypes.LiquidDocParamNameNode,
      content: 2,
      locStart,
      locEnd,
      source,
      required: false,
    },
    paramDescription: textNode(),
    exampleNode: {
      type: ConcreteNodeTypes.LiquidDocExampleNode,
      name: "example",
      locStart,
      locEnd,
      source,
      content: 2,
      isInline: function (_node) {
        return !this.children[1].sourceString.includes("\n");
      },
    },
    promptNode: {
      type: ConcreteNodeTypes.LiquidDocPromptNode,
      name: "prompt",
      locStart,
      locEnd,
      source,
      content: 1,
    },
    multilineTextContent: textNode(),
    textValue: textNode(),
    fallbackNode: textNode(),
  };

  return toAST(res, LiquidDocMappings);
}

export function toLiquidHtmlAST(str) {
  const grammarStr = String.raw`
    Helpers {
      Node = TextNode*
      TextNode = AnyExceptPlus<openControl>
      openControl = end

      empty = /* nothing */
      anyExcept<lit> = (~ lit any)
      anyExceptStar<lit> = (~ lit any)*
      anyExceptPlus<lit> = (~ lit any)+
      AnyExcept<lit> = (~ lit any)
      AnyExceptPlus<lit> = (~ lit any)+
      AnyExceptStar<lit> = (~ lit any)*
      identifierCharacter = alnum | "_" | "-"

      orderedListOf<a, b, sep> =
        | nonemptyOrderedListOf<a, b, sep>
        | emptyListOf<a, sep>
      nonemptyOrderedListOf<a, b, sep> =
        | nonemptyListOf<b, sep>
        | nonemptyOrderedListOfBoth<a, b, sep>
        | nonemptyListOf<a, sep>
      nonemptyOrderedListOfBoth<a, b, sep> =
        nonemptyListOf<a, sep> (sep nonemptyListOf<b, sep>)

      singleQuote = "'"
      doubleQuote = "\""
      controls = "\u{007F}".."\u{009F}"
      noncharacters = "\u{FDD0}".."\u{FDEF}"
      newline = "\n"
    }

    LiquidDoc <: Helpers {
      Node := ImplicitDescription (LiquidDocNode | TextNode)*
      LiquidDocNode =
        | paramNode
        | exampleNode
        | descriptionNode
        | promptNode
        | fallbackNode

      endOfDescription = strictSpace* openControl
      descriptionContent = anyExceptStar<endOfDescription>
      ImplicitDescription = descriptionContent

      // By default, space matches new lines as well. We override it here to make writing rules easier.
      strictSpace = " " | "\t"
      // We use this as an escape hatch to stop matching TextNode and try again when one of these characters is encountered
      openControl:=  strictSpace* ("@" | end)
      // List of supported tags we use to identify boundaries
      supportedTags = "@prompt" | "@example" | "@description" | "@param"


      paramNode = "@param" strictSpace* paramType? strictSpace* (optionalParamName | paramName) (strictSpace* "-")? strictSpace* paramDescription
      paramType = "{" strictSpace* paramTypeContent strictSpace* "}"
      paramTypeContent = anyExceptStar<("}"| strictSpace)>

      paramName = textValue
      optionalParamName = "[" strictSpace* textValue strictSpace* "]"
      textValue = identifierCharacter+

      paramDescription = (~"]" anyExceptStar<endOfParam>)
      endOfParam = strictSpace* (newline | end)

      // Prompt node is system-controlled, so we don't strip the leading spaces to maintain indentation
      promptNode = "@prompt"  multilineTextContent
      exampleNode = "@example" space* multilineTextContent
      descriptionNode = "@description" space* multilineTextContent

      // We want multilineTextContent to be free-form, so instead of terminating the match at "@" we explicitly look for a suppported tag
      // This means that malformed tags will be considered part of the multilineTextContent
      multilineTextContent = anyExceptStar<endOfMultilineText>
      endOfMultilineText =  strictSpace* (supportedTags | end)

      fallbackNode = "@" anyExceptStar<endOfParam>
    }
  `;
  const grammars = ohm.grammars(grammarStr);
  const grammar = grammars.LiquidDoc;

  return toCST(str, grammar);
}
