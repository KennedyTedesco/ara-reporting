use ara_reporting::annotation::Annotation;
use ara_reporting::builder::Charset;
use ara_reporting::builder::ColorChoice;
use ara_reporting::builder::ReportBuilder;
use ara_reporting::error::Error;
use ara_reporting::issue::Issue;
use ara_reporting::Report;
use ara_source::source::Source;
use ara_source::source::SourceKind;
use ara_source::SourceMap;
fn main() -> Result<(), Error> {
    let origin = "example.ara";
    let code = r#"
$b = match $a {
    1 => 2,
    2 => 3,
    default => "string",
};
"#;

    let map = SourceMap::new(vec![Source::new(SourceKind::Script, origin, code)]);

    let report = Report::new().with_issue(
        Issue::error(
            "E0417",
            "`match` arms have incompatible types",
            origin,
            6,
            61,
        )
        .with_annotation(
            Annotation::new(origin, 26, 1).with_message("this is found to be of type `{int}`"),
        )
        .with_annotation(
            Annotation::new(origin, 38, 1).with_message("this is found to be of type `{int}`"),
        )
        .with_annotation(
            Annotation::new(origin, 56, 8).with_message("expected `{int}`, found `{string}`"),
        )
        .with_note("for more information about this error, try `ara --explain E0417`"),
    );

    let builder = ReportBuilder::new(&map, report)
        .with_colors(ColorChoice::Always)
        .with_charset(Charset::Unicode);

    builder.print()
}