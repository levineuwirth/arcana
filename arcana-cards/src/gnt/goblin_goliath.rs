//! Goblin Goliath — `{4}{R}{R}` 5/4 Goblin Mutant.
//!
//! Oracle:
//! * When this creature enters, create a number of 1/1 red Goblin creature
//!   tokens equal to the number of opponents you have.
//! * {3}{R}, {T}: If a source you control would deal damage to an opponent this
//!   turn, it deals double that damage to that player instead. (GAP'd: a
//!   damage-doubling replacement effect — no primitive doubles dealt damage.)

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Goblin Goliath");
    let goblin = reg.interner_mut().intern("Goblin");
    let mutant = reg.interner_mut().intern("Mutant");
    // Token subtype interned for the resolver to look up.
    let _token_goblin = reg.interner_mut().intern("Goblin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(mutant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_make_goblins,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{R}, {T}: If a source you control would deal damage to an opponent this turn, it deals double that damage to that player instead.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{R}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: double_damage,
            }),
    )
}

fn etb_make_goblins(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let n = script::opponents(state, trig.controller).len() as u32;
    let goblin = reg.interner().lookup("Goblin").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    let mut out = Vec::new();
    for _ in 0..n {
        out.push(Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: goblin,
                colors: ColorSet::red(),
                types: TypeLine::CREATURE.into(),
                subtypes: subtypes.clone(),
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![],
                abilities: vec![],
            },
        });
    }
    out
}

fn double_damage(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "If a source you control would deal damage to an opponent this turn,
    // it deals double that damage to that player instead" — a damage-doubling
    // replacement effect; no primitive expresses doubling of dealt damage.
    Vec::new()
}
