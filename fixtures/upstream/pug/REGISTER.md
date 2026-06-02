Upstream fixture register
========================
total fixtures: 459

By bucket
- packages/pug-filters/test/cases: 1
- packages/pug-lexer/test/cases: 118
- packages/pug-lexer/test/errors: 26
- packages/pug-linker/test/cases-src: 40
- packages/pug-linker/test/errors-src: 3
- packages/pug-linker/test/fixtures: 17
- packages/pug-linker/test/special-cases-src: 3
- packages/pug/examples: 19
- packages/pug/test/anti-cases: 22
- packages/pug/test/browser: 1
- packages/pug/test/cases: 137
- packages/pug/test/cases-es2015: 1
- packages/pug/test/dependencies: 7
- packages/pug/test/duplicate-block: 2
- packages/pug/test/eachOf/error: 4
- packages/pug/test/eachOf/passing: 2
- packages/pug/test/extends-not-top-level: 3
- packages/pug/test/fixtures: 39
- packages/pug/test/markdown-it: 2
- packages/pug/test/regression-2436: 6
- packages/pug/test/shadowed-block: 3
- packages/pug/test/temp: 3

By role
- example: 19
- case: 308
- anti-case: 55
- support: 77

Format outcomes by role
- example / idempotent: 3
- example / rewritten: 16
- case / idempotent: 58
- case / rewritten: 250
- anti-case / idempotent: 12
- anti-case / rewritten: 43
- support / idempotent: 13
- support / rewritten: 64

Structure coverage by role
- example / fully-structured: 8
- example / mixed: 11
- case / no-statements: 4
- case / fully-structured: 174
- case / mixed: 126
- case / raw-only: 4
- anti-case / fully-structured: 34
- anti-case / mixed: 9
- anti-case / raw-only: 12
- support / no-statements: 1
- support / fully-structured: 61
- support / mixed: 7
- support / raw-only: 8

Rewritten anti-cases
- packages/pug/test/anti-cases/attrs.unescaped.pug [packages/pug/test/anti-cases]
- packages/pug/test/anti-cases/case-when.pug [packages/pug/test/anti-cases]
- packages/pug/test/anti-cases/case-without-with.pug [packages/pug/test/anti-cases]
- packages/pug/test/anti-cases/else-condition.pug [packages/pug/test/anti-cases]
- packages/pug/test/anti-cases/else-without-if.pug [packages/pug/test/anti-cases]
- packages/pug/test/anti-cases/inlining-a-mixin-after-a-tag.pug [packages/pug/test/anti-cases]
- packages/pug/test/anti-cases/mismatched-inline-tag.pug [packages/pug/test/anti-cases]
- packages/pug/test/anti-cases/mixins-blocks-with-bodies.pug [packages/pug/test/anti-cases]
- packages/pug/test/anti-cases/multiple-non-nested-tags-on-a-line.pug [packages/pug/test/anti-cases]
- packages/pug/test/anti-cases/non-existant-filter.pug [packages/pug/test/anti-cases]
- packages/pug/test/anti-cases/non-mixin-block.pug [packages/pug/test/anti-cases]
- packages/pug/test/anti-cases/open-brace-in-attributes.pug [packages/pug/test/anti-cases]
- packages/pug/test/anti-cases/self-closing-tag-with-body.pug [packages/pug/test/anti-cases]
- packages/pug/test/anti-cases/tabs-and-spaces.pug [packages/pug/test/anti-cases]
- packages/pug/test/anti-cases/unclosed-interpolated-call.pug [packages/pug/test/anti-cases]
- packages/pug/test/anti-cases/unclosed-interpolated-tag.pug [packages/pug/test/anti-cases]
- packages/pug/test/anti-cases/unclosed-interpolation.pug [packages/pug/test/anti-cases]
- packages/pug-lexer/test/errors/attribute-invalid-expression.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/case-with-no-expression.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/default-with-expression.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/else-with-condition.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/extends-no-path.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/include-filter-no-path-2.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/include-filter-no-path.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/include-filter-no-space.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/include-no-path.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/inconsistent-indentation.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/interpolated-call.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/invalid-class-name-1.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/invalid-class-name-2.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/invalid-class-name-3.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/invalid-id.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/malformed-each.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/malformed-extend.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/malformed-include.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/mismatched-inline-tag.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/multi-line-interpolation.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/open-interpolation.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/when-with-no-expression.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/while-with-no-expression.pug [packages/pug-lexer/test/errors]
- packages/pug-linker/test/errors-src/child-with-tags.pug [packages/pug-linker/test/errors-src]
- packages/pug-linker/test/errors-src/extends-not-first.pug [packages/pug-linker/test/errors-src]
- packages/pug-linker/test/errors-src/unexpected-block.pug [packages/pug-linker/test/errors-src]

Most opaque case fixtures
- raw 29/47 | mixed | packages/pug-lexer/test/cases/mixin.attrs.pug
- raw 29/47 | mixed | packages/pug/test/cases/mixin.attrs.pug
- raw 12/26 | mixed | packages/pug-lexer/test/cases/code.iteration.pug
- raw 12/14 | mixed | packages/pug-lexer/test/cases/mixin.merge.pug
- raw 12/26 | mixed | packages/pug/test/cases/code.iteration.pug
- raw 12/14 | mixed | packages/pug/test/cases/mixin.merge.pug
- raw 11/39 | mixed | packages/pug-lexer/test/cases/each.else.pug
- raw 11/25 | mixed | packages/pug-lexer/test/cases/mixins.pug
- raw 11/25 | mixed | packages/pug/test/cases/mixins.pug
- raw 9/10 | mixed | packages/pug-lexer/test/cases/html.pug
- raw 9/18 | mixed | packages/pug-lexer/test/cases/tag.interpolation.pug
- raw 9/16 | mixed | packages/pug-lexer/test/cases/tags.self-closing.pug
- raw 9/10 | mixed | packages/pug/test/cases/html.pug
- raw 9/18 | mixed | packages/pug/test/cases/tag.interpolation.pug
- raw 9/16 | mixed | packages/pug/test/cases/tags.self-closing.pug
- raw 8/16 | mixed | packages/pug-lexer/test/cases/styles.pug
- raw 8/32 | mixed | packages/pug/test/cases/each.else.pug
- raw 8/16 | mixed | packages/pug/test/cases/styles.pug
- raw 7/33 | mixed | packages/pug-lexer/test/cases/mixin.blocks.pug
- raw 7/33 | mixed | packages/pug/test/cases/mixin.blocks.pug
