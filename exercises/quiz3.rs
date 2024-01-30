// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.



use std::fmt::Display;

pub struct ReportCard {
    pub grade: f32,
    pub student_name: String,
    pub student_age: u8,
    pub grade_str: AlphabeticalGrade,
}

impl ReportCard {
    pub fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade
        )
    }
}

impl Display for ReportCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} ({}) - achieved a grade of {}",
            self.student_name, self.student_age, self.grade_str.grade_str
        ))
    }
}

pub struct AlphabeticalGrade {
    pub start: f32,
    pub end: f32,
    pub grade_str: String,
}

impl From<f32> for AlphabeticalGrade {
    fn from(value: f32) -> Self {
        let alphabetical_grade_list: Vec<AlphabeticalGrade> = vec![
            AlphabeticalGrade {
                start: 1_f32,
                end: 2.5_f32,
                grade_str: "A+".to_string(),
            },
            AlphabeticalGrade {
                start: 2.5_f32,
                end: 5_f32,
                grade_str: "A".to_string(),
            },
            // TODO other
        ];
        for vg in alphabetical_grade_list {
            if value > vg.start && value <= vg.end {
                return vg;
            }
        }
        panic!("请检查成绩有效性，成绩范围（1.0-5.0）");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            grade_str: 2.1.into(),
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: 2.1,
            grade_str: 2.1.into(),
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            format!("{}", report_card),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
