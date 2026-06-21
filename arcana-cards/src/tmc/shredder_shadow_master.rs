//! Shredder, Shadow Master — `{3}{B}{B}` 5/5 Legendary Human Ninja.
//!
//! Oracle:
//! * "Whenever Shredder attacks a player, for each other opponent, create a
//!   token that's a copy of Shredder tapped and attacking that player, except
//!   it isn't legendary. Sacrifice those tokens at end of combat." — token-copy
//!   of a SPECIFIC permanent (this creature) tapped-and-attacking a CHOSEN
//!   opponent, then sacrifice at end of combat. The engine's CopyPermanent
//!   mints a copy but there is no variant that (a) makes the copy enter
//!   tapped-and-attacking a particular player, (b) strips legendary, or
//!   (c) auto-sacrifices it at end of combat. GAP'd.
//! * "Whenever Shredder deals combat damage to a player, that player loses
//!   half their life, rounded up." — DamageDealt + damaged_player + LoseLife of
//!   half (rounded up). Fully expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shredder, Shadow Master");
    let human = reg.interner_mut().intern("Human");
    let ninja = reg.interner_mut().intern("Ninja");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(ninja);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_make_copies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: combat_damage_half_life,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attack_make_copies(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "for each other opponent, create a token that's a copy of Shredder
    // tapped and attacking that player, except it isn't legendary. Sacrifice
    // those tokens at end of combat." — no Effect variant mints a CopyPermanent
    // token that enters tapped-and-attacking a chosen opponent, strips
    // legendary, and is sacrificed at end of combat.
    Vec::new()
}

fn combat_damage_half_life(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(p) = trig.damaged_player() else { return Vec::new(); };
    let life = script::life(state, p).max(0);
    let half = ((life + 1) / 2) as u32;
    vec![Effect::LoseLife { player: p, amount: half }]
}
