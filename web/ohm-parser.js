(function () {
  let ConcreteNodeTypes = {
    HtmlDoctype: 'HtmlDoctype',
    HtmlComment: 'HtmlComment',
    HtmlRawTag: 'HtmlRawTag',
    HtmlVoidElement: 'HtmlVoidElement',
    HtmlSelfClosingElement: 'HtmlSelfClosingElement',
    HtmlTagOpen: 'HtmlTagOpen',
    HtmlTagClose: 'HtmlTagClose',
    AttrSingleQuoted: 'AttrSingleQuoted',
    AttrDoubleQuoted: 'AttrDoubleQuoted',
    AttrUnquoted: 'AttrUnquoted',
    AttrEmpty: 'AttrEmpty',
    LiquidVariableOutput: 'LiquidVariableOutput',
    LiquidRawTag: 'LiquidRawTag',
    LiquidTag: 'LiquidTag',
    LiquidTagOpen: 'LiquidTagOpen',
    LiquidTagClose: 'LiquidTagClose',
    TextNode: 'TextNode',
    YAMLFrontmatter: 'YAMLFrontmatter',

    LiquidVariable: 'LiquidVariable',
    LiquidFilter: 'LiquidFilter',
    NamedArgument: 'NamedArgument',
    LiquidLiteral: 'LiquidLiteral',
    VariableLookup: 'VariableLookup',
    String: 'String',
    Number: 'Number',
    Range: 'Range',
    Comparison: 'Comparison',
    Condition: 'Condition',

    AssignMarkup: 'AssignMarkup',
    ContentForMarkup: 'ContentForMarkup',
    CycleMarkup: 'CycleMarkup',
    ForMarkup: 'ForMarkup',
    RenderMarkup: 'RenderMarkup',
    PaginateMarkup: 'PaginateMarkup',
    RenderVariableExpression: 'RenderVariableExpression',
    RenderAliasExpression: 'RenderAliasExpression',
    ContentForNamedArgument: 'ContentForNamedArgument',

    LiquidDocParamNode: 'LiquidDocParamNode',
    LiquidDocParamNameNode: 'LiquidDocParamNameNode',
    LiquidDocDescriptionNode: 'LiquidDocDescriptionNode',
    LiquidDocExampleNode: 'LiquidDocExampleNode',
    LiquidDocPromptNode: 'LiquidDocPromptNode',
  }
  const markupTrimEnd = (i) => (tokens) => tokens[i].sourceString.trimEnd();

  function toCST(
    source,
    grammars,
    grammar,
    cstMappings,
    matchingSource,
    offset,
  ) {
    cstMappings ||= ['LiquidMappings']
    matchingSource ||= source
    offset ||= 0

    const locStart = (tokens) => offset + tokens[0].source.startIdx;
    const locEnd = (tokens) => offset + tokens[tokens.length - 1].source.endIdx;
    const locEndSecondToLast = (tokens) => offset + tokens[tokens.length - 2].source.endIdx;

    const textNode = {
      type: ConcreteNodeTypes.TextNode,
      value: function () {
        return (this).sourceString;
      },
      locStart,
      locEnd,
      source,
    };

    const res = grammar.match(matchingSource, 'LiquidDocNode');
    if (res.failed()) {
      console.log(res)
      // throw new LiquidHTMLCSTParsingError(res);
    }

    const HelperMappings = {
      Node: 0,
      TextNode: textNode,
      orderedListOf: 0,

      listOf: 0,
      empty: () => null,
      emptyListOf: () => [],
      nonemptyListOf(first, _sep, rest) {
        const self = this;
        return [first.toAST(self.args.mapping)].concat(rest.toAST(self.args.mapping));
      },

      nonemptyOrderedListOf: 0,
      nonemptyOrderedListOfBoth(nonemptyListOfA, _sep, nonemptyListOfB) {
        const self = this;
        return nonemptyListOfA
          .toAST(self.args.mapping)
          .concat(nonemptyListOfB.toAST(self.args.mapping));
      },
    };

    const LiquidMappings = {
      liquidNode: 0,
      liquidRawTag: 0,
      liquidRawTagImpl: {
        type: ConcreteNodeTypes.LiquidRawTag,
        name: 3,
        body: 9,
        children: (tokens) => {
          const nameNode = tokens[3];
          const rawMarkupStringNode = tokens[9];
          switch (nameNode.sourceString) {
            // {% raw %} accepts syntax errors, we shouldn't try to parse that
            case 'raw': {
              return toCST(
                source,
                grammars,
                TextNodeGrammar,
                ['HelperMappings'],
                rawMarkupStringNode.sourceString,
                offset + rawMarkupStringNode.source.startIdx,
              );
            }

            // {% javascript %}, {% style %}
            default: {
              return toCST(
                source,
                grammars,
                grammars.Liquid,
                ['HelperMappings', 'LiquidMappings'],
                rawMarkupStringNode.sourceString,
                offset + rawMarkupStringNode.source.startIdx,
              );
            }
          }
        },
        markup: 6,
        whitespaceStart: 1,
        whitespaceEnd: 7,
        delimiterWhitespaceStart: 11,
        delimiterWhitespaceEnd: 17,
        locStart,
        locEnd,
        source,
        blockStartLocStart: (tokens) => tokens[0].source.startIdx,
        blockStartLocEnd: (tokens) => tokens[8].source.endIdx,
        blockEndLocStart: (tokens) => tokens[10].source.startIdx,
        blockEndLocEnd: (tokens) => tokens[18].source.endIdx,
      },
      liquidBlockComment: {
        type: ConcreteNodeTypes.LiquidRawTag,
        name: 'comment',
        body: (tokens) => tokens[1].sourceString,
        children: (tokens) => {
          return toCST(
            source,
            grammars,
            TextNodeGrammar,
            ['HelperMappings'],
            tokens[1].sourceString,
            offset + tokens[1].source.startIdx,
          );
        },
        whitespaceStart: (tokens) => tokens[0].children[1].sourceString,
        whitespaceEnd: (tokens) => tokens[0].children[7].sourceString,
        delimiterWhitespaceStart: (tokens) => tokens[2].children[1].sourceString,
        delimiterWhitespaceEnd: (tokens) => tokens[2].children[7].sourceString,
        locStart,
        locEnd,
        source,
        blockStartLocStart: (tokens) => tokens[0].source.startIdx,
        blockStartLocEnd: (tokens) => tokens[0].source.endIdx,
        blockEndLocStart: (tokens) => tokens[2].source.startIdx,
        blockEndLocEnd: (tokens) => tokens[2].source.endIdx,
      },
      liquidDoc: {
        type: ConcreteNodeTypes.LiquidRawTag,
        name: 'doc',
        body: (tokens) => tokens[1].sourceString,
        children: (tokens) => {
          const contentNode = tokens[1];
          return toLiquidDocAST(
            source,
            contentNode.sourceString,
            offset + contentNode.source.startIdx,
          );
        },
        whitespaceStart: (tokens) => tokens[0].children[1].sourceString,
        whitespaceEnd: (tokens) => tokens[0].children[7].sourceString,
        delimiterWhitespaceStart: (tokens) => tokens[2].children[1]?.sourceString || '',
        delimiterWhitespaceEnd: (tokens) => tokens[2].children[7]?.sourceString || '',
        locStart,
        locEnd,
        source,
        blockStartLocStart: (tokens) => tokens[0].source.startIdx,
        blockStartLocEnd: (tokens) => tokens[0].source.endIdx,
        blockEndLocStart: (tokens) => tokens[2].source.startIdx,
        blockEndLocEnd: (tokens) => tokens[2].source.endIdx,
      },
      liquidInlineComment: {
        type: ConcreteNodeTypes.LiquidTag,
        name: 3,
        markup: markupTrimEnd(5),
        whitespaceStart: 1,
        whitespaceEnd: 6,
        locStart,
        locEnd,
        source,
      },

      liquidTagOpen: 0,
      liquidTagOpenStrict: 0,
      liquidTagOpenBaseCase: 0,
      liquidTagOpenRule: {
        type: ConcreteNodeTypes.LiquidTagOpen,
        name: 3,
        markup(nodes) {
          const markupNode = nodes[6];
          const nameNode = nodes[3];
          if (NamedTags.hasOwnProperty(nameNode.sourceString)) {
            return markupNode.toAST((this).args.mapping);
          }
          return markupNode.sourceString.trim();
        },
        whitespaceStart: 1,
        whitespaceEnd: 7,
        locStart,
        locEnd,
        source,
      },

      liquidTagOpenCapture: 0,
      liquidTagOpenForm: 0,
      liquidTagOpenFormMarkup: 0,
      liquidTagOpenFor: 0,
      liquidTagOpenForMarkup: {
        type: ConcreteNodeTypes.ForMarkup,
        variableName: 0,
        collection: 4,
        reversed: 6,
        args: 8,
        locStart,
        locEnd,
        source,
      },
      liquidTagBreak: 0,
      liquidTagContinue: 0,
      liquidTagOpenTablerow: 0,
      liquidTagOpenPaginate: 0,
      liquidTagOpenPaginateMarkup: {
        type: ConcreteNodeTypes.PaginateMarkup,
        collection: 0,
        pageSize: 4,
        args: 6,
        locStart,
        locEnd,
        source,
      },
      liquidTagOpenCase: 0,
      liquidTagOpenCaseMarkup: 0,
      liquidTagWhen: 0,
      liquidTagWhenMarkup: 0,
      liquidTagOpenIf: 0,
      liquidTagOpenUnless: 0,
      liquidTagElsif: 0,
      liquidTagElse: 0,
      liquidTagOpenConditionalMarkup: 0,
      condition: {
        type: ConcreteNodeTypes.Condition,
        relation: 0,
        expression: 2,
        locStart,
        locEnd,
        source,
      },
      comparison: {
        type: ConcreteNodeTypes.Comparison,
        comparator: 2,
        left: 0,
        right: 4,
        locStart,
        locEnd,
        source,
      },

      liquidTagClose: {
        type: ConcreteNodeTypes.LiquidTagClose,
        name: 4,
        whitespaceStart: 1,
        whitespaceEnd: 7,
        locStart,
        locEnd,
        source,
      },

      liquidTag: 0,
      liquidTagStrict: 0,
      liquidTagBaseCase: 0,
      liquidTagAssign: 0,
      liquidTagEcho: 0,
      liquidTagContentFor: 0,
      liquidTagCycle: 0,
      liquidTagIncrement: 0,
      liquidTagDecrement: 0,
      liquidTagRender: 0,
      liquidTagInclude: 0,
      liquidTagSection: 0,
      liquidTagSections: 0,
      liquidTagLayout: 0,
      liquidTagRule: {
        type: ConcreteNodeTypes.LiquidTag,
        name: 3,
        markup(nodes) {
          const markupNode = nodes[6];
          const nameNode = nodes[3];
          if (NamedTags.hasOwnProperty(nameNode.sourceString)) {
            return markupNode.toAST((this).args.mapping);
          }
          return markupNode.sourceString.trim();
        },
        whitespaceStart: 1,
        whitespaceEnd: 7,
        source,
        locStart,
        locEnd,
      },

      liquidTagLiquid: 0,

      liquidTagEchoMarkup: 0,
      liquidTagSectionMarkup: 0,
      liquidTagSectionsMarkup: 0,
      liquidTagLayoutMarkup: 0,
      liquidTagAssignMarkup: {
        type: ConcreteNodeTypes.AssignMarkup,
        name: 0,
        value: 4,
        locStart,
        locEnd,
        source,
      },

      liquidTagCycleMarkup: {
        type: ConcreteNodeTypes.CycleMarkup,
        groupName: 0,
        args: 3,
        locStart,
        locEnd,
        source,
      },

      liquidTagContentForMarkup: {
        type: ConcreteNodeTypes.ContentForMarkup,
        contentForType: 0,
        args: 2,
        locStart,
        locEnd,
        source,
      },
      contentForType: 0,

      liquidTagRenderMarkup: {
        type: ConcreteNodeTypes.RenderMarkup,
        snippet: 0,
        variable: 1,
        alias: 2,
        renderArguments: 3,
        locStart,
        locEnd,
        source,
      },
      renderArguments: 1,
      snippetExpression: 0,
      renderVariableExpression: {
        type: ConcreteNodeTypes.RenderVariableExpression,
        kind: 1,
        name: 3,
        locStart,
        locEnd,
        source,
      },
      renderAliasExpression: {
        type: ConcreteNodeTypes.RenderAliasExpression,
        value: 3,
        locStart,
        locEnd,
        source,
      },

      liquidDrop: {
        type: ConcreteNodeTypes.LiquidVariableOutput,
        markup: 3,
        whitespaceStart: 1,
        whitespaceEnd: 4,
        locStart,
        locEnd,
        source,
      },

      liquidDropCases: 0,
      liquidExpression: 0,
      liquidVariable: {
        type: ConcreteNodeTypes.LiquidVariable,
        expression: 0,
        filters: 1,
        rawSource: (tokens) =>
          source.slice(locStart(tokens), tokens[tokens.length - 2].source.endIdx).trimEnd(),
        locStart,
        // The last node of this rule is a positive lookahead, we don't
        // want its endIdx, we want the endIdx of the previous one.
        locEnd: locEndSecondToLast,
        source,
      },

      liquidFilter: {
        type: ConcreteNodeTypes.LiquidFilter,
        name: 3,
        locStart,
        locEnd,
        source,
        args(nodes) {
          // Traditinally, this would get transformed into null or array. But
          // it's better if we have an empty array instead of null here.
          if (nodes[7].sourceString === '') {
            return [];
          } else {
            return nodes[7].toAST((this).args.mapping);
          }
        },
      },
      filterArguments: 0,
      arguments: 0,
      simpleArgument: 0,
      tagArguments: 0,
      contentForTagArgument: 0,
      positionalArgument: 0,
      namedArgument: {
        type: ConcreteNodeTypes.NamedArgument,
        name: 0,
        value: 4,
        locStart,
        locEnd,
        source,
      },

      contentForNamedArgument: {
        type: ConcreteNodeTypes.NamedArgument,
        name: (node) => node[0].sourceString + node[1].sourceString,
        value: 6,
        locStart,
        locEnd,
        source,
      },

      liquidString: 0,
      liquidDoubleQuotedString: {
        type: ConcreteNodeTypes.String,
        single: () => false,
        value: 1,
        locStart,
        locEnd,
        source,
      },
      liquidSingleQuotedString: {
        type: ConcreteNodeTypes.String,
        single: () => true,
        value: 1,
        locStart,
        locEnd,
        source,
      },

      liquidNumber: {
        type: ConcreteNodeTypes.Number,
        value: 0,
        locStart,
        locEnd,
        source,
      },

      liquidLiteral: {
        type: ConcreteNodeTypes.LiquidLiteral,
        value: (tokens) => {
          const keyword = tokens[0].sourceString;
          return LiquidLiteralValues[keyword];
        },
        keyword: 0,
        locStart,
        locEnd,
        source,
      },

      liquidRange: {
        type: ConcreteNodeTypes.Range,
        start: 2,
        end: 6,
        locStart,
        locEnd,
        source,
      },

      liquidVariableLookup: {
        type: ConcreteNodeTypes.VariableLookup,
        name: 0,
        lookups: 1,
        locStart,
        locEnd,
        source,
      },
      variableSegmentAsLookupMarkup: 0,
      variableSegmentAsLookup: {
        type: ConcreteNodeTypes.VariableLookup,
        name: 0,
        lookups: () => [],
        locStart,
        locEnd,
        source,
      },

      lookup: 0,
      indexLookup: 3,
      dotLookup: {
        type: ConcreteNodeTypes.String,
        value: 3,
        locStart: (nodes) => offset + nodes[2].source.startIdx,
        locEnd: (nodes) => offset + nodes[nodes.length - 1].source.endIdx,
        source,
      }
    };

    const defaultMappings = {
      HelperMappings,
      LiquidMappings,
    };

    const selectedMappings = cstMappings.reduce(
      (mappings, key) => ({
        ...mappings,
        ...defaultMappings[key],
      }),
      {},
    );

    return ohmExtras.toAST(res, selectedMappings);
  }

  window.toLiquidHtmlAST = function (str) {
    const grammars = ohm.grammars(String.raw`
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

        singleQuote = "'" | "‘" | "’"
        doubleQuote = "\"" | "“" | "”"
        controls = "\u{007F}".."\u{009F}"
        noncharacters = "\u{FDD0}".."\u{FDEF}"
        newline = "\r"? "\n"
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
    `);
    const grammar = grammars.LiquidDoc;

    return toCST(str, grammars, grammar);
  };
})();
