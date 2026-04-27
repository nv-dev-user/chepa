use super::living_entity::LivingEntity;

pub struct Player {
    living_entity: LivingEntity,
    coef_exp: f32,
}

impl Player {
    pub fn new(
        living_entity: LivingEntity,
        coef_exp: f32
    ) -> Self {
        Player {
            living_entity,
            coef_exp
        }
    }

    pub fn get_living_entity(&self) -> &LivingEntity {
        &self.living_entity
    }

    pub fn get_coef_exp(&self) -> f32 {
        self.coef_exp
    }
}