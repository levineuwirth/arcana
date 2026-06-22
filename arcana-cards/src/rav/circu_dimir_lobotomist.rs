//! Circu, Dimir Lobotomist — `{2}{U}{B}` 2/3 Legendary Human Wizard (B/U).
//! Whenever you cast a blue spell, exile the top card of target player's library.
//! Whenever you cast a black spell, exile the top card of target player's library.
//! Your opponents can't cast spells with the same name as a card exiled with Circu.
//!
//! Both spell-cast triggers are wired (filtered by spell color, controller You,
//! targeting a player), but the payload "exile the top card of target player's
//! library" has no expressible Effect (no exile-top-of-library variant; Mill
//! goes to graveyard, not exile) — GAP'd. The opponents-can't-cast static is
//! also GAP'd (no name-lock primitive).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Circu, Dimir Lobotomist");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::new().with_colors(ColorSet::blue())),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: exile_top_of_target,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_player()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::new().with_colors(ColorSet::black())),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: exile_top_of_target,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_player()],
            }),
    )
}

fn exile_top_of_target(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile the top card of target player's library" — no
    //      exile-top-of-library Effect exists (Mill goes to graveyard).
    Vec::new()
}
