//! Hikari, Twilight Guardian — `{3}{W}{W}` 4/4 Legendary Creature —
//! Spirit. White.
//! Flying.
//! "Whenever you cast a Spirit or Arcane spell, you may exile Hikari. If
//! you do, return it to the battlefield under its owner's control at the
//! beginning of the next end step."
//!
//! The trigger fires on casting a Spirit-or-Arcane spell you control;
//! the resolution exiles Hikari and schedules its return from exile at
//! the next end step. The "you may" is a resolution-time choice not
//! modeled as a gate — we emit the blink path (a documented
//! simplification consistent with the demonstrated API).

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hikari, Twilight Guardian");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    // "Spirit or Arcane" — match either subtype on the cast spell.
    let spirit_sub = reg.interner_mut().intern("Spirit");
    let arcane_sub = reg.interner_mut().intern("Arcane");
    let spell_filter = ObjectFilter::new()
        .with_subtypes_any(vec![spirit_sub, arcane_sub])
        .controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(spell_filter),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: blink_self_at_end_step,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn blink_self_at_end_step(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::ExilePermanent { target: trig.source },
        Effect::DelayedAction {
            source: trig.source,
            controller: trig.controller,
            when: DelayedWhen::NextEndStep,
            action: DelayedAction::ReturnFromExileToBattlefield,
        },
    ]
}
