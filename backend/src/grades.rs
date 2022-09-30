use std::fmt::Display;

struct Grade {
    value: f32,
    weight: f32,
}

struct Scale {
    score: f32,
    max_score: f32,
    weight: f32,
    tresholds: Vec<(f32, f32)>,
}

impl Grade {
    fn new(value: f32, weight: f32) -> Self {
        Self { value, weight }
    }
}
impl Scale {
    fn new(score: f32, max_score: f32, weight: f32, tresholds: Vec<(f32, f32)>) -> Self {
        Self {
            score,
            max_score,
            weight,
            tresholds,
        }
    }
}

trait Score {
    fn get_raw(&self) -> f32;
    fn get_weighted(&self) -> f32;
}

impl Score for Grade {
    fn get_raw(&self) -> f32 {
        self.value
    }
    fn get_weighted(&self) -> f32 {
        self.get_raw() * self.weight
    }
}

impl Score for Scale {
    fn get_raw(&self) -> f32 {
        self.score / self.max_score
    }
    fn get_weighted(&self) -> f32 {
        self.get_raw() * self.weight
    }
}

impl Display for Grade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Grade {} of weight {}", self.value, self.weight)
    }
}

impl Display for Scale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Scored {} / {}", self.score, self.max_score)
    }
}

impl Into<Grade> for Scale {
    fn into(self) -> Grade {
        let frac = self.get_raw();
        let mut maximum = 0f32;
        self.tresholds.iter().for_each(|x| {
            if frac >= x.0 {
                maximum = maximum.max(x.1)
            }
        });
        Grade::new(maximum, self.weight)
    }
}

#[test]
fn scale_to_grade() {
    let tresholds = vec![
        (0.0, 1.0),
        (0.4, 2.0),
        (0.5, 3.0),
        (0.75, 4.0),
        (0.9, 5.0),
        (0.98, 6.0),
    ];

    fn helper(score: f32, max_score: f32, expected_grade: f32, tresholds: &Vec<(f32, f32)>){
        let scale = Scale::new(score, max_score, 3.0, tresholds.clone());
        let grade: Grade = scale.into();
        assert_eq!(grade.get_raw(), expected_grade)
    }
    
    helper(11.5,20.0,3.0,&tresholds);
    helper(10.0,20.0,3.0,&tresholds);
    helper(9.9,20.0,2.0,&tresholds);
    helper(50.0,20.0,6.0,&tresholds);
    helper(0.0,20.0,1.0,&tresholds);
}
