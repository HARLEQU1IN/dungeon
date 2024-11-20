use crate::direction::Direction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Enemy {
    pub(crate) name: String,
    pub(crate) health: u32,
    pub(crate) direction: Direction,
}

impl Enemy {
    pub fn new(name: String, health: u32, direction: Direction) -> Self {
        Enemy {
            name,
            health,
            direction,
        }
    }

    // Метод для получения урона
    pub fn take_damage(&mut self, damage: u32) {
        if damage >= self.health {
            self.health = 0;
            println!("{} был повержен!", self.name);
        } else {
            self.health -= damage;
            println!(
                "{} получил {} урона. Осталось здоровья: {}",
                self.name, damage, self.health
            );
        }
    }

    // Метод для проверки состояния здоровья
    pub fn is_alive(&self) -> bool { self.health > 0 }

    // Метод для отображения здоровья врага
    #[expect(dead_code)]
    pub fn render_health(&self) -> String {
        format!("{}: Здоровье врага: {}", self.name, self.health)
    }
}
