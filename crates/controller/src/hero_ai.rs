use std::collections::HashMap;

use domain::{Ability, AbilityEffect, AbilityId, AbilityTarget, Hero, HeroId, Position};
use world::{Side, World};

const INVALID_ACTION_SCORE: i32 = -1_000_000;

const POSITION_ORDER: [Position; Position::COUNT] =
    [Position::Frontline, Position::Midline, Position::Backline];

const ABILITY_ORDER: [AbilityId; 6] = [
    AbilityId::Fireball,
    AbilityId::ArcaneExplosion,
    AbilityId::Slam,
    AbilityId::Whirlwind,
    AbilityId::MainAttack,
    AbilityId::OffhandAttack,
];

pub struct DecisionContext<'a> {
    pub actor: &'a Hero,
    pub world: &'a World,
    pub side: Side,
    pub targets: HashMap<HeroId, &'a Hero>,
}

impl<'a> DecisionContext<'a> {
    pub fn actor_position(&self) -> Option<Position> {
        self.world.position_of(self.side, &self.actor.id)
    }

    pub fn allies(&self) -> Vec<(HeroId, &'a Hero)> {
        self.heroes_on(self.side)
    }

    pub fn enemies(&self) -> Vec<(HeroId, &'a Hero)> {
        self.heroes_on(Side::other_side(self.side))
    }

    pub fn hero(&self, hero_id: &HeroId) -> Option<&'a Hero> {
        self.targets.get(hero_id).copied()
    }

    fn heroes_on(&self, side: Side) -> Vec<(HeroId, &'a Hero)> {
        POSITION_ORDER
            .iter()
            .filter_map(|&position| {
                self.world
                    .hero_at(side, position)
                    .and_then(|hero_id| self.hero(hero_id).map(|hero| (hero_id.clone(), hero)))
            })
            .collect()
    }

    fn is_on_side(&self, side: Side, hero_id: &HeroId) -> bool {
        self.world.position_of(side, hero_id).is_some()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TurnAction {
    Ability {
        ability_id: AbilityId,
        target: TargetSelection,
    },
    Move {
        to: Position,
    },
    Swap {
        with: HeroId,
    },
    Bandage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetSelection {
    SelfTarget,
    Single(HeroId),
    Area(Vec<HeroId>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScoredAction {
    pub action: TurnAction,
    pub score: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScoreWeights {
    pub damage_point: i32,
    pub heal_point: i32,
    pub kill_bonus: i32,
    pub critical_health_bonus: i32,
    pub move_cost: i32,
    pub swap_cost: i32,
    pub bandage_heal: u8,
    pub future_position_percent: i32,
}

impl Default for ScoreWeights {
    fn default() -> Self {
        Self {
            damage_point: 10,
            heal_point: 8,
            kill_bonus: 90,
            critical_health_bonus: 50,
            move_cost: 18,
            swap_cost: 24,
            bandage_heal: 8,
            future_position_percent: 45,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ScoringAi {
    pub weights: ScoreWeights,
}

impl ScoringAi {
    pub fn new(weights: ScoreWeights) -> Self {
        Self { weights }
    }

    pub fn best_action(&self, ctx: &DecisionContext) -> ScoredAction {
        let mut actions = self.scored_actions(ctx).into_iter();
        let mut best = actions
            .next()
            .expect("bandage fallback action should always exist");

        for scored_action in actions {
            if scored_action.score > best.score {
                best = scored_action;
            }
        }

        best
    }

    pub fn scored_actions(&self, ctx: &DecisionContext) -> Vec<ScoredAction> {
        let mut actions = vec![ScoredAction {
            action: TurnAction::Bandage,
            score: self.score_bandage(ctx),
        }];

        if !ctx.actor.is_alive() || ctx.actor_position().is_none() {
            return actions;
        }

        self.push_ability_actions(ctx, &mut actions);
        self.push_move_actions(ctx, &mut actions);
        self.push_swap_actions(ctx, &mut actions);

        actions
    }

    pub fn score_action(&self, ctx: &DecisionContext, action: &TurnAction) -> i32 {
        match action {
            TurnAction::Ability { ability_id, target } => {
                let Some(actor_position) = ctx.actor_position() else {
                    return INVALID_ACTION_SCORE;
                };

                let abilities = ctx.actor.class.abilities();
                let Some(ability) = abilities.get(ability_id) else {
                    return INVALID_ACTION_SCORE;
                };

                if !ability.positions_from.contains(&actor_position) {
                    return INVALID_ACTION_SCORE;
                }

                let target_ids = self.target_ids_for_action(ctx, ability, target);

                if target_ids.is_empty() {
                    return INVALID_ACTION_SCORE;
                }

                self.score_effect_on_targets(ctx, ability, &target_ids)
            }
            TurnAction::Move { to } => self.score_move(ctx, *to),
            TurnAction::Swap { with } => self.score_swap(ctx, with),
            TurnAction::Bandage => self.score_bandage(ctx),
        }
    }

    fn push_ability_actions(&self, ctx: &DecisionContext, actions: &mut Vec<ScoredAction>) {
        let Some(actor_position) = ctx.actor_position() else {
            return;
        };

        let abilities = ctx.actor.class.abilities();

        for ability_id in ABILITY_ORDER {
            let Some(ability) = abilities.get(&ability_id) else {
                continue;
            };

            if !ability.positions_from.contains(&actor_position) {
                continue;
            }

            for target in self.targets_for_ability(ctx, ability, ctx.side) {
                let action = TurnAction::Ability { ability_id, target };
                let score = self.score_action(ctx, &action);

                if score > INVALID_ACTION_SCORE {
                    actions.push(ScoredAction { action, score });
                }
            }
        }
    }

    fn push_move_actions(&self, ctx: &DecisionContext, actions: &mut Vec<ScoredAction>) {
        let Some(current_position) = ctx.actor_position() else {
            return;
        };

        for position in POSITION_ORDER {
            if !is_adjacent_position(current_position, position)
                || ctx.world.hero_at(ctx.side, position).is_some()
            {
                continue;
            }

            let action = TurnAction::Move { to: position };
            let score = self.score_action(ctx, &action);

            if score > INVALID_ACTION_SCORE {
                actions.push(ScoredAction { action, score });
            }
        }
    }

    fn push_swap_actions(&self, ctx: &DecisionContext, actions: &mut Vec<ScoredAction>) {
        for (ally_id, _) in ctx.allies() {
            if ally_id == ctx.actor.id {
                continue;
            }

            let action = TurnAction::Swap { with: ally_id };
            let score = self.score_action(ctx, &action);

            if score > INVALID_ACTION_SCORE {
                actions.push(ScoredAction { action, score });
            }
        }
    }

    fn targets_for_ability(
        &self,
        ctx: &DecisionContext,
        ability: &Ability,
        caster_side: Side,
    ) -> Vec<TargetSelection> {
        match &ability.target_type {
            AbilityTarget::SelfTarget => vec![TargetSelection::SelfTarget],
            AbilityTarget::Enemy => ctx
                .heroes_on(Side::other_side(caster_side))
                .into_iter()
                .filter(|(hero_id, hero)| {
                    hero.is_alive()
                        && self.target_position_matches(
                            ctx,
                            Side::other_side(caster_side),
                            hero_id,
                            ability,
                        )
                })
                .map(|(hero_id, _)| TargetSelection::Single(hero_id))
                .collect(),
            AbilityTarget::Ally => ctx
                .heroes_on(caster_side)
                .into_iter()
                .filter(|(hero_id, hero)| {
                    hero.is_alive()
                        && self.target_position_matches(ctx, caster_side, hero_id, ability)
                })
                .map(|(hero_id, _)| TargetSelection::Single(hero_id))
                .collect(),
            AbilityTarget::AreaOfEffect => {
                let target_ids = ctx
                    .heroes_on(Side::other_side(caster_side))
                    .into_iter()
                    .filter(|(hero_id, hero)| {
                        hero.is_alive()
                            && self.target_position_matches(
                                ctx,
                                Side::other_side(caster_side),
                                hero_id,
                                ability,
                            )
                    })
                    .map(|(hero_id, _)| hero_id)
                    .collect::<Vec<_>>();

                if target_ids.is_empty() {
                    Vec::new()
                } else {
                    vec![TargetSelection::Area(target_ids)]
                }
            }
        }
    }

    fn target_ids_for_action(
        &self,
        ctx: &DecisionContext,
        ability: &Ability,
        target: &TargetSelection,
    ) -> Vec<HeroId> {
        match (&ability.target_type, target) {
            (AbilityTarget::SelfTarget, TargetSelection::SelfTarget) => vec![ctx.actor.id.clone()],
            (AbilityTarget::Enemy, TargetSelection::Single(hero_id)) => {
                let enemy_side = Side::other_side(ctx.side);

                if ctx.is_on_side(enemy_side, hero_id)
                    && self.target_position_matches(ctx, enemy_side, hero_id, ability)
                {
                    vec![hero_id.clone()]
                } else {
                    Vec::new()
                }
            }
            (AbilityTarget::Ally, TargetSelection::Single(hero_id)) => {
                if ctx.is_on_side(ctx.side, hero_id)
                    && self.target_position_matches(ctx, ctx.side, hero_id, ability)
                {
                    vec![hero_id.clone()]
                } else {
                    Vec::new()
                }
            }
            (AbilityTarget::AreaOfEffect, TargetSelection::Area(hero_ids)) => {
                let enemy_side = Side::other_side(ctx.side);

                hero_ids
                    .iter()
                    .cloned()
                    .filter(|hero_id| {
                        ctx.is_on_side(enemy_side, hero_id)
                            && self.target_position_matches(ctx, enemy_side, hero_id, ability)
                    })
                    .collect()
            }
            _ => Vec::new(),
        }
    }

    fn target_position_matches(
        &self,
        ctx: &DecisionContext,
        side: Side,
        hero_id: &HeroId,
        ability: &Ability,
    ) -> bool {
        ctx.world
            .position_of(side, hero_id)
            .is_some_and(|position| ability.positions_to.contains(&position))
    }

    fn score_effect_on_targets(
        &self,
        ctx: &DecisionContext,
        ability: &Ability,
        target_ids: &[HeroId],
    ) -> i32 {
        match &ability.effect_type {
            AbilityEffect::Damage(damage) => target_ids
                .iter()
                .filter_map(|hero_id| ctx.hero(hero_id))
                .map(|target| self.score_damage(target, *damage))
                .sum(),
            AbilityEffect::Heal(heal) => target_ids
                .iter()
                .filter_map(|hero_id| ctx.hero(hero_id))
                .map(|target| self.score_heal(target, *heal))
                .sum(),
        }
    }

    fn score_damage(&self, target: &Hero, damage: u8) -> i32 {
        if !target.is_alive() {
            return 0;
        }

        let current_health = target.health.current as i32;
        let damage = damage as i32;
        let effective_damage = damage.min(current_health);
        let mut score = effective_damage * self.weights.damage_point;

        if damage >= current_health {
            score += self.weights.kill_bonus + self.threat_score(target);
        }

        score
    }

    fn score_heal(&self, target: &Hero, heal: u8) -> i32 {
        if !target.is_alive() {
            return 0;
        }

        let missing_health = missing_health(target);
        let heal = heal as i32;
        let effective_heal = heal.min(missing_health);
        let mut score = effective_heal * self.weights.heal_point;

        if is_critical_health(target) && effective_heal > 0 {
            score += self.weights.critical_health_bonus;
        }

        score
    }

    fn score_move(&self, ctx: &DecisionContext, to: Position) -> i32 {
        let Some(current_position) = ctx.actor_position() else {
            return INVALID_ACTION_SCORE;
        };

        if !is_adjacent_position(current_position, to) || ctx.world.hero_at(ctx.side, to).is_some()
        {
            return INVALID_ACTION_SCORE;
        }

        self.position_value(ctx, &ctx.actor.id, ctx.actor, ctx.side, to)
            - self.position_value(ctx, &ctx.actor.id, ctx.actor, ctx.side, current_position)
            - self.weights.move_cost
    }

    fn score_swap(&self, ctx: &DecisionContext, ally_id: &HeroId) -> i32 {
        let Some(actor_position) = ctx.actor_position() else {
            return INVALID_ACTION_SCORE;
        };
        let Some(ally) = ctx.hero(ally_id) else {
            return INVALID_ACTION_SCORE;
        };
        let Some(ally_position) = ctx.world.position_of(ctx.side, ally_id) else {
            return INVALID_ACTION_SCORE;
        };

        if ally_id == &ctx.actor.id || !ally.is_alive() {
            return INVALID_ACTION_SCORE;
        }

        let actor_gain =
            self.position_value(ctx, &ctx.actor.id, ctx.actor, ctx.side, ally_position)
                - self.position_value(ctx, &ctx.actor.id, ctx.actor, ctx.side, actor_position);
        let ally_gain = self.position_value(ctx, ally_id, ally, ctx.side, actor_position)
            - self.position_value(ctx, ally_id, ally, ctx.side, ally_position);

        actor_gain + ally_gain - self.weights.swap_cost
    }

    fn score_bandage(&self, ctx: &DecisionContext) -> i32 {
        if !ctx.actor.is_alive() || missing_health(ctx.actor) == 0 {
            return 0;
        }

        self.score_heal(ctx.actor, self.weights.bandage_heal)
    }

    fn position_value(
        &self,
        ctx: &DecisionContext,
        hero_id: &HeroId,
        hero: &Hero,
        side: Side,
        position: Position,
    ) -> i32 {
        let best_ability_score = self.best_future_ability_score(ctx, hero_id, hero, side, position);

        best_ability_score * self.weights.future_position_percent / 100
    }

    fn best_future_ability_score(
        &self,
        ctx: &DecisionContext,
        hero_id: &HeroId,
        hero: &Hero,
        side: Side,
        position: Position,
    ) -> i32 {
        let abilities = hero.class.abilities();
        let mut best_score = 0;

        for ability_id in ABILITY_ORDER {
            let Some(ability) = abilities.get(&ability_id) else {
                continue;
            };

            if !ability.positions_from.contains(&position) {
                continue;
            }

            let score = self.best_future_ability_target_score(ctx, ability, hero_id, side);

            if score > best_score {
                best_score = score;
            }
        }

        best_score
    }

    fn best_future_ability_target_score(
        &self,
        ctx: &DecisionContext,
        ability: &Ability,
        caster_id: &HeroId,
        caster_side: Side,
    ) -> i32 {
        let mut best_score = 0;

        for target in self.targets_for_ability(ctx, ability, caster_side) {
            let target_ids =
                self.target_ids_for_future(ctx, ability, &target, caster_id, caster_side);

            if target_ids.is_empty() {
                continue;
            }

            let score = self.score_effect_on_targets(ctx, ability, &target_ids);

            if score > best_score {
                best_score = score;
            }
        }

        best_score
    }

    fn target_ids_for_future(
        &self,
        ctx: &DecisionContext,
        ability: &Ability,
        target: &TargetSelection,
        caster_id: &HeroId,
        caster_side: Side,
    ) -> Vec<HeroId> {
        match (&ability.target_type, target) {
            (AbilityTarget::SelfTarget, TargetSelection::SelfTarget) => vec![caster_id.clone()],
            (AbilityTarget::Enemy, TargetSelection::Single(hero_id)) => {
                let enemy_side = Side::other_side(caster_side);

                if ctx.is_on_side(enemy_side, hero_id)
                    && self.target_position_matches(ctx, enemy_side, hero_id, ability)
                {
                    vec![hero_id.clone()]
                } else {
                    Vec::new()
                }
            }
            (AbilityTarget::Ally, TargetSelection::Single(hero_id)) => {
                if ctx.is_on_side(caster_side, hero_id)
                    && self.target_position_matches(ctx, caster_side, hero_id, ability)
                {
                    vec![hero_id.clone()]
                } else {
                    Vec::new()
                }
            }
            (AbilityTarget::AreaOfEffect, TargetSelection::Area(hero_ids)) => {
                let enemy_side = Side::other_side(caster_side);

                hero_ids
                    .iter()
                    .cloned()
                    .filter(|hero_id| {
                        ctx.is_on_side(enemy_side, hero_id)
                            && self.target_position_matches(ctx, enemy_side, hero_id, ability)
                    })
                    .collect()
            }
            _ => Vec::new(),
        }
    }

    fn threat_score(&self, hero: &Hero) -> i32 {
        let max_damage = hero
            .class
            .abilities()
            .values()
            .filter_map(|ability| match &ability.effect_type {
                AbilityEffect::Damage(damage) => Some(*damage as i32),
                _ => None,
            })
            .max()
            .unwrap_or(0);

        max_damage * 3 + hero.initiative as i32
    }
}

impl HeroAi for ScoringAi {
    fn supports(&self, _hero: &Hero) -> bool {
        true
    }

    fn decide_turn(&self, ctx: &DecisionContext) -> TurnAction {
        self.best_action(ctx).action
    }
}

pub trait HeroAi {
    fn supports(&self, hero: &Hero) -> bool;

    fn decide_turn(&self, ctx: &DecisionContext) -> TurnAction;
}

fn missing_health(hero: &Hero) -> i32 {
    hero.health.max.saturating_sub(hero.health.current) as i32
}

fn is_critical_health(hero: &Hero) -> bool {
    hero.health.current > 0 && hero.health.current as u16 * 4 <= hero.health.max as u16
}

fn is_adjacent_position(from: Position, to: Position) -> bool {
    matches!(
        (from, to),
        (Position::Frontline, Position::Midline)
            | (Position::Midline, Position::Frontline)
            | (Position::Midline, Position::Backline)
            | (Position::Backline, Position::Midline)
    )
}

#[cfg(test)]
mod tests {
    use world::Lineup;

    use super::*;

    fn empty_world() -> World {
        let left = [
            HeroId::new("LeftEmptyFront"),
            HeroId::new("LeftEmptyMid"),
            HeroId::new("LeftEmptyBack"),
        ];
        let right = [
            HeroId::new("RightEmptyFront"),
            HeroId::new("RightEmptyMid"),
            HeroId::new("RightEmptyBack"),
        ];
        let mut world = World::new(Lineup::from(left.clone()), Lineup::from(right.clone()));

        for hero_id in left {
            world
                .remove(Side::Left, &hero_id)
                .expect("placeholder removal should succeed");
        }
        for hero_id in right {
            world
                .remove(Side::Right, &hero_id)
                .expect("placeholder removal should succeed");
        }

        world
    }

    fn context<'a>(
        actor: &'a Hero,
        world: &'a World,
        side: Side,
        heroes: Vec<&'a Hero>,
    ) -> DecisionContext<'a> {
        DecisionContext {
            actor,
            world,
            side,
            targets: heroes
                .into_iter()
                .map(|hero| (hero.id.clone(), hero))
                .collect(),
        }
    }

    #[test]
    fn chooses_lethal_target_over_healthier_enemy() {
        let actor = Hero::mage();
        let mut weak_enemy = Hero::warrior();
        let healthy_enemy = Hero::warrior();
        let mut world = empty_world();

        weak_enemy.take_damage(95);
        world
            .place(Side::Left, &actor.id, Position::Backline)
            .expect("actor placement should succeed");
        world
            .place(Side::Right, &healthy_enemy.id, Position::Frontline)
            .expect("healthy enemy placement should succeed");
        world
            .place(Side::Right, &weak_enemy.id, Position::Backline)
            .expect("weak enemy placement should succeed");

        let ctx = context(
            &actor,
            &world,
            Side::Left,
            vec![&actor, &weak_enemy, &healthy_enemy],
        );

        let action = ScoringAi::default().decide_turn(&ctx);

        match action {
            TurnAction::Ability {
                target: TargetSelection::Single(target_id),
                ..
            } => assert_eq!(target_id, weak_enemy.id),
            other => panic!("expected lethal ability target, got {other:?}"),
        }
    }

    #[test]
    fn chooses_bandage_for_critical_actor_when_no_enemy_is_available() {
        let mut actor = Hero::warrior();
        let mut world = empty_world();

        actor.take_damage(80);
        world
            .place(Side::Left, &actor.id, Position::Frontline)
            .expect("actor placement should succeed");

        let ctx = context(&actor, &world, Side::Left, vec![&actor]);

        let action = ScoringAi::default().decide_turn(&ctx);

        assert_eq!(action, TurnAction::Bandage);
    }

    #[test]
    fn chooses_move_when_a_better_position_unlocks_stronger_actions() {
        let actor = Hero::mage();
        let enemy = Hero::warrior();
        let mut world = empty_world();

        world
            .place(Side::Left, &actor.id, Position::Frontline)
            .expect("actor placement should succeed");
        world
            .place(Side::Right, &enemy.id, Position::Frontline)
            .expect("enemy placement should succeed");

        let ctx = context(&actor, &world, Side::Left, vec![&actor, &enemy]);

        let action = ScoringAi::default().decide_turn(&ctx);

        assert_eq!(
            action,
            TurnAction::Move {
                to: Position::Midline,
            }
        );
    }
}
