use ara_reporting::annotation::Annotation;
use ara_reporting::builder::CharSet;
use ara_reporting::builder::ColorChoice;
use ara_reporting::builder::ReportBuilder;
use ara_reporting::error::Error;
use ara_reporting::issue::Issue;
use ara_reporting::Report;
use ara_reporting::ReportCollection;
use ara_reporting::ReportFooter;
use ara_source::source::Source;
use ara_source::source::SourceKind;
use ara_source::SourceMap;

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn main() -> Result<(), Error> {
    let project_root = format!("{MANIFEST_DIR}/examples/projects/project-a");
    let first_origin = "example.ara";
    let second_origin = "example-2.ara";

    let map = SourceMap::new(vec![
        Source::new(SourceKind::Script, &project_root, first_origin),
        Source::new(SourceKind::Script, &project_root, second_origin),
    ]);

    let first_report = Report::new()
        .with_issue(
            Issue::error("E0417", "`match` arms have incompatible types")
                .with_source(first_origin, 5, 67)
                .with_annotation(
                    Annotation::secondary(first_origin, 26, 27)
                        .with_message("this is found to be of type `{int}`"),
                )
                .with_annotation(
                    Annotation::secondary(first_origin, 38, 39)
                        .with_message("this is found to be of type `{int}`"),
                )
                .with_annotation(
                    Annotation::secondary(first_origin, 56, 64)
                        .with_message("expected `{int}`, found `{string}`"),
                )
                .with_note("for more information about this error, try `ara --explain E0417`"),
        )
        .with_footer(
            ReportFooter::new("this is a report footer message")
                .with_note("this is a note message"),
        );

    let second_report = Report::new()
        .with_issue(
            Issue::error(
                "P0015",
                "scalar type `float` cannot be used in an intersection",
            )
            .with_source(second_origin, 17, 23)
            .with_annotation(
                Annotation::secondary(second_origin, 17, 19)
                    .with_message("scalar type `float` cannot be used in an intersection"),
            )
            .with_note("a scalar type is either `int`, `float`, `string`, or `bool`.")
            .with_note("try using a different type for the intersection."),
        )
        .with_issue(Issue::bug("B0001", "failed to read the file"))
        .with_footer(
            ReportFooter::new("this is a report footer message")
                .with_note("this is a note message"),
        );

    let reports: ReportCollection = vec![&first_report, &second_report];

    let builder = ReportBuilder::new(&map)
        .with_colors(ColorChoice::Always)
        .with_charset(CharSet::Unicode);

    builder.print(&reports)
}
