use crate::bibliotecario::{ValidationOutcome, BibliotecarioFact};

#[derive(Debug, Clone, PartialEq)]
pub struct CameraSpec {
    pub width: usize,
    pub height: usize,
    pub field_of_view: f64,
}

impl CameraSpec {
    pub fn validate(&self) -> ValidationOutcome {
        if self.width == 0 || self.height == 0 {
            return ValidationOutcome::fail("camera dimensions must be greater than zero");
        }
        if !self.field_of_view.is_finite() || self.field_of_view <= 0.0 {
            return ValidationOutcome::fail("camera.field_of_view must be a positive finite number");
        }
        ValidationOutcome::ok()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LightSpec {
    pub position: [f64; 3],
    pub intensity: [f64; 3],
}

impl LightSpec {
    pub fn validate(&self) -> ValidationOutcome {
        if self.position.iter().all(|v| v.is_finite())
            && self.intensity.iter().all(|v| v.is_finite())
            && self.intensity.iter().all(|v| *v >= 0.0)
        {
            return ValidationOutcome::ok();
        }
        ValidationOutcome::fail("light values must be finite and intensity non-negative")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectSpec {
    pub kind: String,
    pub transform: [f64; 16],
    pub material: MaterialSpec,
}

impl ObjectSpec {
    pub fn validate(&self) -> ValidationOutcome {
        if self.kind.trim().is_empty() {
            return ValidationOutcome::fail("object.kind must not be empty");
        }
        self.material.validate()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MaterialSpec {
    pub color: [f64; 3],
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
}

impl MaterialSpec {
    pub fn validate(&self) -> ValidationOutcome {
        if !self.color.iter().all(|v| v.is_finite()) {
            return ValidationOutcome::fail("material.color values must be finite");
        }
        if !self.ambient.is_finite() || !self.diffuse.is_finite() || !self.specular.is_finite() {
            return ValidationOutcome::fail("material scalar values must be finite");
        }
        ValidationOutcome::ok()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SceneSpec {
    pub camera: CameraSpec,
    pub lights: Vec<LightSpec>,
    pub objects: Vec<ObjectSpec>,
    pub facts: Vec<BibliotecarioFact>,
}

impl SceneSpec {
    pub fn default_sample() -> Self {
        Self {
            camera: CameraSpec {
                width: 128,
                height: 128,
                field_of_view: std::f64::consts::FRAC_PI_2,
            },
            lights: vec![LightSpec {
                position: [0.0, 10.0, -10.0],
                intensity: [1.0, 1.0, 1.0],
            }],
            objects: vec![
                ObjectSpec {
                    kind: "sphere".into(),
                    transform: [1.0, 0.0, 0.0, 0.0,
                               0.0, 1.0, 0.0, 0.0,
                               0.0, 0.0, 1.0, 0.0,
                               0.0, 0.0, 0.0, 1.0],
                    material: MaterialSpec {
                        color: [0.8, 1.0, 0.6],
                        ambient: 0.1,
                        diffuse: 0.7,
                        specular: 0.2,
                    },
                },
                ObjectSpec {
                    kind: "sphere".into(),
                    transform: [0.5, 0.0, 0.0, 0.0,
                               0.0, 0.5, 0.0, 0.0,
                               0.0, 0.0, 0.5, 0.0,
                               0.0, 0.0, 0.0, 1.0],
                    material: MaterialSpec {
                        color: [0.7, 0.7, 1.0],
                        ambient: 0.1,
                        diffuse: 0.7,
                        specular: 0.2,
                    },
                },
            ],
            facts: vec![
                BibliotecarioFact::new("SCENE-1", "single light and two spheres", "scene-spec", 1),
            ],
        }
    }

    pub fn validate(&self) -> ValidationOutcome {
        let camera_result = self.camera.validate();
        if !camera_result.ok {
            return camera_result;
        }
        if self.lights.is_empty() {
            return ValidationOutcome::fail("scene.lights must contain at least one light");
        }
        if self.objects.is_empty() {
            return ValidationOutcome::fail("scene.objects must contain at least one object");
        }
        for light in &self.lights {
            let outcome = light.validate();
            if !outcome.ok {
                return outcome;
            }
        }
        for object in &self.objects {
            let outcome = object.validate();
            if !outcome.ok {
                return outcome;
            }
        }
        if self.facts.is_empty() {
            return ValidationOutcome::fail("scene.facts must not be empty");
        }
        for fact in &self.facts {
            let outcome = fact.validate();
            if !outcome.ok {
                return outcome;
            }
        }
        ValidationOutcome::ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scene_schema_accepts_valid_scene() {
        let scene = SceneSpec::default_sample();
        assert!(scene.validate().ok);
        assert_eq!(scene.camera.width, 128);
        assert_eq!(scene.objects.len(), 2);
    }

    #[test]
    fn scene_schema_rejects_invalid_fov_and_missing_light() {
        let mut scene = SceneSpec::default_sample();
        scene.camera.field_of_view = 0.0;
        assert!(!scene.validate().ok);

        let mut bad = SceneSpec::default_sample();
        bad.lights.clear();
        assert!(!bad.validate().ok);
    }
}
