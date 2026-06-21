//! Matsu-Tribe Decoy — `{2}{G}` 1/3 Snake Warrior.
//!
//! {2}{G}: Target creature blocks this creature this turn if able.
//! Whenever this creature deals combat damage to a creature, tap that
//!   creature and it doesn't untap during its controller's next untap
//!   step.
//!
//! Both abilities are wired structurally but their bodies are GAP'd:
//! • "target creature blocks this creature if able" (a lure / forced-
//!   block-a-specific-creature effect) has no primitive — the engine has
//!   Goad / ForbidAttacking / ForbidBlocking but no "must block X".
//! • the combat-damage trigger fires on `DamageDealt` to a creature, but
//!   there is no accessor for the damaged OBJECT (only `damaged_player`),
//!   so "tap that creature [+ stun]" can't reference its id.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Matsu-Tribe Decoy");
    let snake = reg.interner_mut().intern("Snake");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{G}: Target creature blocks this creature this turn if able.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: lure_block,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::default(),
                    target_filter: TargetFilter::Creature,
                    combat_only: true,
                },
                intervening_if: None,
                effect: tap_that_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn lure_block(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "target creature blocks this creature if able" — no
    // forced-block-a-specific-creature (lure) primitive.
    Vec::new()
}

fn tap_that_creature(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no accessor for the damaged object id (only damaged_player),
    // so "tap that creature and it doesn't untap next untap step" can't
    // reference the damaged creature.
    Vec::new()
}
